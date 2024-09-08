use crate::api::state::AppState;
use crate::api::user::controller::create_user_handler;
use axum::{routing::post, Router};

pub struct UserRouter(Router<AppState>);

impl UserRouter {
    pub fn new(state: AppState) -> Self {
        let router = Router::new()
            .route("/", post(create_user_handler))
            // Add more routes as needed
            .with_state(state);

        Self(router)
    }
}

impl From<UserRouter> for Router<AppState> {
    fn from(router: UserRouter) -> Self {
        router.0
    }
}
