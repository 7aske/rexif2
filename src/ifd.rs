use crate::endianness::Endianness;
use std::fs::File;
use std::io::{Read, BufReader};
use crate::endianness::{BigEndian, LittleEndian, Endian};
use crate::tag::Tag;
use core::fmt;
use crate::tagtype::TagType;
use byteorder::ByteOrder;

pub struct Ifd {
    num_dir_ent: u16,
    pub tags: Vec<Tag>,
    next_ifd: u32,
}

impl Ifd {
    pub fn new(f: &mut BufReader<&mut File>, endianness: Endianness, offset: u64) -> Self {
        let num_dir_ent: u16;
        let mut tags: Vec<Tag> = Vec::new();
        let next_ifd: u32;

        if endianness == Endianness::BigEndian {
            num_dir_ent = BigEndian::read_u16(f);
        } else {
            num_dir_ent = LittleEndian::read_u16(f);
        }

        let mut tag;
        for _ in 0..num_dir_ent {
            tag = Tag::new(f, endianness, offset);
            tags.push(tag);
        }

        if endianness == Endianness::BigEndian {
            next_ifd = BigEndian::read_u32(f);
        } else {
            next_ifd = LittleEndian::read_u32(f);
        }

        Ifd {
            num_dir_ent,
            tags,
            next_ifd,
        }
    }
    pub fn next_ifd_ptr(&self) -> u32 {
        let out: u32 = 0;
        for tag in &self.tags {
            if tag.tid == TagType::ExifIFDPointer {
                return byteorder::BigEndian::read_u32(&tag.data);
            }
        }
        out
    }
}


impl fmt::Debug for Ifd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:?}\nTags: {:#?}\n{}", self.num_dir_ent, self.tags, self.next_ifd)
    }
}
