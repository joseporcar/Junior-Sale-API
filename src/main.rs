mod models;
use models::{dollar::{Dollar, ToDollar}, product::Product, student::Student, Queriable, sale::Sale};

use rusqlite::{params, Connection, Result};
use egui;

use std::env::args;

fn main() -> Result<()> {
    let conn = Connection::open("database.db")?;

    

    Product::make_table(&conn);
    Student::make_table(&conn);

    println!("Welcome to the system!");

    let mut buffer = args();

    //let mut buffer = String::new();
    //loop {
        //get_input(&mut buffer);
        // buffer = buffer.trim().to_owned();
    match args().count() {
        0 => {
            dbg!(&buffer);
            let buffer = buffer.next().unwrap();
            if buffer == "exit" || buffer == "quit" {
                panic!();
            } else if buffer == "drop-all" {
                conn.execute("drop table products", [])?;
                panic!();
            } else {
                println!("no input");
                one_input(&conn, &buffer)?;
            }
        }
        2 => {let buffer = buffer.nth(1).unwrap();one_input(&conn, &buffer)?;},
        3 => {            
            dbg!("new-stu");
            let mut args = args();
            args.next();
            if args.next().unwrap() == "new-student" {
                Student::new(
                    args.next().unwrap().to_owned(),
                )
                .new_item(&conn);
            } else {
                panic!("exit")
            }}

        6 => {
            dbg!("yes");
            let mut args = args();
            args.next();
            if args.next().unwrap() == "new-product" {
                Product::new(
                    None,
                    args.next().unwrap().to_owned(),
                    args.next().unwrap().to_dollar(),
                    args.next().unwrap().parse().unwrap(),
                    args.next().unwrap().to_dollar(),
                )
                .new_item(&conn);
            } else {
                panic!("exit")
            }
        }
        _ => (),
    }
        //buffer.clear();
    //}
    Ok(())
}

fn one_input(conn: &Connection, buffer: &str) -> Result<()> {
    if buffer == "plist" {
        // print products list and id alphabetically ordered
        Product::print_table(conn)
    } else if buffer == "slist" {
        Student::print_table(conn)
    }
    Ok(())

}

// fn get_input(buffer: &mut String) {
//     stdin().read_line(buffer);
// }
