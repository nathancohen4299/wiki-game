use crate::page::Page;

mod page;

fn main() {
    let base = "https://en.wikipedia.org";

    let url = "/wiki/Make_Your_Wish";

    let source = Page::new(format!("{}{}", base, url).as_str());

    
}
