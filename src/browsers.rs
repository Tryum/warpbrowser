use std::io;

use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};

#[derive(Debug)]
pub struct Browser {
    pub name: String,
    pub path: String,
}

pub fn list_browsers() -> io::Result<Vec<Browser>> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let mut browsers = Vec::new();

    let browsers_reg = hklm.open_subkey("SOFTWARE\\Clients\\StartMenuInternet")?;

    for key in browsers_reg.enum_keys().flatten() {
        let browser_key = browsers_reg.open_subkey(key)?;
        let browser_name: String = browser_key.get_value("")?;
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
