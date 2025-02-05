use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Maze {
    pub width: usize,
}

#[wasm_bindgen]
impl Maze {
    pub fn new(width: usize) -> Self {
        Maze { width }
    }
}

// wasm-pack build --target web
