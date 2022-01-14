use super::{Module, UpdateMessage};

pub struct Clock {
    date_format: String,
    window: u64,
}

impl Clock {
    pub fn new(window: u64) -> Clock {
        //let date_format = date_format_option.unwrap_or_else(|| "%H:%M".to_string());

        Clock { 
            date_format: String::from("%H:%M"), 
            window,
        }
    }
}

impl Module for Clock {
    fn handle_tick(&self) -> UpdateMessage {
        let now = chrono::Local::now();
        let time = now.format(&self.date_format);
            
        UpdateMessage {
            text: time.to_string(),
            window: self.window,
        }
    }
}