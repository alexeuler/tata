use crate::error::Error;
use derive_more::Display;
use diesel::backend::Backend;
use diesel::deserialize::{self, FromSql};
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::Text;
use diesel::sqlite::Sqlite;
use std::io::prelude::*;

#[derive(Debug, Display, PartialEq, Eq, FromSqlRow, AsExpression, Clone)]
#[sql_type = "Text"]
pub struct PeerId(String);

impl Default for PeerId {
    fn default() -> Self {
        PeerId("".to_string())
    }
}

impl ToSql<Text, Sqlite> for PeerId {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Sqlite>) -> serialize::Result {
        ToSql::<Text, Sqlite>::to_sql(&self.0, out)
    }
}

impl FromSql<Text, Sqlite> for PeerId {
    fn from_sql(binary: Option<&<Sqlite as Backend>::RawValue>) -> deserialize::Result<Self> {
        let binary = <String as FromSql<Text, Sqlite>>::from_sql(binary)?;
        Ok(PeerId(binary.into()))
    }
}

impl PeerId {
    pub fn new(s: String) -> Self {
        Self(s)
    }

    pub fn as_bytes(&self) -> Result<Vec<u8>, Error> {
        Ok(bs58::decode(&self.0).into_vec()?)
    }
}

impl<T: AsRef<[u8]>> From<T> for PeerId {
    fn from(data: T) -> Self {
        PeerId(bs58::encode(data.as_ref()).into_string())
    }
}
