use curl::easy::{Easy2, Handler, WriteError};
use scraper::{Html, Selector};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

struct Collector(Vec<u8>);
impl Handler for Collector {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.0.extend_from_slice(data);
        Ok(data.len())
    }
}

static BASE: &'static str = "https://en.wikipedia.org";

#[derive(Hash, Debug)]
pub struct Page {
    pub path: String,
//    pub links: Vec<String>,
}

impl Page {
    pub fn new(path: &str) -> Page {
        Page {
            path: String::from(path),
            //            links: vec!(),
        }
    }
    pub fn calculate_hash(&self) -> u64 {
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        s.finish()
    }
    pub fn get_urls(&self) -> Vec<String> {
        let mut easy = Easy2::new(Collector(Vec::new()));
        easy.get(true).unwrap();
        easy.url(format!("{}{}", BASE, self.path).as_str()).unwrap(); // Combine Base with path here
        easy.perform().unwrap();

        let contents = easy.get_ref();
        let html_contents = String::from_utf8_lossy(&contents.0);

        let fragment = Html::parse_fragment(html_contents.trim());
        let div_selector = Selector::parse("div").unwrap();
        let a_selector = Selector::parse("a").unwrap();

        let mut urls = Vec::new();

        for div in fragment.select(&div_selector) {
            let id = div.value().attr("id").unwrap_or("");
            if id == "mw-content-text" {
                for element in div.select(&a_selector) {
                    if let Some(path) = element.value().attr("href") {
                        if path.chars().next().unwrap() != '#'
                            && path.len() >= 6
                            && &path[..6] == "/wiki/"
                        {
                            urls.push(String::from(path));
                        }
                    }
                }
            }
        }
        urls
    }

}
