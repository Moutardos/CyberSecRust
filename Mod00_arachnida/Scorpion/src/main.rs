#![warn(clippy::unwrap_used, clippy::all)]
mod exif_parser;

use crate::exif_parser::ExifData;
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
            let reader = ExifData::parse_from_raw(data.as_slice());
             
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}
