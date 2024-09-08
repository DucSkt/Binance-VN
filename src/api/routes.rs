use crate::api::state::AppState;
use crate::api::user::routes::UserRouter;
use axum::Router;

pub struct ApiRouter(Router<AppState>);

impl ApiRouter {
    pub fn new(state: AppState) -> Self {
        let routes: Router<AppState> = Router::new().nest("/users", UserRouter::new(state).into());

        Self(routes)
    }
}

impl From<ApiRouter> for Router<AppState> {
    fn from(router: ApiRouter) -> Self {
        router.0
    }
}
