use std::fmt::{self, Display};

use rusqlite::{params, Connection};

use super::{dollar::Dollar, Queriable};

pub struct Student {
    name: String,
    credit: Dollar,
}
impl Student {
    pub fn new(name: String) -> Student {
        Student { name, credit: Dollar::from(0) }
    }
    fn new_credit(name: String, credit: Dollar) -> Student {
        Student { name, credit }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Display for Student {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "| {:22} | {:<6} |", self.name, self.credit.to_string())
    }
}

impl Default for Student {
    fn default() -> Self {
        Student::new_credit("BAD".to_string(), Dollar(-999))
    }
}
impl Queriable for Student {
    fn make_table(conn: &Connection) {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS students (
                name TEXT NOT NULL,
                credit INTEGER NOT NULL
            )",
            (),
        )
        .unwrap();
    }
    fn new_item(&self, conn: &Connection) {
        match conn.execute(
            "INSERT INTO students (name, credit) VALUES ($1, $2)",
            params![self.name, self.credit],
        ) {
            Ok(a) => println!("{a}"),
            Err(a) => println!("{a}asf"),
        }
    }
    fn print_table(conn: &Connection) {
        let mut table = conn.prepare("select * from students").unwrap();
        let mut table = table.query_map([], |row| {
            Ok(
                Student::new_credit(row.get(0).expect("error at getting string"), 
                    row.get(1).expect("error at getting dollar")))
        }).unwrap();
        println!("| {:^22} | {:^6} |", "name", "credit");
        while let Some(student) = table.next() {
            println!("{}", student.unwrap_or_default())
        }
    }
}
