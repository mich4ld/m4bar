use std::collections::HashMap;

use crate::block::Block;

pub struct Renderer<'a> {
   blocks: HashMap<u64, Block<'a>>
}

impl Renderer<'_> {
    pub fn show_all(&self) {}
}

