use std::collections::HashMap;

pub fn get_browser_rules<'a>() -> HashMap<&'a str, &'a str> {
    let mut browser_mapping = HashMap::new();

    browser_mapping.insert("https://google.com/", "Google Chrome");

    browser_mapping
}
