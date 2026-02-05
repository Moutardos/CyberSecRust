use std::{error, io::Error};

use nom::number::Endianness;

const LE_VALUE: u16 = 0x4949;
const BE_VALUE: u16 = 0x4D4D;

// macro_rules! write_bytes {
//     ($buf:expr, $offset:expr, $value:expr) => {{
//         let value_bytes = $value;
//         $buf[$offset..$offset + value_bytes.len()].copy_from_slice(&value_bytes);
//         $offset += value_bytes.len();
//     }};
// }
#[derive(Debug)]
pub struct ExifData {
    endian: Endianness,
    header: ExifHeader,
    ifds: Vec<ExifIFD>,
}

#[derive(Default, Debug)]
pub struct ExifHeader {
    byte_order: u16,
    version: u16,
    ifd_offset: u32,
}
#[derive(Debug)]
pub struct ExifIFD {
    entry_count: u16,
    entries: Vec<ExifField>,
    offset: u32,
}
#[derive(Debug)]
pub struct ExifField {
    tag: u16,
    etype: u16,
    length: u32,
    value: u32,
}

pub fn endian_byte(bytes: u16) -> Result<Endianness, Box<dyn error::Error>> {
    match bytes {
        LE_VALUE => Ok(Endianness::Little),
        BE_VALUE => Ok(Endianness::Big),
        _ => Err("Couldn't find byte order")?,
    }
}

pub mod exif_parser;
pub mod exif_reader;
