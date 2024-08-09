use crate::internal::errors::Error;
use std::fmt::{self, Display, Formatter};
use uuid::Uuid;

use crate::internal::traits::Serializable;

#[derive(Debug, Clone, PartialEq, Copy)]
pub(crate) struct Identifier {
    pub(crate) id: Uuid,
}

impl Identifier {
    pub(crate) fn new() -> Self {
        Self { id: Uuid::new_v4() }
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Identifier {}", self.id)
    }
}

impl Serializable<Identifier> for Identifier {
    fn serialized_bytes(&self) -> Vec<u8> {
        self.id.as_bytes().to_vec()
    }

    fn deserialize_bytes(bytes: &[u8]) -> Result<Self, Error>
    where
        Self: Sized,
    {
        if bytes.len() != 16 {
            return Err(Error::UuidError(UuidError::InvalidSize {
                got: bytes.len(),
            }));
        }

        let mut id_bytes = [0; 16];
        id_bytes.copy_from_slice(&bytes[..16]);
        Ok(Self {
            id: Uuid::from_bytes(id_bytes),
        })
    }
}

/* -- ERRORS -- */

#[derive(Debug)]
pub(crate) enum UuidError {
    InvalidSize { got: usize },
}

impl Display for UuidError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            UuidError::InvalidSize { got } => {
                write!(f, "Invalid size for UUID: expected 16 bytes, got {}", got)
            }
        }
    }
}

// UuidError should be a Error
impl std::error::Error for UuidError {}
