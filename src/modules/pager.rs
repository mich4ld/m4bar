use std::collections::HashMap;

use crate::{ewmh::Ewmh, renderer::Renderer, ClickEvent, block::BlockAttributes};

pub struct PagerAttributes {
    pub active_attributes: BlockAttributes,
    pub default_attributes: BlockAttributes,
}

impl PagerAttributes {
    pub fn clone(&self) -> Self {
        let active_attributes = BlockAttributes {
            background: self.active_attributes.background.clone(),
            border_bottom: self.active_attributes.border_bottom,
            border_color: self.active_attributes.border_color.clone(),
            border_top: self.active_attributes.border_top,
            font: self.active_attributes.font.clone(),
            color: self.active_attributes.color.clone(),
            height: self.active_attributes.height,
            padding: self.active_attributes.padding,
            width: self.active_attributes.width,
        };

        let default_attributes = BlockAttributes {
            background: self.default_attributes.background.clone(),
            border_bottom: self.default_attributes.border_bottom,
            border_color: self.default_attributes.border_color.clone(),
            border_top: self.default_attributes.border_top,
            font: self.default_attributes.font.clone(),
            color: self.default_attributes.color.clone(),
            height: self.default_attributes.height,
            padding: self.default_attributes.padding,
            width: self.default_attributes.width,
        };

        PagerAttributes {
            active_attributes,
            default_attributes,
        }
    }
}

pub struct Pager {
    blocks: Vec<u64>,
    current: u8,
    attributes: PagerAttributes,
}

impl Pager {
    pub fn new(attributes: PagerAttributes) -> Pager {
        Pager {
            blocks: Vec::new(),
            current: 0,
            attributes,
        }
    }

    pub unsafe fn render_pager(
        &mut self, 
        renderer: &mut Renderer, 
        ewmh: &Ewmh, 
        click_events: &mut HashMap<u64, ClickEvent>,
    ) {
        let virtual_desktops = ewmh.get_virtual_desktops_number();
        let active_virtual_desktop = ewmh.get_current_virtual_desktop();
        self.current = active_virtual_desktop;

        for desktop in 0..virtual_desktops {
            let pager_attributes = self.attributes.clone();
            let attributes = if desktop == active_virtual_desktop {
                pager_attributes.active_attributes
            } else {
                pager_attributes.default_attributes
            };

            let text=  (desktop + 1).to_string();
            let block = renderer.create_block(text, attributes);
            self.add_block(block);
            click_events.insert(block, ClickEvent::ChangeDesktop { desktop_num: desktop });
        }
    }

    pub unsafe fn rerender_pager(&self, renderer: &mut Renderer) {
        for (index, block) in self.blocks.iter().enumerate() {
            let pager_attributes = self.attributes.clone();
            let attributes = if index == self.current as usize {
                pager_attributes.active_attributes
            } else {
                pager_attributes.default_attributes
            };

            renderer.change_block_attributes(block, attributes);
        }
    }

    pub fn add_block(&mut self, block: u64) {
        self.blocks.push(block);
    }

    pub fn change_current_desktop(&mut self, desktop_num: u8) {
        self.current = desktop_num;
    }

    pub unsafe fn change_desktop(&mut self, ewmh: &Ewmh, desktop_num: u8) {
        self.current = desktop_num;
        ewmh.change_virtual_desktop(desktop_num as i64);
    }
}