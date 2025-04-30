use domain::model::{
    rules::RuleType,
    user::{User, UserRole},
};
use sqlx::types::chrono;
use uuid::Uuid;

use super::{problem::NormalProblemsDto, submission::SubmissionsDto};

pub struct UpdateUserData {
    pub user_name: Option<String>,
    pub icon: Option<String>,
    pub x_link: Option<String>,
    pub github_link: Option<String>,
    pub self_introduction: Option<String>,
}

impl UpdateUserData {
    pub fn validate(&self) -> anyhow::Result<()> {
        let rules = vec![
            (&self.user_name, RuleType::UserName),
            (&self.icon, RuleType::Icon),
            (&self.x_link, RuleType::XLink),
            (&self.github_link, RuleType::GitHubLink),
            (&self.self_introduction, RuleType::SelfIntroduction),
        ];
        for (value, rule) in rules {
            if let Some(value) = value {
                rule.validate(value)?;
            }
        }
        Ok(())
    }
}

pub struct UpdatePasswordData {
    pub old_password: String,
    pub new_password: String,
}

impl UpdatePasswordData {
    pub fn validate(&self) -> anyhow::Result<()> {
        RuleType::Password.validate(&self.old_password)?;
        RuleType::Password.validate(&self.new_password)?;
        Ok(())
    }
}

pub enum UserRoleDto {
    Admin,
    CommonUser,
    TrapUser,
}

impl From<UserRole> for UserRoleDto {
    fn from(role: UserRole) -> Self {
        match role {
            UserRole::Admin => UserRoleDto::Admin,
            UserRole::CommonUser => UserRoleDto::CommonUser,
            UserRole::TrapUser => UserRoleDto::TrapUser,
        }
    }
}

pub struct UserDto {
    pub id: Uuid,
    pub display_id: i64,
    pub name: String,
    pub traq_id: Option<String>,
    pub github_id: Option<String>,
    pub icon_url: Option<String>,
    pub post_problems: NormalProblemsDto,
    pub submit_problems: SubmissionsDto,
    pub x_link: Option<String>,
    pub github_link: Option<String>,
    pub self_introduction: String,
    pub role: UserRoleDto,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl UserDto {
    pub fn new(user: User, problems: NormalProblemsDto, submissions: SubmissionsDto) -> Self {
        UserDto {
            id: user.id.0,
            display_id: user.display_id,
            name: user.name,
            traq_id: user.traq_id,
            github_id: user.github_id,
            icon_url: user.icon_url,
            post_problems: problems,
            submit_problems: submissions,
            x_link: user.x_link,
            github_link: user.github_link,
            self_introduction: user.self_introduction,
            role: user.role.into(),
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}
