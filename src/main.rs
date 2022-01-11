use std::{time::Duration};
use m4bar::{protocol, utils::{self, print_notice}, ewmh::Ewmh, modules, bar::Bar, block::{Block, BlockAttributes}};
use x11::xlib;

const ROOT_UID: u32 = 0;

fn main() {
    unsafe {
        let uid = libc::getuid();
        if uid == ROOT_UID {
            utils::throw_critical_error("Cannot use m4bar as root!");
        }

        let x11_client = protocol::X11::new();

        let bar_height = 24;
        let screen_info = x11_client.get_screen(0);
        let bar = Bar::new(
            &x11_client, 
            screen_info.x, 
            screen_info.y, 
            screen_info.width as u32, 
            bar_height, 
            String::from("#bf616a")
        );

        let ewmh = Ewmh::new(&x11_client);
        bar.configure_atoms(&ewmh);
        
        let block_attr = BlockAttributes {
            background: String::from("#ebcb8b"),
            border_bottom: 0,
            border_color: 0,
            border_top: 0,
            color: String::from("#2e3440"),
            font: "MesloLGS NF 10".to_string(),
            height: bar_height,
            padding: 10,
            width: 1,
            x: 20,
        };

        let mut block = Block::new(&x11_client, bar.window, block_attr);
        block.render("21:37".to_string());

        x11_client.show_window(bar.window);

        std::thread::spawn(move || {
            print_notice("Spawning module thread...");
            modules_event_loop();
        });

        let mut example_value = 20;
        loop {
            example_value += 1;
            block.rerender(format!("{}", example_value));
            let event = x11_client.get_event();
            match event {
                Some(e) => {
                    match e.get_type() {
                        xlib::Expose => {
                            block.expose();
                        },
                        _ => {}
                    }
                },
                None => {
                    // reduces little bit asking for pending events
                    std::thread::sleep(Duration::from_millis(200));
                }
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