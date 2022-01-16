use crate::ewmh::Ewmh;

pub struct Pager {
    blocks: Vec<u64>,
    current: u8,
    active: bool
}

impl Pager {
    pub fn new() -> Pager {
        Pager {
            active: false,
            blocks: Vec::new(),
            current: 0,
        }
    }

    pub fn add_block(&mut self, block: u64) {
        self.blocks.push(block);
        self.active = true;
    }

    pub unsafe fn change_desktop(&mut self, ewmh: &Ewmh, desktop_num: u8) {
        self.current = desktop_num;
        ewmh.change_virtual_desktop(desktop_num as i64);
    }
}