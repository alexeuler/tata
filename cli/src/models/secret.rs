use diesel::backend::Backend;
use diesel::deserialize::{self, FromSql};
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::Binary;
use diesel::sqlite::Sqlite;
use std::io::prelude::*;

#[derive(PartialEq, Eq, FromSqlRow, AsExpression, Clone)]
#[sql_type = "Binary"]
pub struct Secret(Vec<u8>);

impl std::fmt::Debug for Secret {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("0x")?;
        for byte in self.0.iter() {
            f.write_fmt(format_args!("{:x?}", byte))?;
        }
        Ok(())
    }
}

impl std::fmt::Display for Secret {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_fmt(format_args!("{:x?}", self.0))?;
        Ok(())
    }
}

impl ToSql<Binary, Sqlite> for Secret {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Sqlite>) -> serialize::Result {
        ToSql::<Binary, Sqlite>::to_sql(&self.0, out)
    }
}

impl FromSql<Binary, Sqlite> for Secret {
    fn from_sql(binary: Option<&<Sqlite as Backend>::RawValue>) -> deserialize::Result<Self> {
        let binary = <Vec<u8> as FromSql<Binary, Sqlite>>::from_sql(binary)?;
        Ok(Secret(binary.into()))
    }
}

impl Secret {
    pub fn new(data: Vec<u8>) -> Self {
        Self(data)
    }
}
