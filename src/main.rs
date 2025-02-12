use std::collections::HashMap;
use std::os::windows::process::CommandExt;
use std::process::Command;
use std::{env, io};

use browsers::list_browsers;

mod browsers;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let url = if args.len() > 1 {
        args[1].clone()
    } else {
        String::new()
    };

    let browsers = list_browsers()?;

    const DETACHED_PROCESS: u32 = 0x00000008;

    let work_browser = "Google Chrome";
    let default_browser = "Mozilla Firefox";

    let mut browser_map = HashMap::new();

    for browser in browsers {
        browser_map.insert(browser.name, browser.path);
    }

    let mut url_map = HashMap::new();

    url_map.insert("https://google.com/", work_browser);

    if url.is_empty() {
        let browser = browser_map[default_browser].clone();
        println!("{browser}");
        Command::new(&browser)
            .creation_flags(DETACHED_PROCESS)
            .spawn()
            .expect("Failed to launch detached process");
    } else {
        let mut browser = default_browser;
        for (k, v) in url_map {
            if url.starts_with(k) {
                browser = v;
                break;
            }
        }
        let browser = browser_map[browser].clone();
        println!("{browser}");
        Command::new(browser)
            .arg(url)
            .creation_flags(DETACHED_PROCESS)
            .spawn()
            .expect("Failed to launch detached process");
    }

    Ok(())
}
