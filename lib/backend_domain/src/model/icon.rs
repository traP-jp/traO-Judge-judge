use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Icon {
    pub id: IconId,
    pub content_type: String,
    pub icon: Vec<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IconId(Uuid);

impl std::fmt::Display for IconId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Into<Uuid> for IconId {
    fn into(self) -> Uuid {
        self.0
    }
}

impl From<Uuid> for IconId {
    fn from(id: Uuid) -> Self {
        Self(id)
    }
}

#[derive(Debug, Clone)]
pub struct CreateIcon {
    pub content_type: String,
    pub icon: Vec<u8>,
}
