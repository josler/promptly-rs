use dirs;
use std::fs;
use std::path::Path;
use toml;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub prefix: String,
}

impl Config {
    pub fn new() -> Self {
        let home = dirs::home_dir().expect("expected home dir");
        let config_dir = home.join(Path::new(".config/promptly"));
        fs::create_dir_all(&config_dir).expect("failed to create config dir");
        let path = config_dir.join("config.toml");
        match fs::read_to_string(&path) {
            Ok(d) => toml::from_str(&d).unwrap(),
            Err(_) => {
                let c = Config {
                    prefix: "".to_string(),
                };
                let st = toml::to_string_pretty(&c).expect("failed to serialize");
                fs::write(&path, st).expect("failed to write");
                c
            }
        }
    }
}
