use std::{io, process::Command};

use winreg::{
    enums::{HKEY_CLASSES_ROOT, HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, KEY_WRITE},
    RegKey,
};

const BROWSER_NAME: &str = "WarpBrowser";
const HANDLER_NAME: &str = "WarpBrowserURL";

#[derive(Debug)]
pub struct Browser {
    pub name: String,
    pub path: String,
}

pub fn get_browsers() -> io::Result<Vec<Browser>> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let mut browsers = Vec::new();

    let browsers_reg = hklm.open_subkey("SOFTWARE\\Clients\\StartMenuInternet")?;

    for key in browsers_reg.enum_keys().flatten() {
        let browser_key = browsers_reg.open_subkey(key)?;
        let browser_name: String = browser_key.get_value("")?;
        if browser_name == BROWSER_NAME {
            continue;
        }
        let browser_path_key = browser_key.open_subkey("shell\\open\\command")?;
        let browser_path: String = browser_path_key.get_value("")?;
        let browser_path = browser_path.trim_matches('\"').to_string();
        browsers.push(Browser {
            name: browser_name,
            path: browser_path,
        });
    }

    Ok(browsers)
}

pub fn get_default_browser() -> Option<String> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let path = r"Software\Microsoft\Windows\Shell\Associations\UrlAssociations\http\UserChoice";

    if let Ok(key) = hkcu.open_subkey(path) {
        if let Ok(prog_id) = key.get_value::<String, _>("ProgId") {
            return Some(prog_id);
        }
    }
    None
}

pub fn register_browser() {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let path = format!(r"SOFTWARE\Clients\StartMenuInternet\{}", BROWSER_NAME);

    let (key, _) = hkcu
        .create_subkey(path)
        .expect("Failed to create registry key");
    key.set_value("", &format!("{}", BROWSER_NAME))
        .expect("Failed to set registry value");

    let command_path = r"shell\open\command";
    let (command_key, _) = key
        .create_subkey(command_path)
        .expect("Failed to create command subkey");
    let binary_path = format!(r#""{}""#, std::env::current_exe().unwrap().display());
    command_key
        .set_value("", &binary_path)
        .expect("Failed to set command path");

    let (capabilities_key, _) = key
        .create_subkey("Capabilities")
        .expect("Failed to create Capabilities subkey");
    capabilities_key
        .set_value(
            "ApplicationDescription",
            &format!(
                "{} is a browser wrapper, redirecting URL calls to browsers given a set of rules.",
                BROWSER_NAME
            ),
        )
        .expect("Failed to set Application Description");
    capabilities_key
        .set_value("ApplicationName", &BROWSER_NAME.to_string())
        .expect("Failed to set ApplicationName");
    let (url_association_key, _) = capabilities_key
        .create_subkey("URLAssociations")
        .expect("Failed to create URLAssociations subkey");

    url_association_key
        .set_value("http", &HANDLER_NAME.to_string())
        .expect("Failed to create http url association");
    url_association_key
        .set_value("https", &HANDLER_NAME.to_string())
        .expect("Failed to create https url association");

    let binary_open_command = format!(
        r#""{}"  --url "%1""#,
        std::env::current_exe().unwrap().display()
    );

    let (handler_key, _) = hkcu
        .create_subkey(&format!(r"Software\Classes\{}", HANDLER_NAME))
        .expect("Failed to create handler subkey");
    handler_key
        .set_value("", &format!("{} URL", BROWSER_NAME))
        .expect("Failed to set registry value");
    handler_key
        .set_value("FriendlyTypeName", &format!("{} URL", BROWSER_NAME))
        .expect("Failed to set registry value");
    handler_key
        .set_value("URL Protocol", &"")
        .expect("Failed to set URL Protocol");
    let (handler_cmd, _) = handler_key
        .create_subkey(command_path)
        .expect("Failed to create subkey");
    handler_cmd
        .set_value("", &binary_open_command)
        .expect("Failed to set handler command");

    let registered_app = hkcu
        .open_subkey_with_flags("Software\\RegisteredApplications", KEY_WRITE)
        .expect("Failed to open RegisteredApplications");
    registered_app
        .set_value(
            BROWSER_NAME,
            &format!(
                r"Software\Clients\StartMenuInternet\{}\Capabilities",
                BROWSER_NAME
            ),
        )
        .expect("Failed to set browswer as registered app.");

    println!("Registered successfully.");
}

pub fn unregister_browser() {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let path = format!(r"Software\Classes\{}", BROWSER_NAME);
    hkcu.delete_subkey(&path)
        .expect(&format!("Failed to remove {} from registry", path));
}

pub fn set_default_browser() {
    register_browser();
    Command::new("cmd")
        .args(&[
            "/C",
            &format!(
                "start ms-settings:defaultapps?registeredAppUser={}",
                BROWSER_NAME
            ),
        ])
        .spawn()
        .expect("Failed to open default apps settings");
}
