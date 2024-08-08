// length_table is a serialized data structure that stores the length of each serialized object, along with the serialized object itself

// Serialized format:

// u32 length_table_length: 4 bytes, length of the length table in bytes
// u32 objects_count: 4 bytes, number of objects in the length table
// [u32; objects_count] object_lengths: stores the length of each object in bytes
// [u8; length_table_length] objects: length_table_length bytes, all the objects serialized

// To deserialize, first read the length_table_length, then read the objects_count, then read the object_lengths, then read the objects
// Then, deserialize each object using the object_lengths

// length_table should extend Vec<T> using the Serializable<T> trait

use crate::errors::Error;
use crate::traits::Serializable;
use std::convert::TryInto;

pub struct LengthTable;

impl LengthTable {
    pub fn serialize<T: Serializable<T>>(objects: &Vec<T>) -> Vec<u8> {
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

    pub fn deserialize<T: Serializable<T>>(bytes: &[u8]) -> Result<Vec<T>, Error> {
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
