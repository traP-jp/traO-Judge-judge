use std::collections::HashMap;

use axum::{body::Body, extract::*, response::Response, routing::*};
use axum_extra::extract::{CookieJar, Multipart};
use bytes::Bytes;
use http::{header::CONTENT_TYPE, HeaderMap, HeaderName, HeaderValue, Method, StatusCode};
use tracing::error;
use validator::{Validate, ValidationErrors};

use crate::{header, types::*};

#[allow(unused_imports)]
use crate::{apis, models};

/// Setup API Server.
pub fn new<I, A>(api_impl: I) -> Router
where
    I: AsRef<A> + Clone + Send + Sync + 'static,
    A: apis::judge::Judge + 'static,
{
    // build our application with a route
    Router::new()
        .route("/v1/judge", post(judge::<I, A>))
        .with_state(api_impl)
}

#[derive(validator::Validate)]
#[allow(dead_code)]
struct JudgeBodyValidator<'a> {
    #[validate(nested)]
    body: &'a models::Judge,
}

#[tracing::instrument(skip_all)]
fn judge_validation(
    body: models::Judge,
) -> std::result::Result<(models::Judge,), ValidationErrors> {
    let b = JudgeBodyValidator { body: &body };
    b.validate()?;

    Ok((body,))
}
/// Judge - POST /v1/judge
#[tracing::instrument(skip_all)]
async fn judge<I, A>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    State(api_impl): State<I>,
    Json(body): Json<models::Judge>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::judge::Judge,
{
    #[allow(clippy::redundant_closure)]
    let validation = tokio::task::spawn_blocking(move || judge_validation(body))
        .await
        .unwrap();

    let Ok((body,)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl.as_ref().judge(method, host, cookies, body).await;

    let mut response = Response::builder();

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::judge::JudgeResponse::Status200_OK => {
                let mut response = response.status(200);
                response.body(Body::empty())
            }
        },
        Err(_) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            response.status(500).body(Body::empty())
        }
    };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}
