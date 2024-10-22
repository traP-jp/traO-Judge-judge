use async_trait::async_trait;
use axum::extract::*;
use axum_extra::extract::{CookieJar, Multipart};
use bytes::Bytes;
use http::Method;
use serde::{Deserialize, Serialize};

use crate::{models, types::*};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum JudgeResponse {
    /// OK
    Status200_OK,
}

/// Judge
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Judge {
    /// 全てのジャッジ.
    ///
    /// Judge - POST /v1/judge
    async fn judge(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        body: models::Judge,
    ) -> Result<JudgeResponse, String>;
}
