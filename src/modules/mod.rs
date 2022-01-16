pub enum ModuleType {
    STATIC,
    CLOCK,
    PAGER,
}

pub trait Module {
    fn handle_tick(&self) -> UpdateMessage;
}

pub struct UpdateMessage {
    pub window: u64,
    pub text: String,
}

pub mod clock;
pub mod pager;

pub enum ModuleObject {
    CLOCK(clock::Clock)
}
