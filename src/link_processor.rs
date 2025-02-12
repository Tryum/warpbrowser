use std::{collections::HashMap, os::windows::process::CommandExt, process::Command};

pub fn process_link<'a>(
    url: &'a str,
    browser_map: HashMap<String, String>,
    browser_rules: HashMap<&str, &str>,
    default_browser: &'a str,
) {
    const DETACHED_PROCESS: u32 = 0x00000008;
    if url.is_empty() {
        let browser = browser_map[default_browser].clone();
        println!("{browser}");
        Command::new(&browser)
            .creation_flags(DETACHED_PROCESS)
            .spawn()
            .expect("Failed to launch detached process");
    } else {
        let mut browser = default_browser;
        for (k, v) in browser_rules {
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
}
