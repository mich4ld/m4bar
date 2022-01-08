pub struct Clock {
    date_format: String,
}

impl Clock {
    pub fn new(date_format_option: Option<String>) -> Clock {
        let date_format = date_format_option.unwrap_or_else(|| "%H:%M".to_string());

        Clock { date_format }
    }

    pub fn handle_tick(&mut self) -> String {
        let now = chrono::Local::now();
        let time = now.format(&self.date_format);
            
        time.to_string()
    }
}