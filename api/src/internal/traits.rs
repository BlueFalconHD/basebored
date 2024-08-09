use crate::internal::errors::Error;

pub(crate) trait PrettyPrintable {
    fn pretty_print(&self, indent: usize) -> String;
}

pub(crate) trait Serializable<T> {
    fn serialized_bytes(&self) -> Vec<u8>;
    fn deserialize_bytes(bytes: &[u8]) -> Result<Self, Error>
    where
        Self: Sized;
}
