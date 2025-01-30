use axum::{
    routing::{get, post, put},
    Router,
};

use super::Repository;

mod authentication;
mod submissions;
mod users;

pub fn make_router(app_state: Repository) -> Router {
    let authentication_router = Router::new()
        .route("/signup/request", post(authentication::sign_up_request))
        .route("/signup", post(authentication::sign_up))
        .route("/login", post(authentication::login))
        .route("/logout", post(authentication::logout))
        .route(
            "/reset-password/request",
            post(authentication::reset_password_request),
        )
        .route("/reset-password", post(authentication::reset_password));

    let users_router = Router::new()
        .route("/me", get(users::get_me).put(users::put_me))
        .route("/me/email", put(users::put_me_email))
        .route("/me/password", put(users::put_me_password))
        .route("/:userId", get(users::get_user));

    let submissions_router =
        Router::new().route("/:submissionId", get(submissions::get_submission));

    Router::new()
        .nest("/", authentication_router)
        .nest("/users", users_router)
        .nest("/submissions", submissions_router)
        .with_state(app_state)
}
