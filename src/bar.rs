use x11::xlib::{ExposureMask, ButtonPressMask};

use crate::{protocol::X11, ewmh::Ewmh, colors};

// STRUT INDEXES:
const TOP: usize = 2;
const TOP_START_X: usize = 8;
const TOP_END_X: usize = 9;
const BOTTOM: usize = 3;
const BOTTOM_START_X: usize = 10;
const BOTTOM_END_X: usize = 11;

pub struct Bar {
    pub window: u64,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    bg_color: String,
}

impl Bar {
    pub unsafe fn new(x11: &X11, x: i32, y: i32, width: u32, height: u32, bg_color: String) -> Bar {
        let window = x11.create_window(
            x, 
            y, 
            width - 1, 
            height, 
            colors::hex64(&bg_color),
        );

        x11.select_input(window, ExposureMask|ButtonPressMask);

        Bar { window, x, y, width, height, bg_color }
    }

    pub unsafe fn configure_atoms(&self, ewmh: &Ewmh) {
        let mut strut: [i64; 12] = [0; 12];
        strut[TOP] = self.height as i64;
        strut[TOP_START_X] = self.x as i64;
        strut[TOP_END_X] = self.x as i64 + self.width as i64 - 1;

        ewmh.set_strut(self.window, strut);
        ewmh.set_always_show(self.window);
        ewmh.set_as_dock_type(self.window);
        ewmh.set_window_state(self.window);
    }
}