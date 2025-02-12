use std::collections::HashMap;
use std::{env, io};

use browser_mapping::get_browser_rules;
use browsers::get_browsers;
use link_processor::process_link;

mod browser_mapping;
mod browsers;
mod link_processor;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let url = if args.len() > 1 {
        args[1].clone()
    } else {
        String::new()
    };

    let browsers = get_browsers()?;

    let default_browser = "Mozilla Firefox";

    let mut browser_map = HashMap::new();

    for browser in browsers {
        browser_map.insert(browser.name, browser.path);
    }

    let browser_rules: HashMap<&str, &str> = get_browser_rules();

    process_link(&url, browser_map, browser_rules, default_browser);

    Ok(())
}
