use crate::byte_deserializer::ByteDeserializer;
use crate::errors::Error;
use crate::length_table::LengthTable;
use crate::sheet::Sheet;
use crate::traits::{PrettyPrintable, Serializable};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Database {
    pub columns: Vec<Sheet>,
}

impl Database {
    pub fn new(columns: Vec<Sheet>) -> Self {
        Self { columns }
    }

    pub fn new_empty() -> Self {
        Self {
            columns: Vec::new(),
        }
    }

    pub fn adopt_sheet(&mut self, sheet: &Sheet) {
        let mut sheet = sheet.clone();
        sheet.adopt(self);
        self.columns.push(sheet);
    }

    pub fn adopt_sheets(&mut self, sheets: Vec<Sheet>) {
        for sheet in sheets {
            self.adopt_sheet(&sheet);
        }
    }
}

impl Serializable<Database> for Database {
    // all numbers are BE
    // length_table<Sheet> sheets: sheets serialized

    fn serialized_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        let sheets_bytes = LengthTable::serialize(&self.columns);
        bytes.extend(sheets_bytes);
        bytes
    }

    fn deserialize_bytes(bytes: &[u8]) -> Result<Database, Error> {
        let deserializer = ByteDeserializer::new(bytes);
        let sheets = LengthTable::deserialize(deserializer.remaining_bytes())?;

        let mut database = Database::new(Vec::new());
        database.adopt_sheets(sheets);

        Ok(database)
    }
}

impl PrettyPrintable for Database {
    fn pretty_print(&self, indent: usize) -> String {
        let mut result = String::new();
        let indent_str = " ".repeat(indent);
        result.push_str(&format!("{}Database:\n", indent_str));
        for sheet in &self.columns {
            result.push_str(&sheet.pretty_print(indent + 2));
        }
        result
    }
}

/* -- ERRORS -- */

#[derive(Debug)]
pub enum DatabaseError {
    PLACEHOLDER,
}

impl std::fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DatabaseError::PLACEHOLDER => write!(f, "Placeholder error"),
        }
    }
}

impl std::error::Error for DatabaseError {}
