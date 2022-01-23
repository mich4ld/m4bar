use toml::{Value,map::Map};
use std::collections::HashMap;
use std::fs::read_to_string;
use crate::block::BlockAttributes;
use crate::errors::ParserError;
use crate::constants::defaults::*;
use crate::constants::config;
use crate::modules::pager::PagerAttributes;

pub struct BarConfig {
    background: String,
    height: i64,
    color: String,
    font: String,
}

pub struct InheritedDefaults {
    background: String,
    color: String,
    padding: i32,
    border_bottom: i32,
    border_top: i32,
    border_color: String,
    font: String,
    height: i64,
}

pub struct ClockConfig {
    attributes: BlockAttributes,
    format: String,
}

pub struct ParsedConfig {
    bar: BarConfig,
    blocks: HashMap<String, BlockAttributes>,
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

    fn parse_attributes(&self, block: &Map<String, Value>, defaults: &InheritedDefaults) -> BlockAttributes {
        let background_default = Value::from(defaults.background.clone());
        let background = block.get(config::BACKGROUND).unwrap_or(&background_default);

        let color_default = Value::from(defaults.color.clone());
        let color = block.get(config::COLOR).unwrap_or(&color_default);

        let padding_default = Value::from(BLOCK_PADDING);
        let padding = block.get(config::PADDING).unwrap_or(&padding_default);

        let font_default = Value::from(defaults.font.clone());
        let font = block.get(config::FONT).unwrap_or(&font_default);

        let border_top_default = Value::from(defaults.border_top);
        let border_top = block.get(config::BORDER_TOP).unwrap_or(&border_top_default);

        let border_bottom_default = Value::from(defaults.border_bottom);
        let border_bottom = block.get(config::BORDER_BOTTOM).unwrap_or(&border_bottom_default);

        let border_color_default = Value::from(defaults.border_color.clone());
        let border_color = block.get(config::BORDER_COLOR).unwrap_or(&border_color_default);

        let attributes = BlockAttributes {
            background: background.as_str().unwrap().to_string(),
            color: color.as_str().unwrap().to_string(),
            padding: padding.as_integer().unwrap() as i32,
            height: defaults.height as u32,
            font: font.as_str().unwrap().to_string(),
            border_bottom: border_bottom.as_integer().unwrap() as i32,
            border_top: border_top.as_integer().unwrap() as i32,
            border_color: border_color.as_str().unwrap().to_string(),
            width: 1,
        };

        attributes

    }

    fn parse_clock(&self, config: &ConfigMap, defaults: &InheritedDefaults) -> Option<ClockConfig> {
        let clock_option = config.get("clock");
        if clock_option.is_none() {
            return None;
        }

        let clock = clock_option.unwrap();
        let clock = clock.as_table().unwrap();
        
        let attributes = self.parse_attributes(clock, defaults);

        let default_format = Value::from(CLOCK_FORMAT);
        let format = clock.get(config::TIME_FORMAT).unwrap_or(&default_format);

        let clock_config = ClockConfig {
            attributes,
            format: format.as_str().unwrap().to_string(),
        };

        Some(clock_config)
    }

    fn parse_pager(&self, config: &ConfigMap, defaults: &InheritedDefaults) -> Option<PagerAttributes> {
        let pager_option = config.get("pager");
        if pager_option.is_none() {
            return None;
        }

        let pager = pager_option.unwrap();
        let pager = pager.as_table().unwrap();

        let attributes = self.parse_attributes(pager, defaults);
        let active_defaults = InheritedDefaults {
            background: attributes.background.clone(),
            color: attributes.color.clone(),
            font: attributes.font.clone(),
            padding: attributes.padding,
            height: attributes.height as i64,
            border_top: attributes.border_top,
            border_bottom: attributes.border_bottom,
            border_color: attributes.border_color.clone(),
        };

        let active_pager = pager.get("active");
        if active_pager.is_none() {
            let active_attributes = BlockAttributes {
                background: active_defaults.background,
                border_bottom: active_defaults.border_bottom,
                border_color: active_defaults.border_color,
                border_top: active_defaults.border_top,
                color: active_defaults.color,
                font: active_defaults.font,
                height: active_defaults.height as u32,
                padding: active_defaults.padding,
                width: 1,
            };

            // active_atributes same as attributes
            let pager_attributes = PagerAttributes {
                active_attributes,
                default_attributes: attributes,
            };

            return Some(pager_attributes);
        }

        let active_pager = active_pager
            .unwrap()
            .as_table()
            .unwrap();
        
        let active_attributes = self.parse_attributes(active_pager, &active_defaults);

        let pager_attributes = PagerAttributes { 
            active_attributes,
            default_attributes: attributes,
         };

         Some(pager_attributes)
    }

    fn parse_blocks(&self, config: &ConfigMap, defaults: &InheritedDefaults) -> HashMap<String, BlockAttributes>  {
        let mut blocks_map: HashMap<String, BlockAttributes> = HashMap::new();

        let raw_blocks = config.get("block").unwrap();
        let blocks = raw_blocks.as_table().unwrap();
        
        for (key, value) in blocks {
            let block = value.as_table().unwrap();
            let attributes = self.parse_attributes(block, defaults);

            blocks_map.insert(
                key.to_string(), 
                attributes
            );
        }

        blocks_map
    }

    pub fn parse(&self) -> Result<ParsedConfig, ParserError> {
        let raw_config = read_to_string(&self.path)?;
        let parsed_config = raw_config.parse::<Value>()?;

        let config: &ConfigMap = parsed_config.as_table().unwrap();
        let bar_config = self.parse_bar(config);

        let defaults = InheritedDefaults {
            background: bar_config.background.clone(),
            color: bar_config.color.clone(),
            border_bottom: BLOCK_BORDER,
            border_top: BLOCK_BORDER,
            border_color: BLOCK_BORDER_COLOR.to_string(),
            font: bar_config.font.clone(),
            height: bar_config.height,
            padding: BLOCK_PADDING,
        };

        let blocks_attributes = self.parse_blocks(config, &defaults);
        let clock = self.parse_clock(config, &defaults);
        
        println!("BAR background: {}", bar_config.background);
        println!("HELLO BLOCK BACKGROUND: {}", blocks_attributes.get("hello").unwrap().background);
        
        let result = ParsedConfig {
            bar: bar_config,
            blocks: blocks_attributes,
        };

        Ok(result)
    }
}
