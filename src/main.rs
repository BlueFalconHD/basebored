mod byte_deserializer;
mod cell;
mod column;
mod data_type;
mod data_value;
mod database;
mod errors;
mod id;
mod length_table;
mod sheet;
mod traits;

use cell::Cell;
use column::Column;
use data_type::Type;
use data_value::Value;
use database::Database;
use sheet::Sheet;
use traits::{PrettyPrintable, Serializable};

use std::{fs::File, io::Write};

fn main() {
    // Create a new database
    let mut db = Database::new_empty();

    // Create a new sheet with columns:
    // - Name: Str
    // - Age: int
    // - Salary: int
    let mut employees = Sheet::new(
        "Employees".to_string(),
        vec![
            Column::new("Name".to_string(), Type::Str, None),
            Column::new("Age".to_string(), Type::Int, None),
            Column::new("Salary".to_string(), Type::Int, None),
        ],
        None,
    );

    // - Name: "Alice"
    // - Age: 25
    // - Salary: 1000
    employees
        .insert_row(vec![
            Value::Str("Alice".to_string()),
            Value::Int(25),
            Value::Int(1000),
        ])
        .unwrap();

    // - Name: "Bob"
    // - Age: 30
    // - Salary: 2000
    employees
        .insert_row(vec![
            Value::Str("Bob".to_string()),
            Value::Int(30),
            Value::Int(2000),
        ])
        .unwrap();

    // - Name: "Charlie"
    // - Age: 35
    // - Salary: 3000
    employees
        .insert_row(vec![
            Value::Str("Charlie".to_string()),
            Value::Int(35),
            Value::Int(3000),
        ])
        .unwrap();

    // Add the sheet to the database
    db.adopt_sheet(&mut employees);

    // Print the database
    println!("{}", db.pretty_print(0));

    // Serialize the database to a file
    let bytes: Vec<u8> = db.serialized_bytes();

    // Write the bytes to a file
    let mut file = File::create("database.bin").unwrap();
    file.write_all(&bytes).unwrap();

    // Deserialize the database from the file
    let bytes = std::fs::read("database.bin").unwrap();
    let db = Database::deserialize_bytes(&bytes).unwrap();

    // Pretty print the database
    println!("{}", db.pretty_print(0));
}
