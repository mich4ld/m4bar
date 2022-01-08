use std::{sync::mpsc::Sender, time::Duration};

use m4bar::{protocol, utils::{self, print_notice}, ewmh::Ewmh, modules, bar::Bar};
use x11::xlib::{ButtonPressMask, ExposureMask};

const ROOT_UID: u32 = 0;

fn main() {
    unsafe {
        let uid = libc::getuid();
        if uid == ROOT_UID {
            utils::throw_critical_error("Cannot use m4bar as root!");
        }

        let x11_client = protocol::X11::new();

        let bar_height = 32;
        let screen_info = x11_client.get_screen(0);
        let bar = Bar::new(
            &x11_client, 
            screen_info.x, 
            screen_info.y, 
            screen_info.width as u32, 
            bar_height, 
            0xffffff
        );

        let ewmh = Ewmh::new(&x11_client);
        bar.configure_atoms(&ewmh);
        
        x11_client.show_window(bar.window);

        std::thread::spawn(move || {
            print_notice("Spawning module thread...");
            modules_event_loop();
        });

        loop {
            let event = x11_client.get_event();
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
    let mut clock = modules::clock::Clock::new(None);

    loop {
        let time_result = &clock.handle_tick();
        println!("Time: {}", time_result);
        
        std::thread::sleep(Duration::from_secs(1));
    }
}