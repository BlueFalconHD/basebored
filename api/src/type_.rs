use crate::internal::data_type::Type;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    Int,
    Flt,
    Str,
    Bool,
}

impl DataType {
    pub(crate) fn from_internal_data_type(internal: Type) -> Self {
        match internal {
            Type::Int => DataType::Int,
            Type::Flt => DataType::Flt,
            Type::Str => DataType::Str,
            Type::Bool => DataType::Bool,
            Type::Unknown => unreachable!(),
        }
    }

    pub(crate) fn as_internal_data_type(&self) -> Type {
        match self {
            DataType::Int => Type::Int,
            DataType::Flt => Type::Flt,
            DataType::Str => Type::Str,
            DataType::Bool => Type::Bool,
        }
    }
}
