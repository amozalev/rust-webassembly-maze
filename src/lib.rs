mod binary_tree;

use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

pub enum BorderSide {
    North = 0b0001,
    East = 0b0010,
    South = 0b0100,
    West = 0b1000,
}

struct Link {
    border: usize,
    neighbor_ind: usize,
    neighbor_border: usize,
}

#[derive(Debug)]
#[wasm_bindgen]
pub struct Cell(usize);

#[wasm_bindgen]
pub struct Maze {
    pub width: usize,
    pub size: usize,
    grid: Vec<Cell>,
}

#[wasm_bindgen]
impl Maze {
    pub fn new(width: usize) -> Self {
        let size = width * width;
        let mut grid: Vec<Cell> = vec![];

        for _ in 0..size {
            grid.push(Cell(0b0000));
        }

        Self { width, size, grid }
    }

    pub fn get_maze(&self) -> *const Cell {
        self.grid.as_ptr()
    }
}

// wasm-pack build --target web
