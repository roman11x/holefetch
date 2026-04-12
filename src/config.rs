use serde::{Serialize, Deserialize}; //so we can serialize and deserialize structs into and from the config.toml file

// This file contains the configuration for the holefetch program
// The config is stored in the config.toml file which this struct deserializes from and serializes to
// The user can set the logo, wallpaper, and brightness in the config.toml file
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ModuleConfig {
    pub show: Vec<String>,
}
// Configuration for the entire program
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
   pub logo: Option<String>,
   pub wallpaper_path: Option<String>,
   pub brightness: Option<f32>,
   pub modules: Option<ModuleConfig>,
}

impl Config {

    // Loads the config from the config.toml file
    // if the file doesn't exist, or any issues were encountered, it returns the default config
    pub fn load() -> Config {
        let home = std::env::var("HOME").unwrap_or_default();
        if home.is_empty(){ // if the home directory is not set, return the default config
            return Config::default();
        }

        let config_path = format!("{}/.config/holefetch/config.toml", home); // the config file path
        return if let Ok(config_file) = std::fs::read_to_string(&config_path) { // if the config file exists, deserialize it
            if let Ok(config) = toml::from_str(&config_file) { // if the config file is valid toml, return it
                config
            } else {
                Config::default() // otherwise return the default config
            }
        } else {
            Config::default() // otherwise return the default config
        }

    }
// Saves the config to the config.toml file
    pub fn save(&self) {
        let home = std::env::var("HOME").unwrap_or_default();
        if home.is_empty(){
            return; // if the home directory is not set, do nothing
        }
        let config_dir = format!("{}/.config/holefetch", home); // the config directory
        let config_path = format!("{}/config.toml", config_dir); // the config file

        std::fs::create_dir_all(&config_dir).unwrap_or_default(); // create the config directory if it doesn't exist
        let serialized = toml::to_string_pretty(&self).unwrap_or_default(); // serialize the config to toml
        std::fs::write(&config_path, serialized).unwrap_or_default(); // write the serialized config to the config.toml file
    }
}