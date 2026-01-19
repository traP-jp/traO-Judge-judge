use sqlx::{Decode, Encode, MySql, Type, prelude::FromRow};
use uuid::Uuid;

#[derive(Debug, FromRow, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UuidRow(pub Uuid);

impl UuidRow {
    pub fn new(id: Uuid) -> Self {
        Self(id)
    }
}

impl<'a> Decode<'a, MySql> for UuidRow {
    fn decode(
        value: <MySql as sqlx::database::HasValueRef<'a>>::ValueRef,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        <Uuid as Decode<'a, MySql>>::decode(value).map(UuidRow)
    }
}

impl<'a> Encode<'a, MySql> for UuidRow {
    fn encode_by_ref(
        &self,
        buf: &mut <MySql as sqlx::database::HasArguments<'a>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        self.0.encode_by_ref(buf)
    }

    fn encode(
        self,
        buf: &mut <MySql as sqlx::database::HasArguments<'a>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        self.0.encode(buf)
    }
}

impl Type<MySql> for UuidRow {
    fn type_info() -> sqlx::mysql::MySqlTypeInfo {
        <Uuid as Type<MySql>>::type_info()
    }
    fn compatible(ty: &<MySql as sqlx::Database>::TypeInfo) -> bool {
        <Uuid as Type<MySql>>::compatible(ty)
    }
}
