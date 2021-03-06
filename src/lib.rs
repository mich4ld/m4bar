pub enum ClickEvent {
    ChangeDesktop { desktop_num: u8 },
    ExecuteCommand { command: String },
}

pub mod constants;
pub mod utils;
pub mod colors;
pub mod ewmh;
pub mod protocol;
pub mod modules;
pub mod bar;
pub mod block;
pub mod renderer;
pub mod errors;
pub mod config;