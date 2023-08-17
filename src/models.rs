
use rusqlite::Connection;

pub mod dollar;
pub mod student;
pub mod product;

pub trait Queriable {
    fn make_table(conn: &Connection);
    fn new_item(&self, conn: &Connection);
    fn print_table(conn:&Connection);
}

