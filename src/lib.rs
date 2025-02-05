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
    pub fn new() -> Self {
        Maze { width: 10 }
    }
}

// wasm-pack build --target web
