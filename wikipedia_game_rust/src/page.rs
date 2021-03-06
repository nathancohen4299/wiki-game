use curl::easy::{Easy2, Handler, WriteError};
use scraper::{Html, Selector};
struct Collector(Vec<u8>);
impl Handler for Collector {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.0.extend_from_slice(data);
        Ok(data.len())
    }
}

pub struct Page{
    pub path: String,
    pub links: Vec<String>
}

impl Page{
    pub fn new(path: &str)-> Page {
        Page{ path: String::from(path), links: get_urls(path) }
    }
}





fn get_urls(url: &str) -> Vec<String> {
    let mut easy = Easy2::new(Collector(Vec::new()));
    easy.get(true).unwrap();
    easy.url(url).unwrap();
    easy.perform().unwrap();

    let contents = easy.get_ref();
    let html_contents = String::from_utf8_lossy(&contents.0);

    let fragment = Html::parse_fragment(html_contents.trim());
    let div_selector = Selector::parse("div").unwrap();
    let a_selector = Selector::parse("a").unwrap();

    let mut urls: Vec<String> = Vec::new();


    for div in fragment.select(&div_selector) {
        let id = div.value().attr("id").unwrap_or("");
        println!("id = {}", id);
        if id == "mw-content-text" {
            for element in div.select(&a_selector) {
                let path = element.value().attr("href").unwrap();
                if path.chars().next().unwrap() != '#' && path.len() >= 6 &&
                    &path[..6] == "/wiki/"{
                    urls.push(format!("{}", path));
                }
            }
        }
    }
    urls
}