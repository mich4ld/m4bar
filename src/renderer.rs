use std::collections::HashMap;
use crate::{block::{Block, BlockAttributes}, protocol::X11};

pub struct Renderer<'a> {
   blocks: HashMap<u64, Block<'a>>,
   x11: &'a X11,
   bar_window: u64,
   current_x: i32,
}

impl Renderer<'_> {
    pub fn new(x11: &X11, bar_window: u64) -> Renderer {
        Renderer {
            x11,
            bar_window,
            blocks: HashMap::new(),
            current_x: 0,
        }
    }

    pub unsafe fn expose_all(&mut self) {
        for (_, block) in self.blocks.iter_mut() {
            block.expose();
        }
    }

    pub unsafe fn create_block(&mut self, text: String, block_attributes: BlockAttributes) -> u64 {
        let mut block = Block::new(self.x11, self.current_x, self.bar_window, block_attributes);
        block.init(text);

        self.current_x += block.attributes.width as i32;
        let window = block.window;
        self.blocks.insert(block.window, block);
        
        window
    }

    unsafe fn handle_width_change(&self) {
        if self.blocks.len() > 1 {
            
        }
    }

    pub unsafe fn change_block_attributes(&mut self, block_id: &u64, block_attributes: BlockAttributes) {
        let block_option = self.blocks.get_mut(block_id);
        
        if block_option.is_some() {
            let block = block_option.unwrap();
            block.change_attributes(block_attributes);
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
