use crate::spider::Spider;
use clap::Parser;
use clap::{self, arg, Arg, Command};
use minreq::URL;
use std::path::PathBuf;
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
    let mut spider: Spider = Spider::new(&cliargs.url, &cliargs.path, len);
    spider.start();
    spider.download_all();
}
