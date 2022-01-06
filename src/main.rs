use m4bar;
use x11::xlib::{ButtonPressMask, ExposureMask, XA_ATOM, PropModeReplace, XA_CARDINAL, PropModeAppend};
mod protocol;

const ROOT_UID: u32 = 0;
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

fn main() {
    unsafe {
        let uid = libc::getuid();
        if uid == ROOT_UID {
            m4bar::throw_critical_error("Cannot use m4bar as root!");
        }

        let x = protocol::X11::new();

        let bar_width = 32;
        let screen_info = x.get_screen(0);

        let bar = x.create_window(
            screen_info.x, 
            screen_info.y, 
            screen_info.width as u32, 
            bar_width, 
            0xffffff
        );
        
        // set atoms
        let state_atom = x.get_atom(_NET_WM_STATE);
        let state_atoms_values = [
            x.get_atom(_NET_WM_STATE_STICKY),
            x.get_atom(_NET_WM_STATE_ABOVE),
        ];

        x.set_atom(
            bar, 
            XA_ATOM, 
            state_atom, 
            state_atoms_values.as_ptr() as *const u8, 
            2, 
            PropModeAppend,
        );

        let strut_atom = x.get_atom(_NET_WM_STRUT);
        let strut_partial_atom = x.get_atom(_NET_WM_STRUT_PARTIAL); 
        let mut strut: [i64; 12] = [0; 12];

        strut[2] = bar_width as i64;
        strut[8] = screen_info.x as i64;
        strut[9] = screen_info.x as i64 + screen_info.width as i64 - 1;

        x.set_atom(
            bar, 
            XA_CARDINAL,
            strut_atom, 
            strut.as_ptr() as *const u8, 
            4,
            PropModeReplace,
        );

        x.set_atom(
            bar, 
            XA_CARDINAL,
            strut_partial_atom, 
            strut.as_ptr() as *const u8, 
            12,
            PropModeReplace,
        );

        let window_type_atom = x.get_atom(_NET_WM_WINDOW_TYPE);
        let dock_type_atom = [x.get_atom(_NET_WM_WINDOW_TYPE_DOCK)];

        x.set_atom(
            bar, 
            XA_ATOM, 
            window_type_atom, 
            dock_type_atom.as_ptr() as *const u8, 
            1,
            PropModeReplace
        );

        let desktop_atom = x.get_atom(_NET_WM_DESKTOP);
        let always_show: [u32; 1] = [0xFFFFFFFF];

        x.set_atom(
            bar,
            XA_CARDINAL, 
            desktop_atom, 
            always_show.as_ptr() as *const u8, 
            1, 
            PropModeReplace,
        );

        x.select_input(bar, ExposureMask | ButtonPressMask);
        x.show_window(bar);
        std::thread::sleep(std::time::Duration::from_secs(10));
        x.close();
    }
}
