use minreq::URL;
use regex::Regex;

use crate::PathBuf;

use std::{collections::HashMap, error, fs::File, io::Write, sync::LazyLock};

static IMG_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"[^"\s]+/([^"\s]+\.(?:png|jpe?g|gif|bmp))"#).unwrap());

pub struct Scraper {}

impl Scraper {
    pub fn get_imgs<'a>(
        base_url: &'a str,
        body: &'a str,
        path: &'a str,
    ) -> impl Iterator<Item = (Vec<u8>, PathBuf)> + 'a {
        IMG_PATTERN
            .captures_iter(body)
            .map(move |capture| capture.extract())
            .filter_map(move |(url, [name])| {
                println!("url:{}", url);
                let full_url =  /* if url.starts_with('.') || url.starts_with('/') { */
                //     println!("new: {}{}", base_url, url);
                //     format!("{}{}", base_url, url)
                // } else {
                    url.to_string();
                // };
                let mut full_path = PathBuf::from(path);
                full_path.push(name);

                match Scraper::get_bytes_from_url(&full_url) {
                    Ok(bytes) => {
                        println!("Got {}", full_url);
                        Some((bytes, full_path))
                    }
                    Err(err) => {
                        println!("Error while getting the img {}: {}", full_url, err);
                        None
                    }
                }
            })
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
