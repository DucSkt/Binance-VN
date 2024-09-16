use crate::api::state::AppState;
use crate::api::user::controller::{create_user, list_user, list_users};
use axum::{
    routing::{get, post},
    Router,
};

pub struct UserRouter(Router<AppState>);

impl UserRouter {
    pub fn new(state: AppState) -> Self {
        let router = Router::new()
            .route("/", post(create_user))
            .route("/", get(list_users))
            .route("/:id", get(list_user))
            .with_state(state);
        Self(router)
    }
}

impl From<UserRouter> for Router<AppState> {
    fn from(router: UserRouter) -> Self {
        router.0
    }
}
