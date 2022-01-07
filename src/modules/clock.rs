pub struct Clock {
    date_format: &str,
}

impl Clock {
    fn new(date_format_option: Option<&str>) -> Clock {
        let date_format = date_format_option.unwrap_or_else("%H:%M");

        Clock { date_format }
    }

    fn handle_tick(&self) -> String {
        let now = chrono::Local::now();
        let time = now.format(self.date_format);

        time.to_string()
    }
}