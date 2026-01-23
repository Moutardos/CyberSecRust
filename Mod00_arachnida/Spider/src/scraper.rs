use itertools::Itertools;
use minreq::URL;
use regex::Regex;
use url::{ParseError, Url};

use std::{cell::RefCell, error, path::PathBuf, sync::LazyLock};

static IMG_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"([^"(\s]*/([^"(]+\.(?:png|jpe?g|gif|bmp)))"#).unwrap());

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
        IMG_PATTERN
            .captures_iter(body)
            .map(move |capture| capture.extract())
            .filter_map(move |(match_, [url, name])| {
                println!(
                    "regex result: match {}\ngroup1: {}\ngroup2: {}",
                    match_, url, name
                );
                let check_full_url = Scraper::add_base_url(base_url, url);
                match check_full_url {
                    Ok(full_url) => {
                        let mut full_path = PathBuf::from(path);
                        full_path.push(name);
                        Some((full_url, full_path))
                    }
                    Err(err) => {
                        eprintln!("err check {} {}", url, err);
                        None
                    }
                }
            })
            .filter_map(move |(url, path)| {
                URLS_DOWNLOADED.with_borrow_mut(|urls_downloaded| {
                    if urls_downloaded.contains(&url) {
                        None
                    } else {
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
                    }
                })
            })
    }

    pub fn add_base_url(base_url: &Url, url: &str) -> Result<String, Box<dyn error::Error>> {
        if url.starts_with("//") {
            Ok(Url::parse((String::from(base_url.scheme()) + ":" + url).as_str())?.to_string())
        } else if url.starts_with("/") {
            Ok((Url::parse(
                (String::from(base_url.scheme()) + "://" + base_url.host_str().unwrap_or_default())
                    .as_str(),
            )?
            .to_string()
                + &url[1..])
                .to_string())
        } else {
            let url_check = Url::parse(url)?;
            if url_check.cannot_be_a_base() {
                Ok(base_url.join(url)?.to_string())
            } else {
                Ok(url_check.to_string())
            }
        }
    }
    pub fn get_bytes_from_url(url: &URL) -> Result<Vec<u8>, Box<dyn error::Error>> {
        let response = minreq::get(url).send()?;
        if response.status_code >= 300 || response.as_str().is_ok() {
            Err("Bad request")?
        } else {
            let result = response.as_bytes();
            Ok(Vec::from(result))
        }
    }
    pub fn get_body_from_url(url: &URL) -> Result<String, Box<dyn error::Error>> {
        let response = minreq::get(url).send()?;
        let result = response.as_str()?;
        Ok(result.to_string())
    }
}
