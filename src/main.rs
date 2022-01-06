use m4bar;
use x11::xlib::{ButtonPressMask, ExposureMask};
mod protocol;

const ROOT_UID: u32 = 0;

fn main() {
    unsafe {
        let uid = libc::getuid();
        if uid == ROOT_UID {
            m4bar::throw_critical_error("Cannot use m4bar as root!");
        }

        let x = protocol::X11::new();
        let bar = x.create_window(200, 400, 1200, 400, 0xffffff);
        x.select_input(bar, ExposureMask | ButtonPressMask);
        x.show_window(bar);
        std::thread::sleep(std::time::Duration::from_secs(5));
        x.close();
    }
}
