use std::collections::HashMap;

use crate::{block::{Block, BlockAttributes}, protocol::X11, modules::{ModuleType, Module, clock::Clock, ModuleObject}};

pub struct Renderer<'a> {
   blocks: HashMap<u64, Block<'a>>,
   x11: &'a X11,
   bar_window: u64,
}

impl Renderer<'_> {
    pub fn new(x11: &X11, bar_window: u64) -> Renderer {
        Renderer {
            x11,
            bar_window,
            blocks: HashMap::new(),
        }
    }

    pub unsafe fn expose_all(&mut self) {
        for (_, block) in self.blocks.iter_mut() {
            block.expose();
        }
    }

    pub unsafe fn create_blocks(&mut self, attributes: Vec<(String, BlockAttributes, ModuleType)>) -> Vec<ModuleObject> {
        let mut current_x = 0;
        let mut modules: Vec<ModuleObject> = Vec::new();

        for (text, block_attributes, module_type) in attributes {
            let mut block = Block::new(self.x11, current_x, self.bar_window, block_attributes);
            
            match module_type {
                ModuleType::CLOCK => {
                    let module = Clock::new(block.window);
                    modules.push(ModuleObject::CLOCK(module));
                },
                _ => {}
            }

            block.init(text);

            current_x += block.attributes.width as i32;
            self.blocks.insert(block.window, block);
        }

        modules
    }

    unsafe fn handle_width_change(&self) {
        if self.blocks.len() > 1 {
            
        }
    }

    pub unsafe fn update_block(&mut self, block_id: u64, text: String) {
        let block_option = self.blocks.get_mut(&block_id);

        if block_option.is_some() {
            let block = block_option.unwrap();
            let changed_width = block.rerender(text);

            if changed_width {
                self.handle_width_change();
            }
        }
    }
}
