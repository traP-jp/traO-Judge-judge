use uuid::Uuid;


#[derive(Debug, Clone, sqlx::FromRow)]
pub struct DepNameRow {
    pub dep_id: Uuid, // UuidRowに変更する
    pub name: String,
}