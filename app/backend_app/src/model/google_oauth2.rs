use serde::{Deserialize, Serialize};
use usecase::model::google_oauth2::GoogleOAuth2AuthorizeDto;

#[derive(Serialize, Deserialize)]
pub struct GoogleOAuth2ParamsResponse {
    pub url: String,
}

impl From<String> for GoogleOAuth2ParamsResponse {
    fn from(url: String) -> Self {
        Self { url }
    }
}

#[derive(Serialize, Deserialize)]
pub struct GoogleOAuth2AuthorizeRequest {
    pub code: String,
}

#[derive(Serialize, Deserialize)]
pub struct GoogleOAuth2AuthorizeResponse {
    pub token: String,
}

impl From<GoogleOAuth2AuthorizeDto> for GoogleOAuth2AuthorizeResponse {
    fn from(value: GoogleOAuth2AuthorizeDto) -> Self {
        Self {
            token: value.token.unwrap(),
        }
    }
}
