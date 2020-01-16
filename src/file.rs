use crate::endianness::{LittleEndian, BigEndian};
use crate::endianness::Endianness;
use crate::header::Header;
use std::fs::File;
use std::io::{Read, BufReader, SeekFrom, Seek};
use byteorder as byte;
use byteorder::ByteOrder;
use crate::ifd::Ifd;
use core::fmt;
use crate::tagtype::{TagType, TagTypeSize};

pub struct Tiff {
    pub header: Header,
    pub files: Vec<Ifd>,
    pub endianness: Endianness,
    offset: u64,
}


impl Tiff {
    pub fn new(header: Header, endianness: Endianness, offset: u64) -> Self {
        Tiff {
            header,
            files: vec![],
            endianness,
            offset,
        }
    }
    pub fn new_from_file(file: &mut File, offset: u64) -> Self {
        let mut f = BufReader::new(file);
        let mut buf: [u8; 8] = [0; 8];
        f.read(&mut buf);

        let end = if byte::BigEndian::read_u16(&buf[..2]) == 19789 { Endianness::BigEndian } else { Endianness::LittleEndian };
        let header = Header::new(&buf, end);
        let mut ifd = Ifd::new(&mut f, end.clone(), offset);
        let mut tiff = Tiff::new(header, end.clone(), offset);
        let mut next_ptr = ifd.next_ifd_ptr() as u64;
        while next_ptr != 0 {
            f.seek(SeekFrom::Start(next_ptr));
            tiff.files.push(ifd);
            ifd = Ifd::new(&mut f, end.clone(), offset);
            next_ptr = ifd.next_ifd_ptr() as u64;
        }
//        tiff.files.push(ifd);

        return tiff;
    }
//    pub fn print_tags(&self) {
//        for ifd in &self.files {
//            for tag in &ifd.tags {
//                if tag.tid.val() != 0 {
//                    let size = TagTypeSize::from(tag.ttype as u8).size();
//                    let mut buf;
//                    if size as u32 * tag.count > 4 {
//                        buf = vec![0u8; ((tag.count as usize) * size)];
//                        f.seek(SeekFrom::Start((tag.offset as u64) + self.offset)).expect(format!("cannot seek file to {}", tag.offset).as_str());
//                        f.read_exact(&mut buf);
//                    } else {
//                        buf = tag.offset.to_be_bytes().to_vec();
//                    }
//                    match tag.tid.type_size() {
//                        TagTypeSize::UNDEFINED => println!("{:?} {}", tag, String::from_utf8_lossy(&buf)),
//                        TagTypeSize::ASCII => println!("{:?} {}", tag, String::from_utf8_lossy(&buf)),
//
//                        TagTypeSize::BYTE => {
//                            println!("{:?} {:24}", tag, buf[0])
//                        }
//                        TagTypeSize::SHORT => {
//                            println!("{:?} {:?}", tag, byte::BigEndian::read_u16(&buf))
//                        }
//                        TagTypeSize::LONG => {
//                            println!("{:?} {:?}", tag, byte::BigEndian::read_u32(&buf))
//                        }
//                        TagTypeSize::SBYTE => {
//                            println!("{:?} {:24}", tag, buf[0])
//                        }
//                        TagTypeSize::SSHORT => {
//                            println!("{:?} {:24}", tag, byte::BigEndian::read_u16(&buf))
//                        }
//                        TagTypeSize::SLONG => {
//                            println!("{:?} {:24}", tag, byte::BigEndian::read_u32(&buf))
//                        }
//
//                        TagTypeSize::FLOAT => println!("{:?} {}", tag, String::from_utf8_lossy(&buf)),
//                        TagTypeSize::DOUBLE => println!("{:?} {}", tag, String::from_utf8_lossy(&buf)),
//                        TagTypeSize::RATIONAL => println!("{:?} {}", tag, String::from_utf8_lossy(&buf)),
//                        TagTypeSize::SRATIONAL => println!("{:?} {}", tag, String::from_utf8_lossy(&buf)),
//                    }
//                }
//            }
//        }
}

impl fmt::Debug for Tiff {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:?} Files: {:#?}", self.header, self.files)
    }
}
