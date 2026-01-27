use axum::{body::Body, extract::State, http::Request, middleware::Next, response::Response};
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use sqlx::SqlitePool;

use crate::app::AppState;
use crate::error::{ApiError, ApiResult};

use wallet_domain::UserId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Admin,
    User,
}

#[derive(Debug, Clone)]
pub struct AuthCtx {
    pub role: Role,
    pub user_id: Option<UserId>,
}

fn dev_no_auth_enabled() -> bool {
    matches!(std::env::var("WALLET_DEV_NO_AUTH"), Ok(v) if v == "1")
}

fn dev_user_from_header(req: &Request<Body>) -> Option<UserId> {
    let raw = req.headers().get("X-Dev-User")?.to_str().ok()?;
    UserId::parse(raw.trim()).ok()
}

fn dev_user_from_query(req: &Request<Body>) -> Option<UserId> {
    let query = req.uri().query()?;
    for pair in query.split('&') {
        let mut iter = pair.splitn(2, '=');
        let key = iter.next().unwrap_or("");
        if key != "as" {
            continue;
        }
        let value = iter.next().unwrap_or("");
        let value = value.replace('+', " ");
        let decoded = percent_decode(&value);
        if let Ok(user) = UserId::parse(decoded.as_str()) {
            return Some(user);
        }
    }
    None
}

fn percent_decode(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    let mut chars = value.as_bytes().iter().copied();
    while let Some(b) = chars.next() {
        if b == b'%' {
            let hi = chars.next();
            let lo = chars.next();
            if let (Some(hi), Some(lo)) = (hi, lo)
                && let (Some(hi), Some(lo)) = (hex_value(hi), hex_value(lo))
            {
                out.push((hi << 4 | lo) as char);
                continue;
            }
            out.push('%');
            if let Some(hi) = hi {
                out.push(hi as char);
            }
            if let Some(lo) = lo {
                out.push(lo as char);
            }
        } else {
            out.push(b as char);
        }
    }
    out
}

fn hex_value(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}

async fn dev_user_from_currency_account(pool: &SqlitePool, account_id: i64) -> Option<UserId> {
    let owner = sqlx::query_scalar::<_, String>(
        "SELECT a.owner_id\n         FROM currency_accounts ca\n         JOIN accounts a ON a.id = ca.root_account_id\n         WHERE ca.id = ?1 AND a.owner_type = 'user'\n         LIMIT 1",
    )
    .bind(account_id)
    .fetch_optional(pool)
    .await
    .ok()
    .flatten()?;

    UserId::parse(&owner).ok()
}

async fn dev_auth_ctx(st: &AppState, path: &str, selected_user: Option<UserId>) -> AuthCtx {
    if path.starts_with("/v1/admin") || path.starts_with("/v1/dev") {
        return AuthCtx {
            role: Role::Admin,
            user_id: None,
        };
    }

    if let Some(user_id) = selected_user {
        return AuthCtx {
            role: Role::User,
            user_id: Some(user_id),
        };
    }

    if let Some(rest) = path.strip_prefix("/v1/wallet/") {
        let user_id = rest.split('/').next().unwrap_or("");
        if let Ok(user_id) = UserId::parse(user_id) {
            return AuthCtx {
                role: Role::User,
                user_id: Some(user_id),
            };
        }
    }

    if let Some(rest) = path.strip_prefix("/v1/accounts/") {
        let account_id_raw = rest.split('/').next().unwrap_or("");
        let account_id_raw = account_id_raw.trim_start_matches("acc_");
        if let Ok(account_id) = account_id_raw.parse::<i64>()
            && let Some(user_id) = dev_user_from_currency_account(&st.pool, account_id).await
        {
            return AuthCtx {
                role: Role::User,
                user_id: Some(user_id),
            };
        }
    }

    let fallback = UserId::parse("u01").expect("dev fallback user id");
    AuthCtx {
        role: Role::User,
        user_id: Some(fallback),
    }
}

