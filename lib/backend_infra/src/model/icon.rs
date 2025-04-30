use domain::model::icon::Icon;
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct IconRow {
    pub id: Uuid,
    pub content_type: String,
    pub icon: Vec<u8>,
}

impl From<IconRow> for Icon {
    fn from(val: IconRow) -> Self {
        Icon {
            id: val.id,
            content_type: val.content_type,
            icon: val.icon,
        }
    }
}
