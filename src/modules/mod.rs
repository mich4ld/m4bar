pub enum ModuleType {
    STATIC,
    CLOCK
}

pub trait Module {
    fn handle_tick(&self) -> UpdateMessage;
}

pub struct UpdateMessage {
    pub window: u64,
    pub text: String,
}

pub mod clock;

pub enum ModuleObject {
    CLOCK(clock::Clock)
}
