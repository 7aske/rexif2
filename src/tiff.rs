use crate::endianness::{LittleEndian, BigEndian, EndiannessError};
use crate::endianness::Endianness;

use crate::header::Header;
use std::fs::File;
use std::io::{Read, BufReader, SeekFrom, Seek};
use byteorder as byte;
use byteorder::ByteOrder;
use crate::ifd::Ifd;
use core::fmt;
use crate::tagtype::{TagType, TagTypeSize};
use std::process::exit;
use crate::header;

pub struct Tiff {
    pub header: Header,
    pub files: Vec<Ifd>,
    pub endianness: Endianness,
    offset: u64,
}

/// Creates a structure representing a TIFF file
impl Tiff {
    pub fn new(header: Header, endianness: Endianness, offset: u64) -> Self {
        Tiff {
            header,
            files: vec![],
            endianness,
            offset,
        }
    }
    pub fn new_from_file(file: &mut File, offset: u64) -> Result<Tiff, TiffError> {
        let mut file_reader = BufReader::new(file);
        let mut header_buffer: [u8; 8] = [0; 8];
        file_reader.read(&mut header_buffer);

        let endianness = Header::endianness(&header_buffer[..2])?;

        let header = Header::new(&header_buffer, endianness);
        let mut ifd = Ifd::new(&mut file_reader, endianness.clone(), offset);
        let mut tiff = Tiff::new(header, endianness.clone(), offset);
        let mut next_ptr = ifd.next_ifd_ptr() as u64;
        while next_ptr != 0 {
            file_reader.seek(SeekFrom::Start(next_ptr));
            tiff.files.push(ifd);
            ifd = Ifd::new(&mut file_reader, endianness.clone(), offset);
            next_ptr = ifd.next_ifd_ptr() as u64;
        }

        return Ok(tiff);
    }
}

impl fmt::Debug for Tiff {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:?} Files: {:#?}", self.header, self.files)
    }
}

#[derive(Debug, Clone)]
pub struct TiffError;


impl From<EndiannessError> for TiffError {
    fn from(err: EndiannessError) -> Self {
        TiffError
    }
}


