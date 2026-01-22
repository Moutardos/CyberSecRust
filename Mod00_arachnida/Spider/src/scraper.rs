use itertools::Itertools;
use minreq::URL;
use regex::Regex;
use url::Url;

use std::{cell::{LazyCell, RefCell}, collections::HashMap, error, fs::File, io::Write, path::PathBuf, sync::{LazyLock, Mutex}};

static IMG_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"["'(]([^"\s]+/([^"\s]+\.(?:png|jpe?g|gif|bmp)))"#).unwrap());

thread_local! {
    pub static URLS_DOWNLOADED: RefCell<Vec<String>> =
    RefCell::new(Vec::new());

}
pub struct Scraper {}

impl Scraper {
    pub fn get_imgs<'a>(
        base_url: &'a Url,
        body: &'a str,
        path: &'a str,
    ) -> impl Iterator<Item = (Vec<u8>, PathBuf)> + 'a {
        IMG_PATTERN.captures_iter(body)
            .map(move |capture| capture.extract())
            .filter_map(move |(_, [url, name])| {
                let check_full_url = Scraper::add_base_url(base_url, url);
                match check_full_url {
                    Ok(full_url) => {
                        let mut full_path = PathBuf::from(path);
                        full_path.push(name);
                        Some((full_url, full_path))
                    } Err(err) => { eprintln!("{}", err); None }
                }
            })
            .filter_map(move |(url, path)| {
                URLS_DOWNLOADED.with_borrow_mut(|urls_downloaded|{
                if urls_downloaded.contains(&url) {
                    None
                }
                else{
                    match Scraper::get_bytes_from_url(&url) {
                        Ok(bytes) => {
                            urls_downloaded.push(url.clone());
                            println!(" Got {}", url);
                            Some((bytes, path))
                        }
                        Err(err) => {
                            println!(" Error while getting the img {}: {}", url, err);
                            None
                        }
                    }
                }}
                )})
        }

    pub fn add_base_url(base_url: &Url, url: &str) -> Result<String, Box<dyn error::Error>> {
        let url_check = Url::parse(url)?;
        if url_check.cannot_be_a_base() {
            Ok(base_url.join(url)?.to_string())
        } else {
            Ok(url_check.to_string())
        }
    }
    pub fn get_bytes_from_url(url: &URL) -> Result<Vec<u8>, Box<dyn error::Error>> {
        let response = minreq::get(url).send()?;
        let result = response.as_bytes();
        Ok(Vec::from(result))
    }
    pub fn get_body_from_url(url: &URL) -> Result<String, Box<dyn error::Error>> {
        let response = minreq::get(url).send()?;
        let result = response.as_str()?;
        Ok(result.to_string())
    }
}
