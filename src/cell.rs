use crate::column::Column;
use crate::data_type::Type;
use crate::data_value::Value;
use crate::errors::Error;
use crate::traits::{PrettyPrintable, Serializable};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Cell {
    pub value: Value,
    pub column: Option<Rc<Column>>,
}

impl Cell {
    pub fn new(value: Value, column: Option<Rc<Column>>) -> Self {
        Self { value, column }
    }

    pub fn adopt(&mut self, column: &Column) {
        self.column = Some(Rc::new(column.clone()));
    }

    pub fn get_value(&self) -> &Value {
        &self.value
    }

    pub fn set_value(&mut self, value: Value) -> Result<(), Error> {
        // If the cell isn't yet adopted, return an error
        match &self.column {
            Some(_) => {
                if self
                    .value
                    .conforms_to(&self.column.as_ref().unwrap().value_type)
                {
                    self.value = value;
                    Ok(())
                } else {
                    Err(Error::CellError(CellError::IncompatibleType {
                        column_name: self.column.as_ref().unwrap().name.clone(),
                        expected: self.column.as_ref().unwrap().value_type,
                        got: value.get_type(),
                    }))
                }
            }
            None => Err(Error::CellError(CellError::IsNotAdopted)),
        }
    }
}

impl Serializable<Cell> for Cell {
    fn serialized_bytes(&self) -> Vec<u8> {
        self.value.serialized_bytes()
    }

    fn deserialize_bytes(bytes: &[u8]) -> Result<Self, Error> {
        let value = Value::deserialize_bytes(bytes)?;
        Ok(Self::new(value, None))
    }
}

impl PrettyPrintable for Cell {
    fn pretty_print(&self, indent: usize) -> String {
        let indent_str = " ".repeat(indent);
        format!("{}Cell: {:?}\n", indent_str, self.value)
    }
}

/* -- ERRORS -- */

#[derive(Debug)]
pub enum CellError {
    IsAlreadyAdopted,
    IsNotAdopted,
    IncompatibleType {
        column_name: String,
        expected: Type,
        got: Type,
    },
}

impl std::fmt::Display for CellError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CellError::IsAlreadyAdopted => write!(f, "Cell is already adopted"),
            CellError::IsNotAdopted => write!(f, "Cell is not adopted"),
            CellError::IncompatibleType {
                column_name,
                expected,
                got,
            } => write!(
                f,
                "Incompatible type for cell in column '{}': expected {:?}, got {:?}",
                column_name, expected, got
            ),
        }
    }
}

impl std::error::Error for CellError {}
