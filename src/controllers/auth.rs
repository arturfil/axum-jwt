use std::sync::Arc;

use axum::{Router, routing::{get, post}, middleware};

use crate::{handlers::handlers::{register_user_handler, login_user_handler, logout_handler, get_me_handler}, AppState, middleware::jwt_auth::auth};


pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/auth/register", post(register_user_handler))
        .route("/api/auth/login", post(login_user_handler))
        .route("/api/auth/logout",
            get(logout_handler)
                .route_layer(middleware::from_fn_with_state(app_state.clone(), auth))
        )
        .route("/api/users/me", get(get_me_handler)
               .route_layer(middleware::from_fn_with_state(app_state.clone(), auth)),
       )
        .with_state(app_state)
}
