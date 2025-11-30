use async_session::chrono;
use serde::{Deserialize, Serialize};
use usecase::model::user::{UserDto, UserMeDto, UserRoleDto};

use super::{problems::ProblemSummariesResponses, submissions::SubmissionSummariesResponse};

#[derive(Serialize, Deserialize)]
pub enum UserRoleResponse {
    #[serde(rename = "CommonUser")]
    CommonUser,
    #[serde(rename = "traPUser")]
    TrapUser,
    #[serde(rename = "Admin")]
    Admin,
}

impl From<UserRoleDto> for UserRoleResponse {
    fn from(role: UserRoleDto) -> Self {
        match role {
            UserRoleDto::CommonUser => UserRoleResponse::CommonUser,
            UserRoleDto::TrapUser => UserRoleResponse::TrapUser,
            UserRoleDto::Admin => UserRoleResponse::Admin,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
    pub id: String,
    pub name: String,
    pub traq_id: Option<String>,
    pub github_id: Option<String>,
    pub icon_url: Option<String>,
    pub post_problems: ProblemSummariesResponses,
    pub submit_problems: SubmissionSummariesResponse,
    pub x_id: Option<String>,
    pub self_introduction: String,
    pub role: UserRoleResponse,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<UserDto> for UserResponse {
    fn from(user: UserDto) -> Self {
        UserResponse {
            id: user.display_id.to_string(),
            name: user.name,
            traq_id: user.traq_id,
            github_id: user.github_id,
            icon_url: user.icon_id.map(|icon_id| {
                format!(
                    "{}/{}",
                    std::env::var("ICON_HOST_URI").unwrap_or_default(),
                    icon_id
                )
            }),
            post_problems: user.post_problems.into(),
            submit_problems: user.submit_problems.into(),
            x_id: user.x_id,
            self_introduction: user.self_introduction,
            role: user.role.into(),
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserMeResponse {
    pub id: String,
    pub name: String,
    pub traq_id: Option<String>,
    pub github_id: Option<String>,
    pub icon_url: Option<String>,
    pub post_problems: ProblemSummariesResponses,
    pub submit_problems: SubmissionSummariesResponse,
    pub x_id: Option<String>,
    pub self_introduction: String,
    pub role: UserRoleResponse,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub authentication: UserAuthentication,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserAuthentication {
    pub email: Option<String>,
    pub github_auth: Option<String>,
    pub google_auth: Option<String>,
    pub traq_auth: Option<String>,
}

impl From<UserMeDto> for UserMeResponse {
    fn from(user: UserMeDto) -> Self {
        UserMeResponse {
            id: user.display_id.to_string(),
            name: user.name,
            traq_id: user.traq_id,
            github_id: user.github_id,
            icon_url: user.icon_id.map(|icon_id| {
                format!(
                    "{}/{}",
                    std::env::var("ICON_HOST_URI").unwrap_or_default(),
                    icon_id
                )
            }),
            post_problems: user.post_problems.into(),
            submit_problems: user.submit_problems.into(),
            x_id: user.x_id,
            self_introduction: user.self_introduction,
            role: user.role.into(),
            created_at: user.created_at,
            updated_at: user.updated_at,
            authentication: UserAuthentication {
                email: user.authentication.email,
                github_auth: user.authentication.github_oauth,
                google_auth: user.authentication.google_oauth,
                traq_auth: user.authentication.traq_oauth,
            },
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct UpdateEmail {
    pub email: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePassword {
    pub old_password: String,
    pub new_password: String,
}

#[derive(serde::Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateMe {
    pub user_name: String,
    pub icon: Option<String>,
    pub github_id: String,
    pub x_id: String,
    pub self_introduction: String,
}
