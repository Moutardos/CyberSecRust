use minreq::URL;

use crate::spider::Spider;

mod spider;

fn main() {
    let url: URL = std::env::args().last().unwrap();
    let mut spider: Spider = Spider::new(url, "./data".into(), 0);
    spider.start();
}
