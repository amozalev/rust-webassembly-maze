use rand::Rng;
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

        let mut maze = Maze { width, size, grid };
        maze.generate_maze(width);
        maze
    }

    fn generate_maze(&mut self, width: usize) {
        for i in 0..self.size {
            let mut neighbors: Vec<Link> = vec![];

            let neighbor_ind = i - width;
            if i >= width && neighbor_ind < self.size {
                neighbors.push(Link {
                    border: BorderSide::North as usize,
                    neighbor_ind,
                    neighbor_border: BorderSide::South as usize,
                });
            }

            let row = i / width;
            let neighbor_ind = i + 1;
            let neighbor_row = neighbor_ind / width;
            if neighbor_ind > 0 && neighbor_ind < self.size && row == neighbor_row {
                neighbors.push(Link {
                    border: BorderSide::East as usize,
                    neighbor_ind,
                    neighbor_border: BorderSide::West as usize,
                });
            }

            let num = rand::thread_rng().gen_range(0..neighbors.len().max(1));

            if neighbors.get(num).is_some() {
                let link = &neighbors[num];
                self.grid[i].0 |= link.border;
                self.grid[link.neighbor_ind].0 |= link.neighbor_border;
            }
        }
    }

    pub fn get_maze(&self) -> *const Cell {
        self.grid.as_ptr()
    }
}

// wasm-pack build --target web
