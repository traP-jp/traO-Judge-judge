use anyhow::Result;

pub trait TextResourceRepository<KeyType> {
    fn get_text(&self, key: &KeyType) -> Result<String>;
}
