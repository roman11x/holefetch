use serde::{Serialize, Deserialize}; //so we can serialize and deserialize structs into and from the config.toml file

// Configuration for which modules to display
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ModuleConfig {
    show: Vec<String>,
}
// Configuration for the entire program
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
   pub logo: Option<String>,
   pub wallpaper_path: Option<String>,
   pub brightness: Option<f32>,
   pub modules: ModuleConfig,
}

impl Config {

    // Loads the config from the config.toml file
    // if the file doesn't exist, or any issues were encountered, it returns the default config
    pub fn load() -> Config {
        let home = std::env::var("HOME").unwrap_or_default();
        if home.is_empty(){
            return Config::default();
        }

        let config_path = format!("{}/.config/holefetch/config.toml", home);
        return if let Ok(config_file) = std::fs::read_to_string(&config_path) {
            if let Ok(config) = toml::from_str(&config_file) {
                config
            } else {
                Config::default()
            }
        } else {
            Config::default()
        }

    }
}