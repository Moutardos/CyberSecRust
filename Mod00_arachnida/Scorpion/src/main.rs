#![warn(clippy::unwrap_used, clippy::all)]
mod exif;

use crate::exif::ExifData;
use std::fs;
use std::fs::File;
use std::io::Read as _;

use clap::Parser;
use clap::{self};

#[derive(Parser)]
struct CliArgs {
    file: String,
}

fn main() {
    let cliargs = CliArgs::parse();
    let image_check = fs::read(cliargs.file);
    match image_check {
        Ok(mut data) => {
            let (data, exif) = ExifData::parse_from_raw(data.as_slice()).unwrap();
            println!("{exif:?}")
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}
