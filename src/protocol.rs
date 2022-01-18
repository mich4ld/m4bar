use std::ffi::CString;
use std::mem::zeroed;
use std::os::raw::{c_int, c_long};
use std::ptr::null;
use std::slice::from_raw_parts;
use x11::xlib::{self, _XDisplay, Visual, XChangeProperty, PropertyChangeMask, FocusChangeMask};
use x11::xinerama;
use crate::utils::{self, print_warn};

pub struct ScreenInfo {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

pub struct X11 {
    pub display: *mut _XDisplay,
    xinerama_status: i32,
    pub root: u64,
    screen: i32,
    pub visual: *mut Visual,
}

unsafe extern "C" fn x11_error_handler(_display: *mut xlib::_XDisplay, _error: *mut xlib::XErrorEvent) -> i32 {
    0
}

impl X11 {
    pub unsafe fn new() -> X11 {
        utils::print_notice("Opening X11 connection...");
        let display = xlib::XOpenDisplay(null());
        if display.is_null() {
            utils::throw_critical_error("Cannot open connection with display server!");
        }

        let root = xlib::XDefaultRootWindow(display);
        let screen = xlib::XDefaultScreen(display);
        let visual = xlib::XDefaultVisual(display, screen);
        let xinerama_status = xinerama::XineramaIsActive(display);

        xlib::XSetErrorHandler(Some(x11_error_handler));
        xlib::XSelectInput(
            display,
            root,
            PropertyChangeMask|FocusChangeMask,
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

    pub unsafe fn create_subwindow(&self, parent: u64, x: i32, y: i32, width: u32, height: u32, bg: u64) -> u64 {
        let window = xlib::XCreateSimpleWindow(
            self.display, 
            parent,
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
            utils::print_warn("Xinerama is not active. Using full X display size");
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
            utils::throw_critical_error("Screen is not available");
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
    }

    pub unsafe fn resize_window(&self, window: u64, width: u32, height: u32) {
        xlib::XResizeWindow(self.display, window, width, height);
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

    pub unsafe fn get_event(&self) -> Option<xlib::XEvent> {
        let pending_events = xlib::XPending(self.display);
        let mut event: xlib::XEvent = zeroed();

        // reads event without blocking event loop
        if pending_events > 0 {
            xlib::XNextEvent(self.display, &mut event);

            return Some(event);
        }

        None
    }

    pub unsafe fn send_event(&self, atom_str: &str, data: [i64; 5]) {
        let atom = self.get_atom(atom_str);

        let event = xlib::XClientMessageEvent {
            window: self.root,
            display: self.display,
            format: 32,
            send_event: 0,
            serial: 0,
            message_type: atom,
            type_: xlib::ClientMessage,
            data: std::mem::transmute(data),            
        };

        xlib::XSendEvent(
            self.display,
            self.root,
            0,
            xlib::SubstructureNotifyMask,
            &mut event.into(),
        );
    }

    pub unsafe fn get_property(&self, atom_str: &str, window: u64) -> Option<Vec<u8>> {
        let atom = self.get_atom( atom_str);
        let mut actual_type: u64 = 0;
        let mut actual_format: i32 = 0;
        let mut nitems: u64 = 0;
        let mut bytes_after: u64 = 0;
        let mut prop: *mut u8 = std::mem::MaybeUninit::uninit().assume_init();

        let result = xlib::XGetWindowProperty(
            self.display,
             window, 
             atom, 
             0, 
             i64::MAX, 
             0,
             0, 
             &mut actual_type, 
             &mut actual_format, 
             &mut nitems, 
             &mut bytes_after, 
             &mut prop,
        );

        if result != 0 || actual_format == 0 {
            return None;
        }

        let arr = from_raw_parts(prop, nitems as usize);

        Some(arr.to_vec())
    }

    pub unsafe fn get_property64(&self, atom_str: &str, window: u64) -> Option<Vec<u64>> {
        let atom = self.get_atom( atom_str);
        let mut actual_type: u64 = 0;
        let mut actual_format: i32 = 0;
        let mut nitems: u64 = 0;
        let mut bytes_after: u64 = 0;
        let mut prop: *mut u8 = std::mem::MaybeUninit::uninit().assume_init();

        let result = xlib::XGetWindowProperty(
            self.display,
             window, 
             atom, 
             0, 
             i64::MAX, 
             0,
             0, 
             &mut actual_type, 
             &mut actual_format, 
             &mut nitems, 
             &mut bytes_after, 
             &mut prop,
        );

        if result != 0 || actual_format == 0 {
            return None;
        }

        let arr = from_raw_parts(std::mem::transmute::<*mut u8, *const u64>(prop), nitems as usize)
            .iter()
            .map(|&c| c as u64)
            .collect();

        Some(arr)
    }
}
