use crate::internal::byte_deserializer::ByteDeserializer;
use crate::internal::errors::Error;
use crate::internal::length_table::LengthTable;
use crate::internal::sheet::Sheet;
use crate::internal::traits::{PrettyPrintable, Serializable};

#[derive(Debug, Clone)]
pub(crate) struct Database {
    pub(crate) columns: Vec<Sheet>,
}

impl Database {
    pub(crate) fn new(columns: Vec<Sheet>) -> Self {
        Self { columns }
    }

    pub(crate) fn new_empty() -> Self {
        Self {
            columns: Vec::new(),
        }
    }

    pub(crate) fn adopt_sheet(&mut self, sheet: &Sheet) {
        let mut sheet = sheet.clone();
        sheet.adopt(self);
        self.columns.push(sheet);
    }

    pub(crate) fn adopt_sheets(&mut self, sheets: Vec<Sheet>) {
        for sheet in sheets {
            self.adopt_sheet(&sheet);
        }
    }

    pub(crate) fn get_sheet_mut(&mut self, name: &str) -> Option<&mut Sheet> {
        self.columns.iter_mut().find(|sheet| sheet.name == name)
    }
}

impl Serializable<Database> for Database {
    // all numbers are BE
    // length_table<Sheet> sheets: sheets serialized

    fn serialized_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        // ascii string magic bytes: "baseboredv1"
        bytes.extend_from_slice(&[98, 97, 115, 101, 98, 111, 114, 101, 100, 118, 49]);

        let sheets_bytes = LengthTable::serialize(&self.columns);
        bytes.extend(sheets_bytes);
        bytes
    }

    fn deserialize_bytes(bytes: &[u8]) -> Result<Database, Error> {
        let mut deserializer = ByteDeserializer::new(bytes);

        // check magic bytes
        let magic_bytes = deserializer.read_bytes(11)?;
        if magic_bytes != [98, 97, 115, 101, 98, 111, 114, 101, 100, 118, 49] {
            return Err(Error::DatabaseError(DatabaseError::InvalidMagicBytes));
        }

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
pub(crate) enum DatabaseError {
    InvalidMagicBytes,
}

impl std::fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DatabaseError::InvalidMagicBytes => write!(f, "Invalid magic bytes"),
        }
    }
}

impl std::error::Error for DatabaseError {}
