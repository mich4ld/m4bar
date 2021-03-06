use crate::{ ewmh::Ewmh, renderer::Renderer, block::BlockAttributes};

pub struct XWindow {
   block: u64,
}

impl XWindow {
    pub unsafe fn new(renderer: &mut Renderer, ewmh: &Ewmh, attributes: BlockAttributes) -> XWindow {
        let window_title = ewmh.get_window_title();
        let block = renderer.create_block(window_title, attributes);

        XWindow { block }
    }

    pub unsafe fn rerender(&self, renderer: &mut Renderer, ewmh: &Ewmh) {
        let window_title = ewmh.get_window_title();

        renderer.update_block(self.block, window_title);
    }
}