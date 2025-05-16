use super::uuid::UuidRow;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct DepNameRow {
    pub dep_id: UuidRow,
    pub name: String,
}
