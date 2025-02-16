use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::{env, io};

use browsers::{get_browsers, get_default_browser, register_browser, set_default_browser, Browser};
use clap::{Arg, ArgAction, Command};
use config::Config;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;
use link_processor::process_link;
use url::Url;

mod browsers;
mod config;
mod link_processor;
mod windows;

fn select_browser(browsers: &[Browser], prompt: String) -> String {
    let options: Vec<String> = browsers.iter().map(|b| b.name.clone()).collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    options[selection].clone()
}

fn main() -> io::Result<()> {
    let args: Vec<_> = env::args().collect();
    {
        let exe = env::current_exe().unwrap();
        let parent = exe.parent().unwrap();
        let mut file = File::create(parent.join("last_call.txt"))?;
        file.write_all(args.join(" ").as_bytes())?;
    }
    let matches = Command::new("WarpBrowser")
        .version("0.1")
        .author("Thibault Jochem")
        .about("Capture and redirects URLs based on rules")
        .arg(
            Arg::new("url")
                .short('u')
                .long("url")
                .help("The URL to capture and redirect")
                .value_name("url")
                .exclusive(true),
        )
        .arg(
            Arg::new("register")
                .long("register")
                .exclusive(true)
                .action(ArgAction::SetTrue)
                .help("Register in system browser list"),
        )
        .arg(
            Arg::new("select_browser")
                .long("select-browser")
                .exclusive(true)
                .action(ArgAction::SetTrue)
                .help("Select default browser"),
        )
        .arg(
            Arg::new("add_rule")
                .long("add-rule")
                .value_name("website")
                .exclusive(true)
                .help("Add a new rule: specify a website"),
        )
        .get_matches();

    let browsers = get_browsers()?;

    let mut config = Config::load()?;

    if matches.get_flag("register") {
        if let Some(default_browser) = get_default_browser() {
            config.set_default_browser(default_browser);
            config.save()?;
        }
        register_browser();
        set_default_browser();
        return Ok(());
    }

    if matches.get_flag("select_browser") {
        let browser = select_browser(
            &browsers,
            "Choose a default browser to open  URLs".to_string(),
        );
        config.set_default_browser(browser);
        config.save().expect("Failed to save default browser");
        return Ok(());
    }

    if let Some(url) = matches.get_one::<String>("add_rule") {
        let _ = Url::parse(url).expect("Failed to parse URL");

        let browser = select_browser(&browsers, format!("Choose a browser to open {}", &url));

        config.add(url.clone(), browser);
        config.save()?;
        return Ok(());
    }

    let mut browser_map = HashMap::new();

    for browser in browsers {
        browser_map.insert(browser.name, browser.path);
    }

    let browser_rules = config.rules();

    if let Some(url) = matches.get_one::<String>("url") {
        let default_browser = config.default_browser();
        process_link(url, &browser_map, browser_rules, &default_browser);
    }

    Ok(())
}
