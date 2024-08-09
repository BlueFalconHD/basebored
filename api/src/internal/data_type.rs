use crate::internal::errors::Error;
use crate::internal::traits::Serializable;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Type {
    Bool,
    Int,
    Flt,
    Str,
    Unknown,
}

impl Serializable<Type> for Type {
    fn serialized_bytes(&self) -> Vec<u8> {
        vec![match self {
            Type::Bool => 0,
            Type::Int => 1,
            Type::Flt => 2,
            Type::Str => 3,
            Type::Unknown => 4,
        }]
    }

    fn deserialize_bytes(bytes: &[u8]) -> Result<Self, Error>
    where
        Self: Sized,
    {
        if bytes.len() != 1 {
            return Err(Error::TypeError(TypeError::InvalidSize {
                got: bytes.len(),
            }));
        }

        match bytes[0] {
            0 => Ok(Type::Bool),
            1 => Ok(Type::Int),
            2 => Ok(Type::Flt),
            3 => Ok(Type::Str),
            4 => Ok(Type::Unknown),
            _ => Err(Error::TypeError(TypeError::InvalidType { got: bytes[0] })),
        }
    }
}

/* -- ERRORS -- */

#[derive(Debug)]
pub(crate) enum TypeError {
    InvalidSize { got: usize },
    InvalidType { got: u8 },
}

impl std::fmt::Display for TypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeError::InvalidSize { got } => {
                write!(f, "Invalid size for Type: expected 1 byte, got {}", got)
            }
            TypeError::InvalidType { got } => {
                write!(f, "Invalid type byte: {}", got)
            }
        }
    }
}

impl std::error::Error for TypeError {}
