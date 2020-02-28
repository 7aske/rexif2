use crate::endianness::{Endianness, EndiannessError};
use byteorder as byte;
use byteorder::ByteOrder;
use core::fmt;
use std::error::Error;
use std::error;
use crate::contants::{TIFF_II, TIFF_MM};

pub struct Header {
    pub identifier: u16,
    pub version: u16,
    pub ifd_offset: u32,
}


impl Header {
    pub fn new(buf: &[u8; 8], endianness: Endianness) -> Self {
        let identifier: u16;
        let version: u16;
        let ifd_offset: u32;

        if endianness == Endianness::BigEndian {
            identifier = byte::BigEndian::read_u16(&buf[..2]);
            version = byte::BigEndian::read_u16(&buf[2..4]);
            ifd_offset = byte::BigEndian::read_u32(&buf[4..8]);
        } else {
            identifier = byte::LittleEndian::read_u16(&buf[..2]);
            version = byte::LittleEndian::read_u16(&buf[2..4]);
            ifd_offset = byte::LittleEndian::read_u32(&buf[4..8]);
        }

        Header {
            identifier,
            version,
            ifd_offset,
        }
    }
    pub fn endianness(buffer: &[u8]) -> Result<Endianness, EndiannessError> {
        return if TIFF_MM.iter().zip(buffer.iter()).all(|(a, b)| a == b) {
            Ok(Endianness::BigEndian)
        } else if TIFF_II.iter().zip(buffer.iter()).all(|(a, b)| a == b) {
            Ok(Endianness::LittleEndian)
        } else {
            Err(EndiannessError)
        };
    }
}

impl fmt::Debug for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
//        write!(f, "{:?}{:?}{:#8}", self.identifier.to_be_bytes(), self.version.to_be_bytes(), self.ifd_offset)
        write!(f, "{:?}{:?}{:#8}", self.identifier.to_le_bytes(), self.version.to_le_bytes(), self.ifd_offset)
    }
}
