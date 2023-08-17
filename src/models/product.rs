use std::fmt::{self, Display};

use rusqlite::{Connection, params};

use super::{Queriable, dollar::Dollar};

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

    fn print_table(conn:&Connection) {
        let mut product_list = conn.prepare("select * from products").unwrap();
        let product_list = product_list.query_map([], |p| {
            Ok (Product::new(
                p.get(0).unwrap(), 
                p.get(1).unwrap(),
                p.get::<_, f32>(2).unwrap().to_string(),
                p.get(3).unwrap(),
                p.get::<_, f32>(4).unwrap().to_string(),)
            )
        }).unwrap();
    
        println!("| {:^5} | {:^22} | {:<5} | {:^5} | {:^6} |", "id", "name", "price", "stock", "cost");
        for product in product_list {
            println!("{}", product.unwrap_or_default())
        };
    }

    fn new_item(&self, conn: &Connection) {
        match conn.execute(
        "INSERT INTO products (name, price, stock, cost) VALUES ($1, $2, $3, $4)", 
            params![self.name, self.price, self.stock, self.cost]) {
                Ok(a) => println!("{a}"),
                Err(a) => println!("{a}asf"),
        }
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
