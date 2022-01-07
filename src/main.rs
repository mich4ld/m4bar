use std::{sync::mpsc::Sender, time::Duration};

use m4bar::{protocol, utils::{self, print_notice}, ewmh::{Ewmh}, modules};
use x11::xlib::{ButtonPressMask, ExposureMask};

const ROOT_UID: u32 = 0;

// STRUT INDEXES:
const TOP: usize = 2;
const BOTTOM: usize = 3;
const TOP_START_X: usize = 8;

const TOP_END_X: usize = 9;
const BOTTOM_START_X: usize = 10;
const BOTTOM_END_X: usize = 11;

fn main() {
    unsafe {
        let uid = libc::getuid();
        if uid == ROOT_UID {
            utils::throw_critical_error("Cannot use m4bar as root!");
        }

        let x = protocol::X11::new();

        let bar_height = 32;
        let screen_info = x.get_screen(0);

        let bar = x.create_window(
            screen_info.x, 
            screen_info.y, 
            screen_info.width as u32 - 1, 
            bar_height, 
            0xffffff
        );

        let mut strut: [i64; 12] = [0; 12];
        strut[TOP] = bar_height as i64;
        strut[TOP_START_X] = screen_info.x as i64;
        strut[TOP_END_X] = screen_info.x as i64 + screen_info.width as i64 - 1;
        
        let ewmh = Ewmh::new(&x);
        ewmh.set_strut(bar, strut);
        ewmh.set_always_show(bar);
        ewmh.set_as_dock_type(bar);
        ewmh.set_window_state(bar);

        x.select_input(bar, ExposureMask | ButtonPressMask);
        x.show_window(bar);
        

        std::thread::spawn(move || {
            print_notice("Spawning module thread...");
            modules_event_loop();
        });

        loop {
            let event = x.get_event();
            match event {
                Some(e) => {
                    println!("X11 event: {}", e.get_type());
                },
                None => {}
            }
        }
    }
}

fn modules_event_loop() {
    let clock = modules::clock::Clock::new(None);

    loop {
        let time = clock.handle_tick();
        println!("Current time: {}", time);
        std::thread::sleep(Duration::from_secs(1));
    }
}