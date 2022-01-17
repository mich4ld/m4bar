use x11::xlib::{XA_CARDINAL, PropModeReplace, XA_ATOM, PropModeAppend};
use crate::protocol::X11;
use crate::constants::atoms::*;

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

    pub unsafe fn change_virtual_desktop(&self, desktop_num: i64) {
        let data: [i64; 5] = [desktop_num, 0, 0, 0, 0];
        self.x11.send_event(_NET_CURRENT_DESKTOP, data);
    }

    pub unsafe fn get_virtual_desktops_number(&self) -> u8 {
        let desktops_num = self.x11
            .get_property(_NET_NUMBER_OF_DESKTOPS, self.x11.root)
            .unwrap_or_else(|| Vec::new());
            
        if desktops_num.len() < 1 {
            return 0;
        }

        return desktops_num[0];
    }

    pub unsafe fn get_current_virtual_desktop(&self) -> u8 {
        let current_desktop = self.x11
            .get_property(_NET_CURRENT_DESKTOP, self.x11.root)
            .unwrap_or_else(|| [0].to_vec());
        
        return current_desktop[0];
    }
}