/// Simple MVP auth:
/// - `Authorization: Bearer <ADMIN_TOKEN>` -> Admin
/// - `Authorization: Bearer user:<user_id>:<sig>` -> User
///   where `sig = base64url(hmac_sha256(USER_TOKEN_SECRET, "user:<user_id>"))`
///
/// Notes:
/// - This is not intended as production-grade auth. It's a minimal RBAC gate for the MVP.
/// - We keep it deterministic and small on purpose.
pub async fn auth_middleware(
    State(st): State<AppState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, ApiError> {
    // Public endpoints.
    let path = req.uri().path();
    if path == "/health"
        || path == "/api-doc/openapi.json"
        || path.starts_with("/swagger-ui")
        || path == "/v1/currencies"
    {
        return Ok(next.run(req).await);
    }

    if dev_no_auth_enabled() {
        let selected_user = dev_user_from_header(&req).or_else(|| dev_user_from_query(&req));
        let ctx = dev_auth_ctx(&st, path, selected_user).await;
        req.extensions_mut().insert(ctx);
        return Ok(next.run(req).await);
    }

    let token =
        extract_bearer(&req).ok_or(ApiError::Unauthorized("missing Authorization header"))?;
    let ctx = parse_token(&token)?;

    req.extensions_mut().insert(ctx);
    Ok(next.run(req).await)
}

fn extract_bearer(req: &Request<Body>) -> Option<String> {
    let h = req.headers().get("Authorization")?.to_str().ok()?;
    let h = h.trim();
    let prefix = "Bearer ";
    if !h.starts_with(prefix) {
        return None;
    }
    Some(h[prefix.len()..].trim().to_string())
}

fn admin_token() -> String {
    std::env::var("ADMIN_TOKEN").unwrap_or_else(|_| "admin-dev-token".to_string())
}

fn user_secret() -> String {
    std::env::var("USER_TOKEN_SECRET").unwrap_or_else(|_| "dev-secret".to_string())
}

fn parse_token(token: &str) -> ApiResult<AuthCtx> {
    if token == admin_token() {
        return Ok(AuthCtx {
            role: Role::Admin,
            user_id: None,
        });
    }

    // user:<user_id>:<sig>
    let mut parts = token.splitn(3, ':');
    let kind = parts.next().unwrap_or("");
    if kind != "user" {
        return Err(ApiError::Unauthorized("invalid token"));
    }

    let user_id_str = parts
        .next()
        .ok_or(ApiError::Unauthorized("invalid token"))?;
    let _sig = parts
        .next()
        .ok_or(ApiError::Unauthorized("invalid token"))?;

    let user = UserId::parse(user_id_str).map_err(|_| ApiError::Unauthorized("invalid token"))?;

    let expected = sign_user_token(&user);
    if token != expected {
        return Err(ApiError::Unauthorized("invalid token"));
    }

    Ok(AuthCtx {
        role: Role::User,
        user_id: Some(user),
    })
}

pub fn sign_user_token_str(user_id: &str) -> String {
    let user = UserId::parse(user_id).expect("valid user id for token");
    sign_user_token(&user)
}

pub fn sign_user_token(user_id: &UserId) -> String {
    type H = Hmac<Sha256>;

    let msg = format!("user:{}", user_id.as_str());
    let secret = user_secret();

    let mut mac = H::new_from_slice(secret.as_bytes()).expect("HMAC can take key of any size");
    mac.update(msg.as_bytes());
    let raw = mac.finalize().into_bytes();

    let sig = URL_SAFE_NO_PAD.encode(raw);
    format!("user:{}:{}", user_id.as_str(), sig)
}

pub fn require_admin(ctx: &AuthCtx) -> ApiResult<()> {
    match ctx.role {
        Role::Admin => Ok(()),
        Role::User => Err(ApiError::Forbidden("admin only")),
    }
}

pub fn require_user(ctx: &AuthCtx) -> ApiResult<UserId> {
    match ctx.role {
        Role::Admin => Err(ApiError::Forbidden("user only")),
        Role::User => ctx
            .user_id
            .clone()
            .ok_or(ApiError::Unauthorized("missing user")),
    }
}
