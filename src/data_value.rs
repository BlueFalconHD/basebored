use crate::data_type::Type;
use crate::errors::Error;
use crate::traits::Serializable;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Bool(bool),
    Int(i64),
    Flt(f64),
    Str(String),
    Nil,
}

impl Value {
    pub fn conforms_to(&self, other: &Type) -> bool {
        match self {
            Value::Bool(_) => *other == Type::Bool,
            Value::Int(_) => *other == Type::Int,
            Value::Flt(_) => *other == Type::Flt,
            Value::Str(_) => *other == Type::Str,
            Value::Nil => true,
        }
    }

    pub fn get_type(&self) -> Type {
        match self {
            Value::Bool(_) => Type::Bool,
            Value::Int(_) => Type::Int,
            Value::Flt(_) => Type::Flt,
            Value::Str(_) => Type::Str,
            Value::Nil => Type::Unknown,
        }
    }
}

impl Serializable<Value> for Value {
    fn serialized_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        match self {
            Value::Bool(b) => {
                bytes.push(Type::Bool as u8);
                bytes.push(*b as u8);
            }
            Value::Int(i) => {
                bytes.push(Type::Int as u8);
                bytes.extend_from_slice(&i.to_be_bytes());
            }
            Value::Flt(f) => {
                bytes.push(Type::Flt as u8);
                bytes.extend_from_slice(&f.to_be_bytes());
            }
            Value::Str(s) => {
                bytes.push(Type::Str as u8);
                bytes.extend_from_slice(&(s.len() as u32).to_be_bytes());
                bytes.extend_from_slice(s.as_bytes());
            }
            Value::Nil => {
                bytes.push(Type::Unknown as u8);
            }
        }

        bytes
    }

    fn deserialize_bytes(bytes: &[u8]) -> Result<Self, Error>
    where
        Self: Sized,
    {
        if bytes.len() < 1 {
            return Err(Error::ValueError(ValueError::InvalidSize {
                got: bytes.len(),
            }));
        }

        let value_type = Type::deserialize_bytes(&bytes[0..1])?;

        // if value_type is not in Type enum, it is an error
        if value_type != Type::Bool
            && value_type != Type::Int
            && value_type != Type::Flt
            && value_type != Type::Str
            && value_type != Type::Unknown
        {
            return Err(Error::ValueError(ValueError::InvalidType {
                got: value_type as u8,
            }));
        }

        match value_type {
            Type::Bool => {
                if bytes.len() != 2 {
                    return Err(Error::ValueError(ValueError::InvalidSize {
                        got: bytes.len(),
                    }));
                }

                Ok(Value::Bool(bytes[1] != 0))
            }
            Type::Int => {
                if bytes.len() != 9 {
                    return Err(Error::ValueError(ValueError::InvalidSize {
                        got: bytes.len(),
                    }));
                }

                let mut int_bytes = [0; 8];
                int_bytes.copy_from_slice(&bytes[1..9]);

                Ok(Value::Int(i64::from_be_bytes(int_bytes)))
            }
            Type::Flt => {
                if bytes.len() != 9 {
                    return Err(Error::ValueError(ValueError::InvalidSize {
                        got: bytes.len(),
                    }));
                }

                let mut flt_bytes = [0; 8];
                flt_bytes.copy_from_slice(&bytes[1..9]);

                Ok(Value::Flt(f64::from_be_bytes(flt_bytes)))
            }
            Type::Str => {
                if bytes.len() < 5 {
                    return Err(Error::ValueError(ValueError::InvalidSize {
                        got: bytes.len(),
                    }));
                }

                let mut str_len_bytes = [0; 4];
                str_len_bytes.copy_from_slice(&bytes[1..5]);
                let str_len = u32::from_be_bytes(str_len_bytes) as usize;

                if bytes.len() != 5 + str_len {
                    return Err(Error::ValueError(ValueError::InvalidSize {
                        got: bytes.len(),
                    }));
                } else {
                    let str_bytes = &bytes[5..];
                    let str = String::from_utf8(str_bytes.to_vec()).map_err(|_| {
                        Error::ValueError(ValueError::InvalidUtf8Str {
                            bytes: str_bytes.to_vec(),
                        })
                    })?;

                    Ok(Value::Str(str))
                }
            }
            Type::Unknown => Ok(Value::Nil),
        }
    }
}

/* -- ERRORS -- */

#[derive(Debug)]
pub enum ValueError {
    InvalidSize { got: usize },
    InvalidType { got: u8 },
    InvalidUtf8Str { bytes: Vec<u8> },
}

impl std::fmt::Display for ValueError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueError::InvalidSize { got } => {
                write!(f, "Invalid size for Value: expected 1 byte, got {}", got)
            }
            ValueError::InvalidType { got } => {
                write!(f, "Invalid type byte: {}", got)
            }
            ValueError::InvalidUtf8Str { bytes } => {
                write!(f, "Invalid UTF-8 string: {:?}", bytes)
            }
        }
    }
}

impl std::error::Error for ValueError {}
