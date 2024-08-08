
# Basebored
Custom database library and serialization format built in Rust.

## Overview
As of now, databases can only be created and manipulated through the API. The database is stored in memory and can be serialized to and from a file. In the future, the database will be able to be queried and manipulated through a CLI. The database is designed to be simple and efficient, using a combination of custom serialization for database components and a length-prefixed encoding scheme for collections.

The database consists of several components:
- **Database**: The top-level structure containing sheets.
- **Sheet**: A collection of columns. Similar to a table in a relational database.
- **Column**: A collection of cells. Similar to a column in a relational database.
- **Cell**: Holds a value and a reference to its column.
- **Value**: Represents different data types (Bool, Int, Flt, Str, Nil).

## Serialization Format
See [SPEC.md](/docs/SPEC.md) for the detailed serialization format.

## Usage
There is currently no cargo package for this project, nor is the project set up to be used as a dependency. As of right now the best way to play around with the project is to clone the repository and run the code using `cargo run`.

## License
MIT, TODO: Add license file
