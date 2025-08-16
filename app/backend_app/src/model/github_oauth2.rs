use serde::{Deserialize, Serialize};
use usecase::model::github_oauth2::GitHubOAuth2AuthorizeDto;

#[derive(Serialize, Deserialize)]
pub struct GitHubOAuth2ParamsResponse {
    pub url: String,
}

impl From<String> for GitHubOAuth2ParamsResponse {
    fn from(url: String) -> Self {
        Self { url }
    }
}

#[derive(Serialize, Deserialize)]
pub struct GitHubOAuth2AuthorizeRequest {
    pub code: String,
}

#[derive(Serialize, Deserialize)]
pub struct GitHubOAuth2AuthorizeResponse {
    pub token: String,
}

impl From<GitHubOAuth2AuthorizeDto> for GitHubOAuth2AuthorizeResponse {
    fn from(value: GitHubOAuth2AuthorizeDto) -> Self {
        Self {
            token: value.token.unwrap(),
        }
    }
}
