use anyhow::Result;
use uuid::Uuid;

pub trait TextResourceRepository {
    async fn get_text(&self, key: &Uuid) -> Result<String>;
}
