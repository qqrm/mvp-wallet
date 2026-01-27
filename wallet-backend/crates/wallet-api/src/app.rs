use axum::{Router, middleware};
use sqlx::SqlitePool;
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{api, auth, openapi::ApiDoc};

#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
}

pub fn build_app(pool: SqlitePool) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any) // для MVP проще так; при желании зафиксируй http://localhost:5173
        .allow_methods(Any)
        .allow_headers(Any);

    let state = AppState { pool };

    Router::new()
        .merge(api::routes())
        .merge(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi()))
        .with_state(state)
        // MVP RBAC gate: Authorization: Bearer ...
        .layer(middleware::from_fn(auth::auth_middleware))
        .layer(cors)
}
