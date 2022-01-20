use toml::{Value,map::Map};
use std::fs::read_to_string;
use crate::errors::ParserError;
use crate::constants::config::*;

pub struct BarConfig {
    background: String,
    height: i64,
    color: String,
}

pub struct Config {
    path: String,
}

type ConfigMap = Map<String, Value>;

impl Config {
    pub fn new(path: String) -> Config {
        Config { path, }
    }

    fn parse_bar(&self, config: &ConfigMap) -> BarConfig {
        let raw_bar = config.get("bar").unwrap();
        let bar = raw_bar.as_table().unwrap();

        let background_default = Value::from(BAR_BACKGROUND);
        let background = bar.get("background").unwrap_or(&background_default);

        let color_default = Value::from(BAR_COLOR);
        let color = bar.get("color").unwrap_or(&color_default);

        let height_default = Value::from(BAR_HEIGHT);
        let height = bar.get("height").unwrap_or(&height_default);

        BarConfig { 
            background: background.as_str().unwrap().to_string(),
            color: color.as_str().unwrap().to_string(),
            height: height.as_integer().unwrap(),
        }
    }

    pub fn parse(&self) -> Result<String, ParserError> {
        let raw_config = read_to_string(&self.path)?;
        let parsed_config = raw_config.parse::<Value>()?;

        let config: &ConfigMap = parsed_config.as_table().unwrap();

        let bar_config = self.parse_bar(config);

        println!("BAR background: {:?}", bar_config.background);
        Ok(String::from("Lol"))
    }
}
