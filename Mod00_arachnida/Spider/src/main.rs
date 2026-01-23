#![warn(clippy::unwrap_used, clippy::all)]

use crate::spider::Spider;
use clap::Parser;
use clap::{self};
use minreq::URL;
use url::Url;
mod scraper;
mod spider;

#[derive(Parser)]
struct CliArgs {
    url: URL,
    #[clap(long, short)]
    recursive: bool,
    #[clap(long, short, default_value = "1")]
    length: u8,
    #[clap(long, short, default_value = "./data")]
    path: String,
}

fn main() {
    let cliargs = CliArgs::parse();
    let len: u8 = if cliargs.recursive { cliargs.length } else { 0 };
    let base_url = Url::parse(&cliargs.url);
    match base_url {
        Ok(url) => {
            let mut spider: Spider = Spider::new(&url, &cliargs.path, len);
            spider.start();
            spider.download_all();
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}
