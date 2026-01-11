use std::{collections::HashMap, error};

use minreq::URL;

macro_rules! get_body {
    ($spider: ident, $url: expr ) => {
        $spider.get_body($url).unwrap_or_else ( |e| {

        }
    }
}

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
            urls_done: vec![],
        }
    }

    pub fn start(&self) {
        let url = &(self.base_url);

        println!("{}", base_url_body);
    }

    fn fill_url(&mut self, url: URL)
    {
        // Check entry
        self.urls_done.contains_key()
        let url_body = self.get_body_result(url).unwrap_or_else(|e| {
            eprintln!("Error for \"{}\" : {e}", &self.base_url);
            "".to_string()
        });
    }

    fn get_body_result(&self, url: &URL) -> Result<String, Box<dyn error::Error>> {
        let response = minreq::get(url).send()?;
        let result = response.as_str()?;
        Ok(result.to_string())
    }
}

// let response = minreq::get(url).send()?;
// response.as_str().map_err(|e| {
//     eprintln!("Error for \"{}\" : {e}", &self.base_url);
// })

