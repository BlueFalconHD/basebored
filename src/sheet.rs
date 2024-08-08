use crate::byte_deserializer::ByteDeserializer;
use crate::column::Column;
use crate::data_value::Value;
use crate::database::Database;
use crate::errors::Error;
use crate::id::Identifier;
use crate::length_table::LengthTable;
use crate::traits::{PrettyPrintable, Serializable};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Sheet {
    pub id: Identifier,
    pub name: String,
    pub columns: Vec<Column>,
    pub database: Option<Rc<Database>>,
}

impl Sheet {
    pub fn new(name: String, columns: Vec<Column>, database: Option<Rc<Database>>) -> Self {
        Self {
            id: Identifier::new(),
            name,
            columns,
            database,
        }
    }

    pub fn new_with_set_id(
        id: Identifier,
        name: String,
        columns: Vec<Column>,
        database: Option<Rc<Database>>,
    ) -> Self {
        Self {
            id,
            name,
            columns,
            database,
        }
    }

    pub fn adopt_column(&mut self, column: &Column) {
        let mut column = column.clone();
        column.adopt(self);
        self.columns.push(column);
    }

    pub fn adopt_columns(&mut self, columns: Vec<Column>) {
        for column in columns {
            self.adopt_column(&column);
        }
    }

    pub fn adopt(&mut self, database: &Database) {
        self.database = Some(Rc::new(database.clone()));
    }

    pub fn get_column_count(&self) -> usize {
        self.columns.len()
    }

    pub fn get_column_n(&self, index: usize) -> Option<&Column> {
        self.columns.get(index)
    }

    pub fn get_column_n_mut(&mut self, index: usize) -> Option<&mut Column> {
        self.columns.get_mut(index)
    }

    pub fn get_column_by_name(&self, name: &str) -> Option<&Column> {
        self.columns.iter().find(|column| column.name == name)
    }

    pub fn get_column_mut_by_name(&mut self, name: &str) -> Option<&mut Column> {
        self.columns.iter_mut().find(|column| column.name == name)
    }

    pub fn get_column_index(&self, name: &str) -> Option<usize> {
        self.columns.iter().position(|column| column.name == name)
    }

    pub fn get_column_by_id(&self, id: &Identifier) -> Option<&Column> {
        self.columns.iter().find(|column| column.id == *id)
    }

    pub fn get_column_mut_by_id(&mut self, id: &Identifier) -> Option<&mut Column> {
        self.columns.iter_mut().find(|column| column.id == *id)
    }

    pub fn get_column_index_by_id(&self, id: &Identifier) -> Option<usize> {
        self.columns.iter().position(|column| column.id == *id)
    }

    // insert_row takes in a vector of values and inserts them into the sheet
    pub fn insert_row(&mut self, values: Vec<Value>) -> Result<(), crate::errors::Error> {
        if values.len() != self.columns.len() {
            return Err(Error::SheetError(SheetError::InvalidRowLength {
                expected: self.columns.len(),
                got: values.len(),
            }));
        }
        for (i, value) in values.iter().enumerate() {
            self.columns[i].insert_value(value.clone());
        }
        Ok(())
    }
}

impl Serializable<Sheet> for Sheet {
    // all numbers are BE
    // u128 id: 16 bytes, uuid of the sheet
    // u32 name_length: 4 bytes, length of the name of the sheet
    // [u8; name_length] name: name_length bytes, name of the sheet
    // length_table<Column> columns: columns serialized

    fn serialized_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.id.serialized_bytes());
        bytes.extend_from_slice(&(self.name.len() as u32).to_be_bytes());
        bytes.extend_from_slice(self.name.as_bytes());
        let columns_bytes = LengthTable::serialize(&self.columns);
        bytes.extend_from_slice(&columns_bytes);
        bytes
    }

    fn deserialize_bytes(bytes: &[u8]) -> Result<Self, crate::errors::Error> {
        let mut deserializer = ByteDeserializer::new(bytes);
        let id = Identifier::deserialize_bytes(&deserializer.read_bytes(16)?)?;
        let name_length = deserializer.read_u32()?;
        let name = deserializer.read_string(name_length as usize)?;
        let columns = LengthTable::deserialize(deserializer.remaining_bytes())?;
        let mut sheet = Sheet::new_with_set_id(id, name, Vec::new(), None);
        sheet.adopt_columns(columns);
        Ok(sheet)
    }
}

impl PrettyPrintable for Sheet {
    fn pretty_print(&self, indent: usize) -> String {
        let mut result = String::new();
        let indent_str = " ".repeat(indent);
        result.push_str(&format!("{}Sheet: {}\n", indent_str, self.name));
        for column in &self.columns {
            result.push_str(&column.pretty_print(indent + 2));
        }
        result
    }
}

/* -- ERRORS -- */

#[derive(Debug)]
pub enum SheetError {
    SheetAlreadyAdopted {
        sheet_id: Identifier,
        sheet_name: String,
    },
    SheetNotAdopted {
        sheet_id: Identifier,
        sheet_name: String,
    },
    ColumnAlreadyExists {
        column_name: String,
    },
    ColumnNotFound {
        column_name: String,
    },
    InvalidRowLength {
        expected: usize,
        got: usize,
    },
}

impl std::fmt::Display for SheetError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SheetError::SheetAlreadyAdopted {
                sheet_id,
                sheet_name,
            } => {
                write!(
                    f,
                    "Sheet with id {} and name {} is already adopted",
                    sheet_id, sheet_name
                )
            }
            SheetError::SheetNotAdopted {
                sheet_id,
                sheet_name,
            } => {
                write!(
                    f,
                    "Sheet with id {} and name {} is not adopted",
                    sheet_id, sheet_name
                )
            }
            SheetError::ColumnAlreadyExists { column_name } => {
                write!(f, "Column with name {} already exists", column_name)
            }
            SheetError::ColumnNotFound { column_name } => {
                write!(f, "Column with name {} not found", column_name)
            }
            SheetError::InvalidRowLength { expected, got } => {
                write!(f, "Invalid row length, expected {} got {}", expected, got)
            }
        }
    }
}

impl std::error::Error for SheetError {}
