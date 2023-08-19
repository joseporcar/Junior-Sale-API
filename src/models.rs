use rusqlite::Connection;

pub mod dollar;
pub mod product;
pub mod student;
pub mod sale;

pub trait Queriable {
    fn make_table(conn: &Connection);
    fn new_item(&self, conn: &Connection);
    fn print_table(conn: &Connection);
}
