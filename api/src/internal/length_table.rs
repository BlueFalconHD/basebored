use crate::internal::errors::Error;
use crate::internal::traits::Serializable;
use std::convert::TryInto;

pub(crate) struct LengthTable;

impl LengthTable {
    pub(crate) fn serialize<T: Serializable<T>>(objects: &Vec<T>) -> Vec<u8> {
        let mut bytes = Vec::new();

        // Serialize each object and store its length
        let mut object_lengths = Vec::new();
        let mut serialized_objects = Vec::new();

        for object in objects {
            let serialized_object = object.serialized_bytes();
            object_lengths.push(serialized_object.len() as u32);
            serialized_objects.extend(serialized_object);
        }

        // Serialize the length table length
        let length_table_length = object_lengths.len() * 4 + serialized_objects.len();
        bytes.extend(&(length_table_length as u32).to_le_bytes());

        // Serialize the number of objects
        bytes.extend(&(objects.len() as u32).to_le_bytes());

        // Serialize the object lengths
        for length in object_lengths {
            bytes.extend(&length.to_le_bytes());
        }

        // Serialize the objects
        bytes.extend(&serialized_objects);

        bytes
    }

    pub(crate) fn deserialize<T: Serializable<T>>(bytes: &[u8]) -> Result<Vec<T>, Error> {
        let mut cursor = 0;

        // Read the length table length
        let length_table_length =
            u32::from_le_bytes(bytes[cursor..cursor + 4].try_into().unwrap()) as usize;
        cursor += 4;

        // Read the number of objects
        let objects_count =
            u32::from_le_bytes(bytes[cursor..cursor + 4].try_into().unwrap()) as usize;
        cursor += 4;

        // Read the object lengths
        let mut object_lengths = Vec::with_capacity(objects_count);
        for _ in 0..objects_count {
            let length = u32::from_le_bytes(bytes[cursor..cursor + 4].try_into().unwrap()) as usize;
            cursor += 4;
            object_lengths.push(length);
        }

        // Read and deserialize the objects
        let mut objects = Vec::with_capacity(objects_count);
        for length in object_lengths {
            let end = cursor + length;
            let object_bytes = &bytes[cursor..end];
            let object = T::deserialize_bytes(object_bytes)?;
            objects.push(object);
            cursor = end;
        }

        Ok(objects)
    }
}
