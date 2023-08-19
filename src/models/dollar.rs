use std::fmt::Display;

use rusqlite::{
    types::{FromSql, ToSqlOutput},
    ToSql,
};

// Struct to avoid any problems with float accuracy while maintaining the
// understandability of integers through the API
pub trait ToDollar {
    fn to_dollar(&self) -> Dollar;
}

impl ToDollar for i32 {
    fn to_dollar(&self) -> Dollar {
        Dollar::from(*self)
    }
}

impl ToDollar for &str {
    fn to_dollar(&self) -> Dollar {
        Dollar::from(*self)

    }
}

impl ToDollar for String {
    fn to_dollar(&self) -> Dollar {
        Dollar::from(self)
    }
}

#[derive(Debug)]
pub struct Dollar(pub i32);
impl Dollar {

    pub fn get(&self) -> i32 {
        self.0
    }

    pub fn add(&self, other: &Dollar) -> Dollar {
        Dollar(self.0 + other.0)
    }

    pub fn sub(&self, other: &Dollar) -> Dollar {
        Dollar(self.0 - other.0)
    }
}

impl From<&str> for Dollar {
    fn from(s: &str) -> Dollar {
        match s.split_once('.') {
            Some((whole, decimal)) => {
                let decimal = format!("{:0<2}", decimal);
                Dollar(whole.parse::<i32>().expect("error at parsing whole") * 100
                    + decimal.parse::<i32>().expect("error at parsing decimal"))
            },
            None => Dollar(s.parse::<i32>().expect("Error at parsing dollar") * 100),
        }
    }
}

impl From<&String> for Dollar {
    fn from(s: &String) -> Dollar {
        match s.split_once('.') {
            Some((whole, decimal)) => {
                let decimal = format!("{:0<2}", decimal);
                Dollar(whole.parse::<i32>().expect("error at parsing whole") * 100
                    + decimal.parse::<i32>().expect("error at parsing decimal"))
            },
            None => Dollar(s.parse::<i32>().expect("Error at parsing dollar") * 100),
        }
    }
}

impl From<i32> for Dollar {
    fn from(value: i32) -> Self {
        Dollar(value)
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
        Ok(ToSqlOutput::from(format!("{}", self.0)))
    }
}

impl FromSql for Dollar {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        Ok(Dollar(i32::column_result(value).unwrap()))
    }
}