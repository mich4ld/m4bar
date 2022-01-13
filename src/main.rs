use std::time::Duration;
use m4bar::{protocol, utils::{self, print_notice}, ewmh::Ewmh, modules, bar::Bar, block::BlockAttributes, renderer::Renderer};
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
            border_bottom: 3,
            border_color: String::from("#000000"),
            border_top: 3,
            color: String::from("#2e3440"),
            font:  String::from("MesloLGS NF 10"),
            height: bar_height,
            padding: 10,
            width: 1,
        };

        let block_attr2 = BlockAttributes {
            background: String::from("#ffffff"),
            border_bottom: 0,
            border_color: String::from("#000000"),
            border_top: 0,
            color: String::from("#2e3440"),
            font:  String::from("Roboto 10"),
            height: bar_height,
            padding: 10,
            width: 1,
        };

        let mut renderer = Renderer::new(&x11_client, bar.window);
        let blocks = vec![
            (String::from("Random text"), block_attr),
            (String::from("Another random text"), block_attr2),
        ];

        renderer.create_blocks(blocks);

        x11_client.show_window(bar.window);

        std::thread::spawn(move || {
            print_notice("Spawning module thread...");
            modules_event_loop();
        });

        loop {
            let event = x11_client.get_event();

            match event {
                Some(e) => {
                    match e.get_type() {
                        xlib::Expose => {
                            //block.expose();
                            renderer.expose_all();
                        },
                        _ => {}
                    }
                },
                None => {
                    // reduces little bit asking for pending events
                    std::thread::sleep(Duration::from_millis(100));
                }
            }
        }
    }
}

fn modules_event_loop() {
    let mut clock = modules::clock::Clock::new(None);

    loop {
        let time_result = &clock.handle_tick();
        //println!("Time: {}", time_result);
        
        std::thread::sleep(Duration::from_secs(1));
    }
}