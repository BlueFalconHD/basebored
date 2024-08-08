use crate::byte_deserializer::ByteDeserializer;
use crate::cell::Cell;
use crate::data_type::Type;
use crate::data_value::Value;
use crate::errors::Error;
use crate::id::Identifier;
use crate::length_table::LengthTable;
use crate::sheet::Sheet;
use crate::traits::{PrettyPrintable, Serializable};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Column {
    pub id: Identifier,
    pub name: String,
    pub value_type: Type,
    pub cells: Vec<Cell>,
    pub sheet: Option<Rc<Sheet>>,
}

impl Column {
    pub fn new(name: String, value_type: Type, sheet: Option<Rc<Sheet>>) -> Self {
        Self {
            id: Identifier::new(),
            name,
            value_type,
            cells: Vec::new(),
            sheet,
        }
    }

    /// adopt_cell takes a cell, clones it, adopts it, and adds it to cells
    pub fn adopt_cell(&mut self, cell: &Cell) {
        let mut cell = cell.clone();
        cell.adopt(self);
        self.cells.push(cell);
    }

    pub fn adopt_cells(&mut self, cells: Vec<Cell>) {
        for cell in cells {
            self.adopt_cell(&cell);
        }
    }

    pub fn adopt(&mut self, sheet: &Sheet) {
        self.sheet = Some(Rc::new(sheet.clone()));
    }

    pub fn get_row_count(&self) -> usize {
        self.cells.len()
    }

    pub fn get_cell(&self, index: usize) -> Option<&Cell> {
        self.cells.get(index)
    }

    pub fn get_cell_mut(&mut self, index: usize) -> Option<&mut Cell> {
        self.cells.get_mut(index)
    }

    // Add a cell to the column with a value and adopt it
    pub fn insert_value(&mut self, value: Value) {
        let cell = Cell::new(value, None);
        self.adopt_cell(&cell);
    }
}

impl Serializable<Column> for Column {
    // all numbers are BE
    // u128 id: 16 bytes, uuid of the column
    // u32 name_length: 4 bytes, length of the name of the column
    // [u8; name_length] name: name_length bytes, name of the column
    // u8 value_type: 1 byte, type of the column
    // length_table<Cell> cells: cells serialized

    fn serialized_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.id.serialized_bytes());
        bytes.extend_from_slice(&(self.name.len() as u32).to_be_bytes());
        bytes.extend_from_slice(self.name.as_bytes());
        bytes.push(self.value_type.serialized_bytes()[0]);

        // LengthTable for cells
        let table_bytes = LengthTable::serialize(&self.cells);

        bytes.extend_from_slice(&table_bytes);

        bytes
    }

    fn deserialize_bytes(bytes: &[u8]) -> Result<Self, Error> {
        let mut deserializer = ByteDeserializer::new(bytes);
        let id = Identifier::deserialize_bytes(&deserializer.read_bytes(16)?)?;
        let name_length = deserializer.read_u32()?;
        let name = deserializer.read_string(name_length as usize)?;
        let value_type = Type::deserialize_bytes(&[deserializer.read_u8()?])?;

        // Pass the remaining bytes to the LengthTable deserializer
        let cells = LengthTable::deserialize(deserializer.remaining_bytes())?;

        // Initialize the column with an empty vec of cells
        let mut column = Column {
            id,
            name,
            value_type,
            cells: Vec::new(),
            sheet: None,
        };

        // Adopt the cells
        // This is done after the column is created because the cells need to know the column they are being adopted by
        column.adopt_cells(cells);

        Ok(column)
    }
}

impl PrettyPrintable for Column {
    fn pretty_print(&self, indent: usize) -> String {
        let mut result = String::new();
        let indent_str = " ".repeat(indent);
        result.push_str(&format!("{}Column: {}\n", indent_str, self.name));
        for cell in &self.cells {
            result.push_str(&cell.pretty_print(indent + 2));
        }
        result
    }
}

/* -- ERRORS -- */

#[derive(Debug)]
pub enum ColumnError {
    ColumnAlreadyAdopted {
        column_id: Identifier,
        column_name: String,
    },
    ColumnNotAdopted {
        column_id: Identifier,
        column_name: String,
    },
}

impl std::fmt::Display for ColumnError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ColumnError::ColumnAlreadyAdopted {
                column_id,
                column_name,
            } => write!(
                f,
                "Column with id {} and name {} is already adopted",
                column_id, column_name
            ),
            ColumnError::ColumnNotAdopted {
                column_id,
                column_name,
            } => write!(
                f,
                "Column with id {} and name {} is not adopted",
                column_id, column_name
            ),
        }
    }
}

impl std::error::Error for ColumnError {}
