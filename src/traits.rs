use crate::errors::Error;

pub trait PrettyPrintable {
    fn pretty_print(&self, indent: usize) -> String;
}

pub trait Serializable<T> {
    fn serialized_bytes(&self) -> Vec<u8>;
    fn deserialize_bytes(bytes: &[u8]) -> Result<Self, Error>
    where
        Self: Sized;
}
