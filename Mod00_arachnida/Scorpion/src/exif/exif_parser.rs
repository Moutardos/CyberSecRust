use nom::combinator::verify;
use nom::multi::count;
use nom::number::Endianness;
use nom::number::complete::{be_u16, u16, u32};
use nom::{IResult, Parser};
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use crate::exif::{BE_VALUE, ExifData, ExifField, ExifHeader, ExifIFD, LE_VALUE};
use crate::exif::{endian_byte};
impl ExifData {
    pub fn parse_from_raw(raw_data: &[u8]) -> IResult<&[u8], ExifData> {
        let (input, header) = ExifHeader::parse_header(raw_data)?;
        let endian = header.endianness();
        let mut next_ifd = header.ifd_offset as usize;
        let mut ifds: Vec<ExifIFD> = vec![];
        let mut size: usize = ExifHeader::TOTAL_SIZE;
        while next_ifd != 0 {
            let (_, ifd) = ExifIFD::parse_ifd(&raw_data[next_ifd..], &endian)?;
            next_ifd = ifd.offset as usize;
            size += ifd.entries.len() * ExifField::TOTAL_SIZE + size_of::<u16>() + size_of::<u32>();
            ifds.push(ifd);
        }
        Ok((
            input,
            ExifData {
                size,
                endian,
                header,
                ifds,
            },
        ))
    }
}

impl ExifHeader {
    const TOTAL_SIZE: usize =
        size_of::<u16>() + size_of::<u16>() + size_of::<u16>() + size_of::<u32>();
    fn parse_header(input: &[u8]) -> IResult<&[u8], ExifHeader> {
        let (input, byte_order) = verify(be_u16, |value| *value == LE_VALUE || *value == BE_VALUE).parse(input)?;
        let endian = endian_byte(byte_order).expect("Shouldn't happen, value is pre-validated"); 
        let parse_16 = &u16(endian);
        let parse_32 = &u32(endian);
        let (input, (version, ifd_offset)) = (parse_16, parse_32).parse(input)?;
        Ok((
            input,
            ExifHeader {
                byte_order,
                version,
                ifd_offset,
            },
        ))
    }

    pub fn endianness(&self) -> Endianness {
        endian_byte(self.byte_order).expect("Shouldn't happen, value is prevalidated")
    }
    // fn to_bytes(&self, endian: Endianness) -> [u8; Self::TOTAL_SIZE] {
    //     let mut bytes = [0; Self::TOTAL_SIZE];
    //     let mut offset = 0;
    //     match endian {
    //         Endianness::Big => {
    //             write_bytes!(bytes, offset, self.byte_order.to_be_bytes());
    //             write_bytes!(bytes, offset, self.version.to_be_bytes());
    //             write_bytes!(bytes, offset, self.ifd_offset.to_be_bytes());
    //         }
    //         Endianness::Little => {
    //             write_bytes!(bytes, offset, self.byte_order.to_le_bytes());
    //             write_bytes!(bytes, offset, self.version.to_le_bytes());
    //             write_bytes!(bytes, offset, self.ifd_offset.to_le_bytes());
    //         }
    //         _ => todo!(),
    //     };
    //     bytes
    // }
}

impl ExifIFD {
    fn parse_ifd<'a>(raw_data: &'a [u8], endian: &Endianness) -> IResult<&'a [u8], ExifIFD> {
        let parse_16 = &u16(*endian);
        let parse_32 = &u32(*endian);

        let (input, entry_count) = parse_16(raw_data)?;

        let (input, (entries, offset)) = (
            count(
                move |i| ExifField::parse_field(i, endian),
                entry_count as usize,
            ),
            parse_32,
        )
            .parse(input)?;

        Ok((
            input,
            ExifIFD {
                entry_count,
                entries,
                offset,
            },
        ))
    }
    // pub fn to_bytes(&self, endian: Endianness) -> Vec<u8> {
    //     let mut bytes = Vec::new();
    //
    //     match endian {
    //         Endianness::Little => bytes.extend_from_slice(&self.entry_count.to_le_bytes()),
    //         Endianness::Big => bytes.extend_from_slice(&self.entry_count.to_be_bytes()),
    //         _ => todo!(),
    //     }
    //
    //     for entry in &self.entries {
    //         bytes.extend_from_slice(&entry.to_bytes(endian));
    //     }
    //
    //     match endian {
    //         Endianness::Little => bytes.extend_from_slice(&self.offset.to_le_bytes()),
    //         Endianness::Big => bytes.extend_from_slice(&self.entry_count.to_be_bytes()),
    //         _ => todo!(),
    //     }
    //
    //     bytes
    // }
}

impl ExifField {
    const TOTAL_SIZE: usize =
        size_of::<u16>() + size_of::<u16>() + size_of::<u32>() + size_of::<u32>();

    fn parse_field<'a>(raw_data: &'a [u8], endian: &Endianness) -> IResult<&'a [u8], ExifField> {
        let parse_16 = &u16(*endian);
        let parse_32 = &u32(*endian);

        let (input, (tag, etype, length, value)) =
            (parse_16, parse_16, parse_32, parse_32).parse(raw_data)?;
        Ok((
            input,
            ExifField {
                tag,
                etype,
                length,
                value,
            },
        ))
    }
    // fn to_bytes(&self, endian: Endianness) -> [u8; Self::TOTAL_SIZE] {
    //     let mut bytes = [0; Self::TOTAL_SIZE];
    //     let mut offset = 0;
    //     match endian {
    //         Endianness::Big => {
    //             write_bytes!(bytes, offset, self.tag.to_be_bytes());
    //             write_bytes!(bytes, offset, self.etype.to_be_bytes());
    //             write_bytes!(bytes, offset, self.length.to_be_bytes());
    //             write_bytes!(bytes, offset, self.value.to_be_bytes());
    //         }
    //         Endianness::Little => {
    //             write_bytes!(bytes, offset, self.tag.to_le_bytes());
    //             write_bytes!(bytes, offset, self.etype.to_le_bytes());
    //             write_bytes!(bytes, offset, self.length.to_le_bytes());
    //             write_bytes!(bytes, offset, self.value.to_le_bytes());
    //         }
    //         _ => todo!(),
    //     };
    //     bytes
    // }
}
