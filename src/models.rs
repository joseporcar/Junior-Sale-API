// Struct to avoid any problems with float accuracy while maintaining the
// understandability of integers through the API

use core::fmt;
use std::error::Error;
use std::fmt::Display;

use rusqlite::Connection;
use rusqlite::ToSql;
use rusqlite::params;
use rusqlite::types::ToSqlOutput;


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
pub trait Queriable {
    fn make_table(conn: &Connection);
    fn new_item(&self, conn: &Connection);
    fn id(&self) -> Box<dyn ToSql>;
    fn print_table<E: Error>(conn:&Connection) -> Result<(), E>;
}

#[derive(Debug)]
pub struct Product {
    pub id: Option<u32>,
    pub name: String,
    pub price: Dollar,
    pub stock: u32,
    pub cost: Dollar
}

impl Product {
    pub fn new(id: Option<u32>, name: String, price: String, stock: u32, cost: String) -> Product {
        Product {
            id,
            name,
            price: Dollar::from(&price),
            stock,
            cost: Dollar::from(&cost),
            
        }
    }
}
impl Queriable for Product {
    fn make_table(conn: &Connection) {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS products (
                id INTEGER NOT NULL PRIMARY KEY,
                name TEXT NOT NULL,
                price REAL NOT NULL,
                stock INTEGER NOT NULL,
                cost REAL NOT NULL
            )",
            (),).unwrap();
    }

    fn print_table(conn:&Connection) -> Result<()>  {
        let mut product_list = conn.prepare("select * from products")?;
        let product_list = product_list.query_map([], |p| {
            Ok (Product::new(
                p.get(0)?, 
                p.get(1)?,
                p.get::<_, f32>(2).unwrap().to_string(),
                p.get(3).unwrap(),
                p.get::<_, f32>(4)?.to_string(),)
            )
        })?;
    
        println!("| {:^5} | {:^22} | {:<5} | {:^5} | {:^6} |", "id", "name", "price", "stock", "cost");
        for product in product_list {
            println!("{}", product.unwrap_or_default())
        };
        Ok(())
    }

    fn new_item(&self, conn: &Connection) {
        match conn.execute(
        "INSERT INTO products (name, price, stock, cost) VALUES ($1, $2, $3, $4)", 
            params![self.name, self.price, self.stock, self.cost]) {
                Ok(a) => println!("{a}"),
                Err(a) => println!("{a}asf"),
        }
    }

    fn id(&self) -> Box<dyn ToSql> {
        Box::new(self.id)
    }
}
impl Default for Product {
    fn default() -> Self {
        Product { id: None, name: "default".to_owned(), price: Dollar::from("0"), stock: 0, cost: Dollar::from("0") }
    }
}
impl Display for Product {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,
            "| {:<5} | {:<22} | {:<5} | {:<5} | {:<6} |",
                self.id.unwrap_or(999),
                self.name,
                self.price.to_string(),
                self.stock,
                self.cost.to_string())
    }
}
pub struct Student {
    name: String,
    credit: Dollar,
}

impl Display for Student {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut name = self.name.split_whitespace();
        let id = name.next().unwrap().to_owned() + name.next().expect("No last name");
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
            params![self.name, self.id(), self.credit]) {
                Ok(a) => println!("{a}"),
                Err(a) => println!("{a}asf"),
        }
    }
    fn id(&self) -> Box<dyn ToSql> {
        let mut name = self.name.split_whitespace();
        Box::new(name.next().unwrap().to_owned() + name.next().expect("No last name"))
    }
}

