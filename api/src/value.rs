use crate::internal::cell::Cell as InternalCell;
use crate::internal::data_type::Type as InternalDataType;
use crate::internal::data_value::Value as InternalValue;
use crate::type_::DataType;

#[derive(Debug, Clone)]
pub enum Value {
    Bool(bool),
    Int(i64),
    Flt(f64),
    Str(String),
    Nil,
}

impl Value {
    pub fn conforms_to(&self, type_: &DataType) -> bool {
        match self {
            Value::Bool(_) => *type_ == DataType::Bool,
            Value::Int(_) => *type_ == DataType::Int,
            Value::Flt(_) => *type_ == DataType::Flt,
            Value::Str(_) => *type_ == DataType::Str,
            Value::Nil => true,
        }
    }

    pub(crate) fn as_internal_value(&self) -> InternalValue {
        match self {
            Value::Bool(b) => InternalValue::Bool(*b),
            Value::Int(i) => InternalValue::Int(*i),
            Value::Flt(f) => InternalValue::Flt(*f),
            Value::Str(s) => InternalValue::Str(s.clone()),
            Value::Nil => InternalValue::Nil,
        }
    }

    pub(crate) fn from_internal_value(value: InternalValue) -> Self {
        match value {
            InternalValue::Bool(b) => Value::Bool(b),
            InternalValue::Int(i) => Value::Int(i),
            InternalValue::Flt(f) => Value::Flt(f),
            InternalValue::Str(s) => Value::Str(s),
            InternalValue::Nil => Value::Nil,
        }
    }

    pub(crate) fn as_internal_cell(&self) -> InternalCell {
        InternalCell::new(self.as_internal_value(), None)
    }

    pub(crate) fn from_internal_cell(cell: InternalCell) -> Self {
        Value::from_internal_value(cell.value)
    }

    pub(crate) fn as_internal_data_type(&self) -> InternalDataType {
        match self {
            Value::Bool(_) => InternalDataType::Bool,
            Value::Int(_) => InternalDataType::Int,
            Value::Flt(_) => InternalDataType::Flt,
            Value::Str(_) => InternalDataType::Str,
            Value::Nil => InternalDataType::Unknown,
        }
    }
}
