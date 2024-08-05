mod file_io;
use file_io::{Endianness, FileIO, StringEncoding};

fn main() {
    let path = "example.bin";
    let mut file_io = match FileIO::new(path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to open file: {}", e);
            return;
        }
    };

    // Write various types of data to the file
    file_io.write_u32(0x12345678, Endianness::Big);
    file_io.write_u64(0x123456789ABCDEF0, Endianness::Little);
    file_io.write_u16(0xABCD, Endianness::Big);
    if let Err(e) = file_io.write_string("Hello, World!", StringEncoding::Utf8) {
        eprintln!("Failed to write string: {}", e);
        return;
    }

    // Flush buffer to file
    if let Err(e) = file_io.flush() {
        eprintln!("Failed to flush data: {}", e);
        return;
    }

    // Reset file pointer to the start to read back data
    if let Err(e) = file_io.seek(0) {
        eprintln!("Failed to seek in file: {}", e);
        return;
    }

    // Read back the data
    let value_u32 = file_io.read_u32(Endianness::Big).unwrap_or_else(|e| {
        eprintln!("Failed to read u32: {}", e);
        0
    });
    let value_u64 = file_io.read_u64(Endianness::Little).unwrap_or_else(|e| {
        eprintln!("Failed to read u64: {}", e);
        0
    });
    let value_u16 = file_io.read_u16(Endianness::Big).unwrap_or_else(|e| {
        eprintln!("Failed to read u16: {}", e);
        0
    });
    let value_string = file_io
        .read_string(13, StringEncoding::Utf8)
        .unwrap_or_else(|e| {
            eprintln!("Failed to read string: {}", e);
            String::from("Error")
        });

    // Display the read values
    println!("Read u32: 0x{:08X}", value_u32);
    println!("Read u64: 0x{:016X}", value_u64);
    println!("Read u16: 0x{:04X}", value_u16);
    println!("Read string: {}", value_string);
}
