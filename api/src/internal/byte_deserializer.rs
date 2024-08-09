use crate::internal::errors::Error;

#[derive(Debug, Clone)]
pub(crate) struct ByteDeserializer<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl ByteDeserializer<'_> {
    pub(crate) fn new(bytes: &[u8]) -> ByteDeserializer {
        ByteDeserializer { bytes, pos: 0 }
    }

    pub(crate) fn read_u8(&mut self) -> Result<u8, Error> {
        if self.pos + 1 > self.bytes.len() {
            return Err(Error::ByteError(ByteError::OutOfBoundsError {
                pos: self.pos,
                len: self.bytes.len(),
            }));
        }

        let value = self.bytes[self.pos];
        self.pos += 1;
        Ok(value)
    }

    pub(crate) fn read_u16(&mut self) -> Result<u16, Error> {
        if self.pos + 2 > self.bytes.len() {
            return Err(Error::ByteError(ByteError::OutOfBoundsError {
                pos: self.pos,
                len: self.bytes.len(),
            }));
        }

        let value = u16::from_be_bytes([self.bytes[self.pos], self.bytes[self.pos + 1]]);
        self.pos += 2;
        Ok(value)
    }

    pub(crate) fn read_u32(&mut self) -> Result<u32, Error> {
        if self.pos + 4 > self.bytes.len() {
            return Err(Error::ByteError(ByteError::OutOfBoundsError {
                pos: self.pos,
                len: self.bytes.len(),
            }));
        }

        let value = u32::from_be_bytes([
            self.bytes[self.pos],
            self.bytes[self.pos + 1],
            self.bytes[self.pos + 2],
            self.bytes[self.pos + 3],
        ]);
        self.pos += 4;
        Ok(value)
    }

    pub(crate) fn read_u64(&mut self) -> Result<u64, Error> {
        if self.pos + 8 > self.bytes.len() {
            return Err(Error::ByteError(ByteError::OutOfBoundsError {
                pos: self.pos,
                len: self.bytes.len(),
            }));
        }

        let value = u64::from_be_bytes([
            self.bytes[self.pos],
            self.bytes[self.pos + 1],
            self.bytes[self.pos + 2],
            self.bytes[self.pos + 3],
            self.bytes[self.pos + 4],
            self.bytes[self.pos + 5],
            self.bytes[self.pos + 6],
            self.bytes[self.pos + 7],
        ]);
        self.pos += 8;
        Ok(value)
    }

    pub(crate) fn read_i8(&mut self) -> Result<i8, Error> {
        if self.pos + 1 > self.bytes.len() {
            return Err(Error::ByteError(ByteError::OutOfBoundsError {
                pos: self.pos,
                len: self.bytes.len(),
            }));
        }

        let value = self.bytes[self.pos] as i8;
        self.pos += 1;
        Ok(value)
    }

    pub(crate) fn read_i16(&mut self) -> Result<i16, Error> {
        if self.pos + 2 > self.bytes.len() {
            return Err(Error::ByteError(ByteError::OutOfBoundsError {
                pos: self.pos,
                len: self.bytes.len(),
            }));
        }

        let value = i16::from_be_bytes([self.bytes[self.pos], self.bytes[self.pos + 1]]);
        self.pos += 2;
        Ok(value)
    }

    pub(crate) fn read_i32(&mut self) -> Result<i32, Error> {
        if self.pos + 4 > self.bytes.len() {
            return Err(Error::ByteError(ByteError::OutOfBoundsError {
                pos: self.pos,
                len: self.bytes.len(),
            }));
        }

        let value = i32::from_be_bytes([
            self.bytes[self.pos],
            self.bytes[self.pos + 1],
            self.bytes[self.pos + 2],
            self.bytes[self.pos + 3],
        ]);
        self.pos += 4;
        Ok(value)
    }

    pub(crate) fn read_i64(&mut self) -> Result<i64, Error> {
        if self.pos + 8 > self.bytes.len() {
            return Err(Error::ByteError(ByteError::OutOfBoundsError {
                pos: self.pos,
                len: self.bytes.len(),
            }));
        }

        let value = i64::from_be_bytes([
            self.bytes[self.pos],
            self.bytes[self.pos + 1],
            self.bytes[self.pos + 2],
            self.bytes[self.pos + 3],
            self.bytes[self.pos + 4],
            self.bytes[self.pos + 5],
            self.bytes[self.pos + 6],
            self.bytes[self.pos + 7],
        ]);
        self.pos += 8;
        Ok(value)
    }

    pub(crate) fn read_f32(&mut self) -> Result<f32, Error> {
        if self.pos + 4 > self.bytes.len() {
            return Err(Error::ByteError(ByteError::OutOfBoundsError {
                pos: self.pos,
                len: self.bytes.len(),
            }));
        }

        let value = f32::from_be_bytes([
            self.bytes[self.pos],
            self.bytes[self.pos + 1],
            self.bytes[self.pos + 2],
            self.bytes[self.pos + 3],
        ]);
        self.pos += 4;
        Ok(value)
    }

    pub(crate) fn read_f64(&mut self) -> Result<f64, Error> {
        if self.pos + 8 > self.bytes.len() {
            return Err(Error::ByteError(ByteError::OutOfBoundsError {
                pos: self.pos,
                len: self.bytes.len(),
            }));
        }

        let value = f64::from_be_bytes([
            self.bytes[self.pos],
            self.bytes[self.pos + 1],
            self.bytes[self.pos + 2],
            self.bytes[self.pos + 3],
            self.bytes[self.pos + 4],
            self.bytes[self.pos + 5],
            self.bytes[self.pos + 6],
            self.bytes[self.pos + 7],
        ]);
        self.pos += 8;
        Ok(value)
    }

    pub(crate) fn read_bool(&mut self) -> Result<bool, Error> {
        if self.pos + 1 > self.bytes.len() {
            return Err(Error::ByteError(ByteError::OutOfBoundsError {
                pos: self.pos,
                len: self.bytes.len(),
            }));
        }

        let value = self.bytes[self.pos];
        self.pos += 1;

        if value != 0 && value != 1 {
            return Err(Error::ByteError(ByteError::BadBoolError { value }));
        }

        Ok(value == 1)
    }

    pub(crate) fn read_string(&mut self, length: usize) -> Result<String, Error> {
        if self.pos + length > self.bytes.len() {
            return Err(Error::ByteError(ByteError::OutOfBoundsError {
                pos: self.pos,
                len: self.bytes.len(),
            }));
        }

        let value = String::from_utf8_lossy(&self.bytes[self.pos..self.pos + length]).to_string();
        self.pos += length;
        Ok(value)
    }

    pub(crate) fn read_bytes(&mut self, length: usize) -> Result<Vec<u8>, Error> {
        if self.pos + length > self.bytes.len() {
            return Err(Error::ByteError(ByteError::OutOfBoundsError {
                pos: self.pos,
                len: self.bytes.len(),
            }));
        }

        let value = self.bytes[self.pos..self.pos + length].to_vec();
        self.pos += length;
        Ok(value)
    }

    pub(crate) fn remaining_bytes(&self) -> &[u8] {
        &self.bytes[self.pos..]
    }
}

/* -- ERRORS -- */

#[derive(Debug)]
pub(crate) enum ByteError {
    OutOfBoundsError { pos: usize, len: usize },
    BadBoolError { value: u8 },
}

impl std::fmt::Display for ByteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ByteError::OutOfBoundsError { pos, len } => {
                write!(f, "Out of bounds error: pos {} out of len {}", pos, len)
            }

            ByteError::BadBoolError { value } => {
                write!(f, "Bad bool error: value {} is not 0 or 1", value)
            }
        }
    }
}

impl std::error::Error for ByteError {}
