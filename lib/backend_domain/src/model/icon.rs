use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Icon {
    pub id: Uuid,
    pub content_type: String,
    pub icon: Vec<u8>,
}
