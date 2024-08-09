use crate::internal::byte_deserializer::ByteError;
use crate::internal::cell::CellError;
use crate::internal::column::ColumnError;
use crate::internal::data_type::TypeError;
use crate::internal::data_value::ValueError;
use crate::internal::database::DatabaseError;
use crate::internal::id::UuidError;
use crate::internal::sheet::SheetError;

#[derive(Debug)]
pub(crate) enum Error {
    UuidError(UuidError),
    ValueError(ValueError),
    TypeError(TypeError),
    CellError(CellError),
    ColumnError(ColumnError),
    SheetError(SheetError),
    DatabaseError(DatabaseError),
    ByteError(ByteError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::UuidError(err) => write!(f, "{}", err),
            Error::ValueError(err) => write!(f, "{}", err),
            Error::TypeError(err) => write!(f, "{}", err),
            Error::CellError(err) => write!(f, "{}", err),
            Error::ColumnError(err) => write!(f, "{}", err),
            Error::SheetError(err) => write!(f, "{}", err),
            Error::DatabaseError(err) => write!(f, "{}", err),
            Error::ByteError(err) => write!(f, "{}", err),
        }
    }
}
