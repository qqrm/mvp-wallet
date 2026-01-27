use axum::{body::Body, http::Request, middleware::Next, response::Response};
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use hmac::{Hmac, Mac};
use sha2::Sha256;

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

/// Simple MVP auth:
/// - `Authorization: Bearer <ADMIN_TOKEN>` -> Admin
/// - `Authorization: Bearer user:<user_id>:<sig>` -> User
///   where `sig = base64url(hmac_sha256(USER_TOKEN_SECRET, "user:<user_id>"))`
///
/// Notes:
/// - This is not intended as production-grade auth. It's a minimal RBAC gate for the MVP.
/// - We keep it deterministic and small on purpose.
pub async fn auth_middleware(mut req: Request<Body>, next: Next) -> Result<Response, ApiError> {
    // Public endpoints.
    let path = req.uri().path();
    if path == "/health"
        || path == "/api-doc/openapi.json"
        || path.starts_with("/swagger-ui")
        || path == "/v1/currencies"
    {
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
