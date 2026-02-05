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
        while next_ifd != 0 {
            let (_, ifd) = ExifIFD::parse_ifd(&raw_data[next_ifd..], &endian)?;
            next_ifd = ifd.offset as usize;
            ifds.push(ifd);
        }
        Ok((
            input,
            ExifData {
                endian,
                header,
                ifds,
            },
        ))
    }
}

impl ExifHeader {
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
}

impl ExifField {

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
}
