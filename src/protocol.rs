use std::ffi::CString;
use std::os::raw::{c_int, c_long};
use std::ptr::null;
use std::slice::from_raw_parts;
use x11::xlib::{self, _XDisplay, Visual, XChangeProperty, PropertyChangeMask, FocusChangeMask};
use x11::xinerama;

pub struct ScreenInfo {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

pub struct X11 {
    display: *mut _XDisplay,
    xinerama_status: i32,
    root: u64,
    screen: i32,
    visual: *mut Visual,
}

impl X11 {
    pub unsafe fn new() -> X11 {
        let display = xlib::XOpenDisplay(null());
        if display.is_null() {
            m4bar::throw_critical_error("Cannot open connection with display server!");
        }

        let root = xlib::XDefaultRootWindow(display);
        let screen = xlib::XDefaultScreen(display);
        let visual = xlib::XDefaultVisual(display, screen);
        let xinerama_status = xinerama::XineramaIsActive(display);

        xlib::XSelectInput(
            display,
            root,
            PropertyChangeMask | FocusChangeMask,
        );

        X11 { display, xinerama_status, root, screen, visual, }
    }

    pub unsafe fn close(&self) {
        xlib::XCloseDisplay(self.display);
    }

    pub unsafe fn create_window(&self, x: i32, y: i32, width: u32, height: u32, bg: u64) -> u64 {
        let window = xlib::XCreateSimpleWindow(
            self.display, 
            self.root,
            x, 
            y, 
            width, 
            height, 
            0, 
            0,
            bg
        );

        window
    }

    pub unsafe fn select_input(&self, window: u64, masks: c_long) {
        xlib::XSelectInput(self.display, window, masks);
    }

    pub unsafe fn get_screen(&self, screen_num: i32) -> ScreenInfo {
        let mut screen_index = 0;

        if self.xinerama_status == 0 {
            m4bar::printwarn("Xinerama is not active. Using full X display size");
            let display_width = xlib::XDisplayWidth(self.display, self.screen);
            let display_height = xlib::XDisplayHeight(self.display, self.screen);

            return ScreenInfo {
                x: 0,
                y: 0,
                height: display_height,
                width: display_width,
            };
        }

        let screens_result = xinerama::XineramaQueryScreens(self.display, &mut screen_index);
        let screens = from_raw_parts(screens_result, screen_index as usize);
        let screen = screens.get(screen_num as usize);

        if screen.is_none() {
            m4bar::throw_critical_error("Screen is not not available");
        }

        let screen = screen.unwrap();
        
        ScreenInfo { 
            x: screen.x_org as i32,
            y: screen.y_org as i32,
            width: screen.width as i32,
            height: screen.height as i32,
         }
    }

    pub unsafe fn show_window(&self, window: u64) {
        xlib::XMapWindow(self.display, window);
        xlib::XSync(self.display, xlib::False);
        xlib::XFlush(self.display);
    }

    pub unsafe fn get_atom(&self, atom_name: &str) -> xlib::Atom {
        let name = CString::new(atom_name).unwrap();

        xlib::XInternAtom(self.display, name.as_ptr(), xlib::False)
    }

    pub unsafe fn set_atom(&self, window: u64, atom_type: u64, atom: u64, value: *const u8, len: i32, mode: c_int) {
        XChangeProperty(
            self.display, 
            window, 
            atom, 
            atom_type, 
            32, 
            mode, 
            value, 
            len
        );
    }
}