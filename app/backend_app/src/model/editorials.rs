use serde::{Deserialize, Serialize};
use sqlx::types::chrono;
use usecase::model::editorial::{
    CreateEditorialData, EditorialDto, EditorialSummaryDto, UpdateEditorialData,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EditorialResponse {
    pub id: i64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub problem_id: i64,
    pub author_id: i64,
    pub statement: String,
    pub is_public: bool,
}

impl From<EditorialDto> for EditorialResponse {
    fn from(value: EditorialDto) -> Self {
        EditorialResponse {
            id: value.id,
            created_at: value.created_at,
            updated_at: value.updated_at,
            problem_id: value.problem_id,
            author_id: value.author_id,
            statement: value.statement,
            is_public: value.is_public,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EditorialSummaryResponse {
    pub id: i64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub problem_id: i64,
    pub author_id: i64,
    pub is_public: bool,
}

impl From<EditorialSummaryDto> for EditorialSummaryResponse {
    fn from(value: EditorialSummaryDto) -> Self {
        EditorialSummaryResponse {
            id: value.id,
            created_at: value.created_at,
            updated_at: value.updated_at,
            problem_id: value.problem_id,
            author_id: value.author_id,
            is_public: value.is_public,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateEditorial {
    pub statement: String,
    pub is_public: bool,
}

impl From<UpdateEditorial> for UpdateEditorialData {
    fn from(value: UpdateEditorial) -> Self {
        UpdateEditorialData {
            statement: value.statement,
            is_public: value.is_public,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateEditorial {
    pub statement: String,
    pub is_public: bool,
}

impl From<CreateEditorial> for CreateEditorialData {
    fn from(value: CreateEditorial) -> Self {
        CreateEditorialData {
            statement: value.statement,
            is_public: value.is_public,
        }
    }
}
