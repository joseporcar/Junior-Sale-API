// new sale: 
//     id:
//     date:
//     student:
//     product:

use std::fmt::{Display, self};

use chrono::{DateTime, Local};
use rusqlite::{Connection, params};

use super::{student::{Student, self}, product::{Product, self}, Queriable};

pub struct Sale {
    id: Option<u32>,
    date: DateTime<Local>,
    student: String,
    product: String,
    paid: bool,   
}
impl Sale {
    fn new(id: Option<u32>, date:DateTime<Local>, student: String, product: String, paid: bool) -> Sale {
        Sale { id, date, student, product, paid }
    }
}
impl Display for Sale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "| {:<8} | {:<20} | {:<22} | {:<22} |",
            self.id.unwrap_or(u32::MAX),
            self.date,
            self.student,
            self.product,
        )
    }
}

// impl Queriable for Sale {
//     fn make_table(conn: &Connection) {
//         conn.execute(
//             "CREATE TABLE IF NOT EXISTS sales (
//                 id INTEGER NOT NULL PRIMARY KEY,
//                 date TEXT NOT NULL,
//                 student TEXT NOT NULL,
//                 product TEXT NOT NULL,
//             )",
//             (),
//         ).unwrap();
//     }
//     fn new_item(&self, conn: &Connection) {
//         match conn.execute(
//             "INSERT INTO sales (date, student, product) VALUES ($1, $2, $3, $4)",
//             params![self.name, self.price, self.stock, self.cost],
//         ) {
//             Ok(a) => println!("{a}"),
//             Err(a) => println!("{a}asf"),
//         }
//     }
// }




