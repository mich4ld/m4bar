use toml::{Value,map::Map};
use std::collections::HashMap;
use std::fs::read_to_string;
use crate::block::BlockAttributes;
use crate::errors::ParserError;
use crate::constants::defaults::*;
use crate::constants::config;

pub struct BarConfig {
    background: String,
    height: i64,
    color: String,
    font: String,
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
        let background = bar.get(config::BACKGROUND).unwrap_or(&background_default);

        let color_default = Value::from(BAR_COLOR);
        let color = bar.get(config::COLOR).unwrap_or(&color_default);

        let height_default = Value::from(BAR_HEIGHT);
        let height = bar.get(config::HEIGHT).unwrap_or(&height_default);

        let font_default = Value::from(BAR_FONT);
        let font = bar.get(config::FONT).unwrap_or(&font_default);

        BarConfig { 
            background: background.as_str().unwrap().to_string(),
            color: color.as_str().unwrap().to_string(),
            height: height.as_integer().unwrap(),
            font: font.as_str().unwrap().to_string(),
        }
    }

    fn parse_blocks(&self, config: &ConfigMap, bar_config: &BarConfig) -> HashMap<String, BlockAttributes>  {
        let mut blocks_map: HashMap<String, BlockAttributes> = HashMap::new();

        let raw_blocks = config.get("block").unwrap();
        let blocks = raw_blocks.as_table().unwrap();
        
        for (key, value) in blocks {
            let block = value.as_table().unwrap();

            let background_default = Value::from(bar_config.background.clone());
            let background = block.get(config::BACKGROUND).unwrap_or(&background_default);

            let color_default = Value::from(bar_config.color.clone());
            let color = block.get(config::COLOR).unwrap_or(&color_default);

            let padding_default = Value::from(BLOCK_PADDING);
            let padding = block.get(config::PADDING).unwrap_or(&padding_default);

            let font_default = Value::from(bar_config.font.clone());
            let font = block.get(config::FONT).unwrap_or(&font_default);

            let border_top_default = Value::from(BLOCK_BORDER);
            let border_top = block.get(config::BORDER_TOP).unwrap_or(&border_top_default);

            let border_bottom_default = Value::from(BLOCK_BORDER);
            let border_bottom = block.get(config::BORDER_BOTTOM).unwrap_or(&border_bottom_default);

            let border_color = block.get(config::BORDER_COLOR).unwrap_or(&background_default);

            let attributes = BlockAttributes {
                background: background.as_str().unwrap().to_string(),
                color: color.as_str().unwrap().to_string(),
                padding: padding.as_integer().unwrap() as i32,
                height: bar_config.height as u32,
                font: font.as_str().unwrap().to_string(),
                border_bottom: border_bottom.as_integer().unwrap() as i32,
                border_top: border_top.as_integer().unwrap() as i32,
                border_color: border_color.as_str().unwrap().to_string(),
                width: 1,
            };

            blocks_map.insert(
                key.to_string(), 
                attributes
            );
        }

        blocks_map
    }

    pub fn parse(&self) -> Result<String, ParserError> {
        let raw_config = read_to_string(&self.path)?;
        let parsed_config = raw_config.parse::<Value>()?;

        let config: &ConfigMap = parsed_config.as_table().unwrap();

        let bar_config = self.parse_bar(config);
        let blocks_attributes = self.parse_blocks(config, &bar_config);

        println!("BAR background: {}", bar_config.background);
        println!("HELLO BLOCK BACKGROUND: {}", blocks_attributes.get("hello").unwrap().background);
        Ok(String::from("Something"))
    }
}
