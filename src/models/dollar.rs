use std::fmt::Display;

use rusqlite::{ToSql, types::{ToSqlOutput, FromSql}};

// Struct to avoid any problems with float accuracy while maintaining the
// understandability of integers through the API

#[derive (Debug)]
pub struct Dollar(i32);
impl Dollar {
    pub fn from(s: &str) -> Dollar {
        match s.split_once('.') {
            Some((whole, decimal)) => Dollar(
                whole.parse::<i32>().expect("error at parsing whole") * 100
                    + decimal.parse::<i32>().expect("error at parsing decimal"),
            ),
            None => Dollar (s.parse::<i32>().expect("Error at parsing dollar") * 100)
        }
    }

    pub fn get(&self) -> i32{
        self.0
    }

    pub fn add(&self, other: &Dollar) -> Dollar {
        Dollar(self.0 + other.0)
    }

    pub fn sub(&self, other: &Dollar) -> Dollar {
        Dollar(self.0 - other.0)
    }
}

impl Display for Dollar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let decimal = self.get() % 100;
        let whole = self.get() / 100;

        write!(f, "{whole}.{decimal:02}")
    }
}

impl ToSql for Dollar {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(format!("{}", self)))
    }
}

impl FromSql for Dollar {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        dbg!(Dollar(i32::column_result(value).unwrap()));
        Ok(Dollar(i32::column_result(value).unwrap()))
    }
}