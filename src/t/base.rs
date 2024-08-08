rust
use crate::sheet::{Sheet, SheetError};
use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};

/// Represents the main registry for sheets.
pub struct Base {
    sheets: HashMap<String, Sheet>,
}

impl Base {
    /// Creates a new, empty `Base`.
    pub fn new() -> Self {
        Self {
            sheets: HashMap::new(),
        }
    }

    /// Adds a new sheet to the registry if it does not already exist.
    pub fn add_sheet(&mut self, sheet: Sheet) -> Result<(), String> {
        match self.sheets.insert(sheet.get_name().clone(), sheet) {
            Some(_) => Err("Sheet with the same name already exists.".to_string()),
            None => Ok(()),
        }
    }

    /// Retrieves a reference to a sheet by its name.
    pub fn get_sheet(&self, name: &str) -> Result<&Sheet, String> {
        self.sheets
            .get(name)
            .ok_or_else(|| "Sheet not found.".to_string())
    }

    /// Retrieves a mutable reference to a sheet by its name.
    pub fn get_sheet_mut(&mut self, name: &str) -> Result<&mut Sheet, String> {
        self.sheets
            .get_mut(name)
            .ok_or_else(|| "Sheet not found.".to_string())
    }

    /// Removes a sheet from the registry by its name.
    pub fn remove_sheet(&mut self, name: &str) -> Result<(), String> {
        if self.sheets.remove(name).is_some() {
            Ok(())
        } else {
            Err("Sheet not found.".to_string())
        }
    }
}

impl Display for Base {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Base Registry: {} sheets registered", self.sheets.len())
    }
}