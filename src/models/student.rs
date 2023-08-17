use std::fmt::{self, Display};

use rusqlite::{Connection, params};

use super::{Queriable, dollar::Dollar};

pub struct Student {
    id: String,
    name: String,
    credit: Dollar,
}
impl Student {
    fn new(name: String, credit: Dollar) -> Student {
        let mut split_names = name.split_whitespace();
        let id = split_names.next().unwrap().to_owned() + split_names.next().expect("No last name");

        Student { id, name, credit }
    }
    fn id(&self) -> String {
        let mut name = self.name.split_whitespace();
        name.next().unwrap().to_owned() + name.next().expect("No last name")
    }
}
impl Display for Student {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let id = self.id();
        writeln!(f, "| {:12} | {:22} | {:6} |", 
            id,
            self.name,
            self.credit
        )
    }
}
impl Queriable for Student {
    fn make_table(conn: &Connection) {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS students (
                id TEXT NOT NULL PRIMARY KEY,
                name TEXT NOT NULL,
                credit REAL NOT NULL
            )",
            (),).unwrap();
    }
    fn new_item(&self, conn: &Connection) {
        match conn.execute(
        "INSERT INTO products (id, name, credit) VALUES ($1, $2, $3)", 
            params![self.id, self.name, self.credit]) {
                Ok(a) => println!("{a}"),
                Err(a) => println!("{a}asf"),
        }
    }
    fn print_table(conn:&Connection) {
        let mut table = conn.prepare("select * from students").unwrap();
        let table = table.query_map([], |row| {
                Ok(
                    Student::new(row.get(1).unwrap(), row.get(2).unwrap())
                )
            });
        println!();
    }
}
