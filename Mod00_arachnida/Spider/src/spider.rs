use minreq::URL;
use regex::Regex;
use std::io::Write;
use std::{collections::HashMap, error, sync::LazyLock};
use std::fs::File;

static IMG_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"[^"]+/([^"]+\.(?:png|jpe?g|gif|bmp))"#).unwrap());

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Spider {
    base_url: URL,
    path: String,
    recursive: u64,
    urls_done: HashMap<URL, String>,
}

impl Spider {
    pub fn new(base_url: String, path: String, recursive: u64) -> Self {
        Spider {
            base_url,
            path,
            recursive,
            urls_done: HashMap::new(),
        }
    }

    pub fn start(&mut self) {
        self.fill_url(&self.base_url.clone());
        self.urls_done
            .values()
            .for_each(|body| self.download_imgs(body));
    }

    /// Fill the HashMap, attributing a body to the URL
    fn fill_url(&mut self, url: &URL) {
        if self.urls_done.contains_key(url) {
            return;
        }

        let body = self
            .get_body_from_url(url)
            .inspect_err(|e| eprintln!("Error for \"{}\": {e}", &self.base_url))
            .unwrap_or_default();

        self.urls_done.insert(url.clone(), body);
    }

    /// Download all the img collected by regex from body into path
    pub fn download_imgs(&self, body: &str) {
        IMG_PATTERN
            .captures_iter(body)
            .map(|capture| capture.extract())
            .filter_map(|(url, [name])| {
                let full_url = if !url.contains(&self.base_url) {
                    format!("{}{}", self.base_url, url)
                } else {
                    url.to_string()
                };
                let full_path = format!("{}/{}", self.path, name);
                match self.get_bytes_from_url(&full_url) {
                    Ok(bytes) =>
                    {
                        println!("Got {}", full_url);
                        Some((bytes, full_path))
                    },
                    Err(err) =>
                    {
                        println!("Couldn't get {}: {}", full_url, err);
                        None
                    }
                }
            })
            .for_each(|(bytes, path)| {
                match self.create_img(bytes, &path) {
                    Ok(_) => { println!("{} Downloaded", path) },
                    Err(err) => { println!("{} Failed: {}", path, err) },
                }
            });
    }
    
    fn create_img(&self, bytes: Vec<u8>, filename: &String) -> std::io::Result<()> {
        let mut img = File::create(filename)?;
        img.write_all(bytes.as_slice())?;
        img.sync_data()?;
        Ok(())
    }
    fn get_bytes_from_url(&self, url: &URL) -> Result<Vec<u8>, Box<dyn error::Error>> {
        let response = minreq::get(url).send()?;
        let result = response.as_bytes();
        Ok(Vec::from(result))
    }
    fn get_body_from_url(&self, url: &URL) -> Result<String, Box<dyn error::Error>> {
        let response = minreq::get(url).send()?;
        let result = response.as_str()?;
        Ok(result.to_string())
    }
}

// let response = minreq::get(url).send()?;
// response.as_str().map_err(|e| {
//     eprintln!("Error for \"{}\" : {e}", &self.base_url);
// })
