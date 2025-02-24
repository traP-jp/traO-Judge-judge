use std::borrow::BorrowMut;

use super::common::RequestBuilderExt;
use axum::{
    body::Body,
    http::{self, Request},
};
use backend_app::handler::make_router;
use di::DiContainer;
use http_body_util::BodyExt;
use infra::provider::Provider;
use serde_json::{json, Value};
use tower::ServiceExt;
use domain::repository::session::SessionRepository;
use domain::repository::user::UserRepository;

#[ignore]
#[sqlx::test(fixtures("common"), migrations = "../../lib/backend_infra/migrations")]
async fn put_user_me_backend(pool: sqlx::MySqlPool) -> anyhow::Result<()> {
    let provider = Provider::create_by_pool(pool).await?;
    let di_container = DiContainer::new(provider.clone()).await;
    let mut app = make_router(di_container);

    let tests = vec![
        (
            1,
            json!({
                "userName": "tt",
                "selfIntroduction": "hello",
            }),
            vec![("name", "tt"), ("selfIntroduction", "hello")],
        ),
        (
            2,
            json!({
                "userName": "t-t",
                "xLink": "https://x.com/test",
            }),
            vec![("name", "t-t"), ("xLink", "https://x.com/test")],
        ),
        (
            3,
            json!({
                "userName": "t-t",
                "xLink": "https://x.com/tester/t",
                "selfIntroduction": "hello",
            }),
            vec![
                ("name", "t-t"),
                ("xLink", "https://x.com/tester/t"),
                ("selfIntroduction", "hello"),
            ],
        ),
    ];

    for (id, req_json, changes) in tests {
        let session_id = provider
            .provide_session_repository()
            .create_session(
                provider
                    .provide_user_repository()
                    .get_user_by_display_id(id)
                    .await?
                    .unwrap(),
            )
            .await?;

        let response = app
            .borrow_mut()
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri("/users/me")
                    .header("Cookie", format!("session_id={}", session_id))
                    .json(req_json),
            )
            .await?;

        assert_eq!(response.status(), 200);

        let resp_json: Value =
            serde_json::from_slice(&response.into_body().collect().await?.to_bytes())?;

        for (key, value) in changes {
            assert_eq!(resp_json[key], value);
        }

        // get_users/me との比較
        let get_user_response = app
            .borrow_mut()
            .oneshot(
                Request::builder()
                    .method(http::Method::GET)
                    .uri("/users/me")
                    .header("Cookie", format!("session_id={}", session_id))
                    .body(Body::empty())?,
            )
            .await?;

        assert_eq!(get_user_response.status(), 200);
        let resp_json2: Value =
            serde_json::from_slice(&get_user_response.into_body().collect().await?.to_bytes())?;

        assert_eq!(resp_json, resp_json2);
    }
    Ok(())
}

#[ignore]
#[sqlx::test(fixtures("common"), migrations = "../../lib/backend_infra/migrations")]
async fn put_user_me_invalid_backend(pool: sqlx::MySqlPool) -> anyhow::Result<()> {
    let provider = Provider::create_by_pool(pool).await?;
    let di_container = DiContainer::new(provider.clone()).await;
    let mut app = make_router(di_container);

    let response = app
        .borrow_mut()
        .oneshot(
            Request::builder()
                .method(http::Method::PUT)
                .uri("/users/me")
                .json(json!({
                    "userName": "test",
                    "xLink": "https://x.com/tester/t",
                    "selfIntroduction": "hello",
                })),
        )
        .await?;

    assert_eq!(response.status(), 401);

    let session_id = provider
        .provide_session_repository()
        .create_session(
            provider
                .provide_user_repository()
                .get_user_by_display_id(1)
                .await?
                .unwrap(),
        )
        .await?;

    let tests = vec![
        json!({
            "userName": "t-",
            "xLink": "https://x.com/tester/t",
            "selfIntroduction": "hello",
        }),
        json!({
            "userName": "t-t",
            "xLink": "https://x.com",
        }),
        json!({
            "userName": "Test/Test",
            "selfIntroduction": "hello",
        }),
    ];

    for req_json in tests {
        let response = app
            .borrow_mut()
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri("/users/me")
                    .header("Cookie", format!("session_id={}", session_id))
                    .json(req_json),
            )
            .await?;

        assert_eq!(response.status(), 400);
    }

    Ok(())
}
