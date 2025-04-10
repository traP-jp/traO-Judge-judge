use crate::di::DiContainer;
use axum::{
    routing::{get, post, put},
    Router,
};

pub mod auth;
pub mod editorial;
pub mod problems;
pub mod submissions;
pub mod users;

pub fn make_router(di_container: DiContainer) -> Router {
    let auth_router = Router::new()
        .route("/signup/request", post(auth::signup_request))
        .route("/signup", post(auth::signup))
        .route("/login", post(auth::login))
        .route("/logout", post(auth::logout))
        .route(
            "/reset-password/request",
            post(auth::reset_password_request),
        )
        .route("/reset-password", post(auth::reset_password));

    let user_router = Router::new()
        .route("/me", get(users::get_me).put(users::put_me))
        .route("/me/email", put(users::put_me_email))
        .route("/me/password", put(users::put_me_password))
        .route("/:userId", get(users::get_user));

    let submission_router = Router::new()
        .route("/", get(submissions::get_submissions))
        .route("/:submissionId", get(submissions::get_submission));

    let problem_router = Router::new()
        .route(
            "/",
            post(problems::post_problem).get(problems::get_problems),
        )
        .route(
            "/:problemId",
            get(problems::get_problem)
                .put(problems::put_problem)
                .delete(problems::delete_problem),
        )
        .route(
            "/:problemId/editorials",
            get(editorial::get_editorials).post(editorial::post_editorial),
        );

    let editorials_router = Router::new().route(
        "/:editorialId",
        get(editorial::get_editorial)
            .put(editorial::put_editorial)
            .delete(editorial::delete_editorial),
    );

    Router::new()
        .nest("/", auth_router)
        .nest("/users", user_router)
        .nest("/submissions", submission_router)
        .nest("/problems", problem_router)
        .nest("/editorials", editorials_router)
        .with_state(di_container)
}
