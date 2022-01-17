use std::{time::Duration, sync::mpsc, collections::HashMap};
use m4bar::{protocol, utils::{self, print_notice}, constants::atoms, ewmh::Ewmh, modules::{Module, UpdateMessage, ModuleType, ModuleObject, clock::Clock, pager::{Pager, PagerAttributes}, xwindow::XWindow}, bar::Bar, block::BlockAttributes, renderer::Renderer, ClickEvent};
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

        let block_attr3 = BlockAttributes {
            background: String::from("#ebcb8b"),
            border_bottom: 0,
            border_color: String::from("#000000"),
            border_top: 0,
            color: String::from("#2e3440"),
            font:  String::from("Roboto Mono 10"),
            height: bar_height,
            padding: 10,
            width: 1,
        };

        let mut renderer = Renderer::new(&x11_client, bar.window);
        let mut modules = Vec::new();
        let mut click_events = HashMap::new();
        
        let pager_default_attributes = BlockAttributes {
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

        let pager_active_attributes = BlockAttributes {
            background: String::from("#000000"),
            color: String::from("#ffffff"),
            border_bottom: 0,
            border_color: String::from("#000000"),
            border_top: 0,
            font:  String::from("Roboto 10"),
            height: bar_height,
            padding: 10,
            width: 1,
        };

        let pager_attributes = PagerAttributes {
              active_attributes: pager_active_attributes,
              default_attributes: pager_default_attributes,
        };

        let xwindow_attributes = BlockAttributes {
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

        let mut pager = Pager::new(pager_attributes);
        pager.render_pager(&mut renderer, &ewmh, &mut click_events);

        let blocks = vec![
            (String::from("Random text"), block_attr, ModuleType::STATIC),
            (String::from("Another random text"), block_attr2, ModuleType::STATIC),
            (String::from("21:37"), block_attr3, ModuleType::CLOCK),
        ];

        for (text, block_attributes, module_type) in blocks {
            let block = renderer.create_block(text, block_attributes);
            match module_type {
                ModuleType::CLOCK => {
                    let module = Clock::new(block);
                    modules.push(ModuleObject::CLOCK(module));
                },
                _ => {}
            }
        }

        let _xwindow = XWindow::new(&mut renderer, &ewmh, xwindow_attributes);
        x11_client.show_window(bar.window);

        let (sender, receiver) = mpsc::channel();
        std::thread::spawn(move || {
            print_notice("Spawning module thread...");
            modules_event_loop(sender, modules);
        });

        let current_desktop_atom = x11_client.get_atom(atoms::_NET_CURRENT_DESKTOP);

        loop {
            let event = x11_client.get_event();
            if let Ok(message) = receiver.try_recv() {
                renderer.update_block(message.window, message.text);
            };

            match event {
                Some(e) => {
                    match e.get_type() {
                        xlib::Expose => {
                            renderer.expose_all();
                        },
                        xlib::PropertyNotify => {
                            if e.property.atom == current_desktop_atom {
                                let desktop_num = ewmh.get_current_virtual_desktop();
                                pager.change_desktop(&ewmh, desktop_num);
                                pager.rerender_pager(&mut renderer);
                            }
                        },
                        xlib::ButtonPress => {
                            let click_event_option = click_events.get(&e.button.subwindow);

                            match click_event_option {
                                Some(click_event) => {
                                    match click_event {
                                        ClickEvent::ChangeDesktop { desktop_num } => {
                                            pager.change_desktop(&ewmh, *desktop_num);
                                        }
                                        ClickEvent::ExecuteCommand { command } => todo!(),
                                    }
                                },
                                _ => {}
                            }
                        }
                        _ =>  {}
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

fn modules_event_loop(sender: mpsc::Sender<UpdateMessage>, modules: Vec<ModuleObject>) {
    loop {
        for module in modules.iter() {
            match module {
                ModuleObject::CLOCK(clock) => {
                    let message = clock.handle_tick();
                    sender.send(message).unwrap();
                }
            }
        }

        std::thread::sleep(Duration::from_secs(1));
    }
}