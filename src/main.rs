use std::collections::HashMap;
use std::{env, io};

use browser_mapping::get_browser_rules;
use browsers::get_browsers;
use clap::{Arg, ArgAction, Command};
use link_processor::process_link;

mod browser_mapping;
mod browsers;
mod link_processor;

fn main() -> io::Result<()> {
    let matches = Command::new("WarpBrowser")
        .version("0.1")
        .author("Thibault Jochem")
        .about("Capture and redirects URLs based on rules")
        .arg(
            Arg::new("url")
                .help("The URL to capture and redirect")
                .required(false)
                .index(1),
        )
        .arg(
            Arg::new("browsers")
                .short('l')
                .long("list-browsers")
                .exclusive(true)
                .action(ArgAction::SetTrue)
                .help("List available browsers"),
        )
        .get_matches();

    let browsers = get_browsers()?;

    if matches.get_flag("browsers") {
        for b in browsers {
            println!("- {} : {}", b.name, b.path);
        }
        return Ok(());
    }

    let default_browser = "Mozilla Firefox";

    let mut browser_map = HashMap::new();

    for browser in browsers {
        browser_map.insert(browser.name, browser.path);
    }

    let browser_rules: HashMap<&str, &str> = get_browser_rules();

    if let Some(url) = matches.get_one::<String>("url") {
        process_link(&url, browser_map, browser_rules, default_browser);
    }

    Ok(())
}
