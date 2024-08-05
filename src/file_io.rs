use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::Path;

pub enum Endianness {
    Big,
    Little,
}

pub enum StringEncoding {
    Utf8,
    Ascii,
}

pub struct FileIO {
    file: File,
    buffer: Vec<u8>,
    path: String,
}

impl FileIO {
    pub fn new<P: AsRef<Path>>(path: P) -> io::Result<FileIO> {
        let file = File::options()
            .read(true)
            .write(true)
            .create(true)
            .open(&path)?;
        Ok(FileIO {
            file,
            buffer: Vec::new(),
            path: path.as_ref().to_string_lossy().to_string(),
        })
    }

    pub fn write_u32(&mut self, value: u32, endianness: Endianness) {
        let bytes = match endianness {
            Endianness::Big => value.to_be_bytes(),
            Endianness::Little => value.to_le_bytes(),
        };
        self.buffer.extend_from_slice(&bytes);
    }

    pub fn write_u16(&mut self, value: u16, endianness: Endianness) {
        let bytes = match endianness {
            Endianness::Big => value.to_be_bytes(),
            Endianness::Little => value.to_le_bytes(),
        };
        self.buffer.extend_from_slice(&bytes);
    }

    pub fn write_u64(&mut self, value: u64, endianness: Endianness) {
        let bytes = match endianness {
            Endianness::Big => value.to_be_bytes(),
            Endianness::Little => value.to_le_bytes(),
        };
        self.buffer.extend_from_slice(&bytes);
    }

    pub fn read_u32(&mut self, endianness: Endianness) -> io::Result<u32> {
        let mut bytes = [0; 4];
        self.file.read_exact(&mut bytes)?;
        Ok(match endianness {
            Endianness::Big => u32::from_be_bytes(bytes),
            Endianness::Little => u32::from_le_bytes(bytes),
        })
    }

    pub fn read_u64(&mut self, endianness: Endianness) -> io::Result<u64> {
        let mut bytes = [0; 8];
        self.file.read_exact(&mut bytes)?;
        Ok(match endianness {
            Endianness::Big => u64::from_be_bytes(bytes),
            Endianness::Little => u64::from_le_bytes(bytes),
        })
    }

    pub fn read_u16(&mut self, endianness: Endianness) -> io::Result<u16> {
        let mut bytes = [0; 2];
        self.file.read_exact(&mut bytes)?;
        Ok(match endianness {
            Endianness::Big => u16::from_be_bytes(bytes),
            Endianness::Little => u16::from_le_bytes(bytes),
        })
    }

    pub fn write_string(&mut self, value: &str, encoding: StringEncoding) -> io::Result<()> {
        match encoding {
            StringEncoding::Utf8 => self.buffer.extend_from_slice(value.as_bytes()),
            StringEncoding::Ascii => {
                if !value.is_ascii() {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "String contains non-ASCII characters",
                    ));
                }
                self.buffer.extend(value.bytes());
            }
        }
        Ok(())
    }

    pub fn read_string(&mut self, size: usize, encoding: StringEncoding) -> io::Result<String> {
        let mut buffer = vec![0; size];
        self.file.read_exact(&mut buffer)?;
        match encoding {
            StringEncoding::Utf8 => {
                String::from_utf8(buffer).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
            }
            StringEncoding::Ascii => Ok(String::from_utf8_lossy(&buffer).to_string()),
        }
    }

    pub fn flush(&mut self) -> io::Result<()> {
        self.file.write_all(&self.buffer)?;
        self.file.sync_all()?;
        self.buffer.clear();
        Ok(())
    }

    pub fn seek(&mut self, pos: u64) -> io::Result<u64> {
        self.file.seek(SeekFrom::Start(pos))
    }
}
