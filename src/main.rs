use m4bar::{protocol, utils, ewmh::{Ewmh}};
use x11::xlib::{ButtonPressMask, ExposureMask};

const ROOT_UID: u32 = 0;

fn main() {
    unsafe {
        let uid = libc::getuid();
        if uid == ROOT_UID {
            utils::throw_critical_error("Cannot use m4bar as root!");
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

        let mut strut: [i64; 12] = [0; 12];

        strut[2] = bar_width as i64;
        strut[8] = screen_info.x as i64;
        strut[9] = screen_info.x as i64 + screen_info.width as i64 - 1;
        
        let ewmh = Ewmh::new(&x);

        ewmh.set_strut(bar, strut);
        ewmh.set_always_show(bar);
        ewmh.set_dock_type(bar);
        ewmh.set_window_state(bar);

        x.select_input(bar, ExposureMask | ButtonPressMask);
        x.show_window(bar);
        std::thread::sleep(std::time::Duration::from_secs(10));
        x.close();
    }
}
