mod models;
use models::{student::Student, Queriable, product::Product, dollar::Dollar};


use std::io::stdin;
use rusqlite::{params, Connection, Result};

fn main() -> Result<()>{
    let conn = Connection::open("database.db")?;

    println!("{}", Dollar::from("0.50"));

    Product::make_table(&conn);
    Student::make_table(&conn);
    
    println!("Welcome to the system!");
    
    let mut buffer = String::new();
    loop {
        get_input(&mut buffer);
        buffer = buffer.trim().to_owned();
        match buffer.matches(' ').count() {
            0 => {
                dbg!(&buffer);
                if buffer == "exit" || buffer == "quit" {
                    break
                } else if buffer == "drop-all" {
                    conn.execute("drop table products", [])?;
                    break
                }
                else {
                    one_input(&conn, &buffer)?;
                }
            }
            1 => {

            }
            4 => {
                let mut args = buffer.split_whitespace();
                if args.next().unwrap() == "new-product" {
                    Product::new(
                        None,
                        args.next().unwrap().to_owned(), 
                        args.next().unwrap().to_owned(), 
                        args.next().unwrap().parse().unwrap(),
                        args.next().unwrap().to_owned()).new_item(&conn);
                    
                }
                else{ panic!("exit")}
            }
            _ => ()
        }
        buffer.clear();
    }
    Ok(())
}

fn one_input(conn: &Connection, buffer: &str) -> Result<()>{
    if buffer == "plist" {
        // print products list and id alphabetically ordered
        get_all_products(conn)?
    } 
    Ok(()) 
}

fn get_all_products(conn: &Connection) -> Result<()>{
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

fn get_input(buffer: &mut String) {
    stdin().read_line(buffer);
}

