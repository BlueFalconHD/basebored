
# Basebored
Custom database library and serialization format built in Rust.

## Overview
~~As of now, databases can only be created and manipulated through the API. The database is stored in memory and can be serialized to and from a file. In the future, the database will be able to be queried and manipulated through a CLI. The database is designed to be simple and efficient, using a combination of custom serialization for database components and a length-prefixed encoding scheme for collections.~~
Currently, the project is being refactored so the API is more friendly to work with. Feel free to contribute. There is also a CLI crate in the works that will use the new API to allow manipulation of the database through a command line interface.

The database consists of several components:
- **Database**: The top-level structure containing sheets.
- **Sheet**: A collection of columns. Similar to a table in a relational database.
- **Column**: A collection of cells. Similar to a column in a relational database.
- **Cell**: Holds a value and a reference to its column.
- **Value**: Represents different data types (Bool, Int, Flt, Str, Nil).

## Serialization Format
See [SPEC.md](/docs/SPEC.md) for the detailed serialization format.

## Usage
~~There is currently no cargo package for this project, nor is the project set up to be used as a dependency. As of right now the best way to play around with the project is to clone the repository and run the code using `cargo run`.~~
Currently, there is nothing to run, the project is being refactored so ~50% of the outward facing API is unimplemented. Therefore, the best way to test the serialization format is to use the commit before the refactor or wait until the refactor is complete.
Once the refactor is complete, the project will be published to crates.io and can be used as a dependency in other projects.

## License
MIT, TODO: Add license file
