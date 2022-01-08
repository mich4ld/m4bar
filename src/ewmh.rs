use x11::xlib::{XA_CARDINAL, PropModeReplace, XA_ATOM, PropModeAppend};

use crate::protocol::X11;

// ATOM CONSTANTS
const _NET_ACTIVE_WINDOW: &str = "_NET_ACTIVE_WINDOW";
const _NET_WM_STATE_STICKY: &str = "_NET_WM_STATE_STICKY";
const _NET_WM_STATE_ABOVE: &str = "_NET_WM_STATE_ABOVE";
const _NET_WM_STATE: &str = "_NET_WM_STATE";
const _NET_WM_WINDOW_TYPE: &str = "_NET_WM_WINDOW_TYPE";
const _NET_WM_WINDOW_TYPE_DOCK: &str = "_NET_WM_WINDOW_TYPE_DOCK";
const _NET_WM_STRUT: &str = "_NET_WM_STRUT";
const _NET_WM_STRUT_PARTIAL: &str = "_NET_WM_STRUT_PARTIAL";
const _NET_WM_DESKTOP: &str = "_NET_WM_DESKTOP";
const _NET_WM_CURRENT_DESKTOP: &str = "_NET_CURRENT_DESKTOP";
const _NET_WM_NUMBER_OF_DESKTOPS: &str = "_NET_NUMBER_OF_DESKTOPS";
const _NET_WM_NAME: &str = "_NET_WM_NAME";

pub struct Ewmh<'a> {
    x11: &'a X11,
}

impl Ewmh<'_> {
    pub fn new(x11: &X11) -> Ewmh {
        Ewmh { x11 }
    }

    pub unsafe fn set_as_dock_type(&self, window: u64) {
        let window_type_atom = self.x11.get_atom(_NET_WM_WINDOW_TYPE);
        let dock_type_atom = [self.x11.get_atom(_NET_WM_WINDOW_TYPE_DOCK)];

        self.x11.set_atom(
            window, 
            XA_ATOM, 
            window_type_atom, 
            dock_type_atom.as_ptr() as *const u8, 
            1,
            PropModeReplace
        );
    }

    pub unsafe fn set_always_show(&self, window: u64) {
        let desktop_atom = self.x11.get_atom(_NET_WM_DESKTOP);
        let always_show: [u32; 1] = [0xFFFFFFFF];

        self.x11.set_atom(
            window,
            XA_CARDINAL, 
            desktop_atom, 
            always_show.as_ptr() as *const u8, 
            1, 
            PropModeReplace,
        );
    }
    
    pub unsafe fn set_window_state(&self, bar: u64) {
        let state_atom = self.x11.get_atom(_NET_WM_STATE);
        let state_atoms_values = [
            self.x11.get_atom(_NET_WM_STATE_STICKY),
            self.x11.get_atom(_NET_WM_STATE_ABOVE),
        ];

        self.x11.set_atom(
            bar, 
            XA_ATOM, 
            state_atom, 
            state_atoms_values.as_ptr() as *const u8, 
            2, 
            PropModeAppend,
        );
    }
    
    pub unsafe fn set_strut(&self, window: u64, strut: [i64; 12]) {
        let strut_atom = self.x11.get_atom(_NET_WM_STRUT);
        let strut_partial_atom = self.x11.get_atom(_NET_WM_STRUT_PARTIAL); 

        self.x11.set_atom(
            window, 
            XA_CARDINAL,
            strut_atom, 
            strut.as_ptr() as *const u8, 
            4,
            PropModeReplace,
        );

        self.x11.set_atom(
            window, 
            XA_CARDINAL,
            strut_partial_atom, 
            strut.as_ptr() as *const u8, 
            12,
            PropModeReplace,
        );
    }
}