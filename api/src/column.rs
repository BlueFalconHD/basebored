use crate::internal::cell::CellError;
use crate::internal::column::Column as InternalColumn;
use crate::internal::id::Identifier;
use crate::type_::DataType;
use crate::value::Value;

use crate::internal::errors::Error;

#[derive(Debug, Clone)]
pub struct Column {
    pub name: String,
    pub id: Identifier,
    pub type_: DataType,
    pub cells: Vec<Value>,
}

impl Column {
    pub fn new(name: String, id: Identifier, type_: DataType, cells: Vec<Value>) -> Self {
        Self {
            name,
            id,
            type_,
            cells,
        }
    }

    pub fn new_empty(name: String, id: Identifier, type_: DataType) -> Self {
        Self {
            name,
            id,
            type_,
            cells: Vec::new(),
        }
    }

    pub fn push(&mut self, value: Value) -> Result<(), Error> {
        // Check if the value is of the correct type
        if value.conforms_to(&self.type_) {
            self.cells.push(value);
            Ok(())
        } else {
            Err(Error::CellError(CellError::IncompatibleType {
                column_name: self.name.clone(),
                expected: self.type_.clone().as_internal_data_type(),
                got: value.as_internal_data_type(),
            }))
        }
    }

    /* INTERNALs */

    pub(crate) fn from_internal_column(internal_column: InternalColumn) -> Self {
        let cells = internal_column
            .cells
            .iter()
            .map(|cell| Value::from_internal_cell(cell.clone()))
            .collect();
        Self {
            name: internal_column.name.clone(),
            id: internal_column.id.clone(),
            type_: DataType::from_internal_data_type(internal_column.value_type),
            cells,
        }
    }

    pub(crate) fn as_internal_column(&self) -> InternalColumn {
        let cells = self
            .cells
            .iter()
            .map(|cell| cell.as_internal_cell())
            .collect();
        let mut c = InternalColumn::new_with_set_id(
            self.id.clone(),
            self.name.clone(),
            DataType::as_internal_data_type(&self.type_),
            None,
        );

        c.adopt_cells(cells);

        c
    }
}
