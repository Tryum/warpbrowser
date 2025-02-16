use std::{collections::HashMap, fs, io, path::PathBuf};

use serde::{Deserialize, Serialize};

const CONFIG_FILE: &str = ".warpbrowser.json";

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    default_browser: String,
    rules: HashMap<String, String>,
}

impl Config {
    fn get_config_path() -> io::Result<PathBuf> {
        match dirs::home_dir() {
            Some(path) => Ok(path.join(CONFIG_FILE)),
            None => {
                return Err(io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Home directory not found",
                ))
            }
        }
    }

    pub fn save(&self) -> io::Result<()> {
        let content = serde_json::to_string_pretty(&self)?;
        let config_path = Self::get_config_path()?;
        fs::write(config_path, content)?;
        Ok(())
    }

    pub fn add(&mut self, website: String, browser: String) {
        self.rules.insert(website, browser);
    }

    pub fn load() -> io::Result<Config> {
        let config_path = Self::get_config_path()?;
        let config = if let Ok(content) = fs::read_to_string(config_path) {
            let config: Config = serde_json::from_str(&content)?;
            config
        } else {
            Config {
                rules: HashMap::new(),
                default_browser: String::new(),
            }
        };

        Ok(config)
    }

    pub fn set_default_browser(&mut self, default_browser: String) {
        self.default_browser = default_browser;
    }

    pub fn default_browser(&self) -> String {
        self.default_browser.clone()
    }

    pub fn rules(&self) -> &HashMap<String, String> {
        &self.rules
    }
}
