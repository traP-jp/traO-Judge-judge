pub trait TextResourceRepository<KeyType> {
    fn get_text(&self, key: KeyType) -> String;
}
