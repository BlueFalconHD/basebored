# Basebored Serialization Specification

This document outlines the serialization format for the `Basebored` database. The format is designed to be simple and efficient, using a combination of custom serialization for database components and a length-prefixed encoding scheme for collections.

## Overview

The database consists of several components:
- **Database**: The top-level structure containing sheets.
- **Sheet**: A collection of columns.
- **Column**: A collection of cells.
- **Cell**: Holds a value and a reference to its column.
- **Value**: Represents different data types (Bool, Int, Flt, Str, Nil).

### Serialization Format

Each component implements the `Serializable<T>` trait, which defines methods for serializing to and deserializing from bytes.

### General Format

- All numbers are encoded in Big Endian (BE) format.
- Strings are encoded as length-prefixed UTF-8 byte arrays.
- Each component starts with a type identifier, followed by its serialized data.

## Detailed Format

### Magic Bytes

| Type         | Size (bytes) | Description                      |
|--------------|--------------|----------------------------------|
| `u8[11]`     | 11           | Magic bytes: "baseboredv1"       |

### Database

| Type          | Size (bytes)       | Description                           |
|---------------|--------------------|---------------------------------------|
| `u8[11]`      | 11                 | Magic bytes                           |
| `LengthTable` | Variable           | Serialized sheets                     |

### Sheet

| Type            | Size (bytes)       | Description                           |
|-----------------|--------------------|---------------------------------------|
| `u128`          | 16                 | UUID of the sheet                     |
| `u32`           | 4                  | Length of the name                    |
| `u8[]`          | Variable           | Name of the sheet                     |
| `LengthTable`   | Variable           | Serialized columns                    |

### Column

| Type            | Size (bytes)       | Description                           |
|-----------------|--------------------|---------------------------------------|
| `u128`          | 16                 | UUID of the column                    |
| `u32`           | 4                  | Length of the name                    |
| `u8[]`          | Variable           | Name of the column                    |
| `u8`            | 1                  | Value type of the column              |
| `LengthTable`   | Variable           | Serialized cells                      |

### Cell

| Type            | Size (bytes)       | Description                           |
|-----------------|--------------------|---------------------------------------|
| `u8[]`          | Variable           | Serialized value                      |

### Value

| Type            | Size (bytes)       | Description                           |
|-----------------|--------------------|---------------------------------------|
| `u8`            | 1                  | Value type identifier                 |
| `u8[]`          | Variable           | Serialized data                       |

#### Value Types

- `Bool` (Type::Bool)
  | Type            | Size (bytes)       | Description                           |
  |-----------------|--------------------|---------------------------------------|
  | `u8`            | 1                  | 0 (false) or 1 (true)                 |

- `Int` (Type::Int)
  | Type            | Size (bytes)       | Description                           |
  |-----------------|--------------------|---------------------------------------|
  | `i64`           | 8                  | 64-bit signed integer                 |

- `Flt` (Type::Flt)
  | Type            | Size (bytes)       | Description                           |
  |-----------------|--------------------|---------------------------------------|
  | `f64`           | 8                  | 64-bit floating-point number          |

- `Str` (Type::Str)
  | Type            | Size (bytes)       | Description                           |
  |-----------------|--------------------|---------------------------------------|
  | `u32`           | 4                  | Length of the string                  |
  | `u8[]`          | Variable           | UTF-8 encoded string                  |

- `Nil` (Type::Unknown)
  | Type            | Size (bytes)       | Description                           |
  |-----------------|--------------------|---------------------------------------|
  | N/A             | 0                  | Represents a nil value                |

### LengthTable

A length table is used to serialize collections of objects, encoding the length of each object followed by its serialized bytes.

| Type            | Size (bytes)       | Description                           |
|-----------------|--------------------|---------------------------------------|
| `u32`           | 4                  | Length of the length table            |
| `u32`           | 4                  | Number of objects                     |
| `u32[]`         | 4 * objects_count  | Lengths of each object                |
| `u8[]`          | Variable           | Serialized objects                    |
