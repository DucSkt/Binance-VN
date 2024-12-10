use crate::api::state::AppState;
use crate::api::user::controller::{create_user, delete_user, list_user, list_users, update_user};
use axum::{
    routing::{delete, get, post, put},
    Router,
};

pub struct UserRouter(Router<AppState>);

impl UserRouter {
    pub fn new(state: AppState) -> Self {
        let router = Router::new()
            .route("/", post(create_user))
            .route("/", get(list_users))
            .route("/:id", get(list_user))
            .route("/:id", put(update_user))
            .route("/:id", delete(delete_user))
            .with_state(state);
        Self(router)
    }
}

impl From<UserRouter> for Router<AppState> {
    fn from(router: UserRouter) -> Self {
        router.0
    }
}
