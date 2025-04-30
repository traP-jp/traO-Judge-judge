use domain::model::icon::Icon;
use super::uuid::UuidRow;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct IconRow {
    pub id: UuidRow,
    pub content_type: String,
    pub icon: Vec<u8>,
}

impl From<IconRow> for Icon {
    fn from(val: IconRow) -> Self {
        Icon {
            id: val.id.0,
            content_type: val.content_type,
            icon: val.icon,
        }
    }
}
