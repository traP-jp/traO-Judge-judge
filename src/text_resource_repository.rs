use anyhow::Result;

pub trait TextResourceRepository<KeyType> {
    async fn get_text(&self, key: &KeyType) -> Result<String>;
}
