use crate::di::DiContainer;
use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
};
use axum_extra::extract::cookie::CookieJar;
use domain::{model::session::SessionUser, repository::session::SessionRepository};

pub struct ExtractedSessionUser(pub Option<SessionUser>);

#[async_trait]
impl FromRequestParts<DiContainer> for ExtractedSessionUser {
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(
        parts: &mut Parts,
        di_container: &DiContainer,
    ) -> Result<Self, Self::Rejection> {
        let jar = CookieJar::from_request_parts(parts, di_container)
            .await
            .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid Cookie header"))?;

        if let Some(session_id) = jar.get("session_id") {
            let session_user = di_container
                .session_repository()
                .get_session_user(session_id.value())
                .await
                .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Session retrieval error"))?;
            Ok(ExtractedSessionUser(session_user))
        } else {
            Ok(ExtractedSessionUser(None))
        }
    }
}
