use minreq::URL;
use regex::Regex;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::{collections::HashMap};

use crate::scraper::Scraper;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Spider {
    base_url: URL,
    path: String,
    recursive: u8,
    images_to_download: HashMap<Vec<u8>, PathBuf>,
    url_pattern: Regex,
}

impl Spider {
    pub fn new(base_url: &URL, path: &str, recursive: u8) -> Self {
        let pattern = format!(r#"href=["']({}.+?)['"]"#, regex::escape(base_url));
        Spider {
            base_url: base_url.clone(),
            path: path.to_string(),
            recursive,
            images_to_download: HashMap::new(),
            url_pattern: Regex::new(&pattern).unwrap(),
        }
    }

    pub fn start(&mut self) {
        let mut urls_done: HashMap<URL, String> = HashMap::new();
        self.start_filling_url_bodies(&mut urls_done, &self.base_url.clone());
        println!("urls got: {:?}", urls_done.keys());
        urls_done.values().for_each(|body| {
            self.images_to_download
                .extend(Scraper::get_imgs(&self.base_url, body, &self.path))
        });
    }

    fn create_img(bytes: &Vec<u8>, filename: &PathBuf) -> std::io::Result<()> {
        let mut img = File::create_new(filename)?;
        img.write_all(bytes.as_slice())?;
        img.sync_data()?;
        Ok(())
    }
    pub fn download_all(self) {
        for (bytes, path) in self.images_to_download {
            match Spider::create_img(&bytes, &path) {
                Ok(_) => {
                    println!("{} Downloaded", path.display())
                }
                Err(err) => {
                    println!("{} Failed: {}", path.display(), err)
                }
            };
        }
    }
    fn start_filling_url_bodies(&mut self, urls_done: &mut HashMap<URL, String>, url: &URL) {
        self.filling_url_bodies(urls_done, url, 0);
    }

    /// Fill the HashMap, attributing a body to the URL
    fn filling_url_bodies(&mut self, urls_done: &mut HashMap<URL, String>, url: &URL, depth: u8) {
        if depth > self.recursive || urls_done.contains_key(url) {
            return;
        }

        let body = Scraper::get_body_from_url(url)
            .inspect_err(|e| eprintln!("Error getting body from \"{}\": {e}", &self.base_url))
            .unwrap_or_default();

        urls_done.insert(url.clone(), body.clone());
        self.url_pattern
            .clone()
            .captures_iter(&body)
            .map(|capture| capture.extract())
            .for_each(|(_, [link])| {
                self.filling_url_bodies(urls_done, &link.to_string(), depth + 1);
            });
    }
}

// let response = minreq::get(url).send()?;
// response.as_str().map_err(|e| {
//     eprintln!("Error for \"{}\" : {e}", &self.base_url);
// })
