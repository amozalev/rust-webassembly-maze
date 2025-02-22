use rand::{thread_rng, Rng};
use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

pub enum Border {
    North = 0b0001,
    East = 0b0010,
    South = 0b0100,
    West = 0b1000,
}

#[derive(Debug)]
#[wasm_bindgen]
pub struct Cell(usize);

struct Neighbor(usize, usize, usize);

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

        for i in 0..size {
            grid.push(Cell(0b0000));
        }

        let mut maze = Maze { width, size, grid };
        maze.generate_maze(width);
        maze
    }

    fn has_neighbor(ind: usize, maze_size: usize) -> bool {
        ind > 0 && ind < maze_size
    }

    fn generate_maze(&mut self, width: usize) {
        for i in 0..self.size {
            let mut neighbors: Vec<Neighbor> = vec![];

            let row = i / width;

            if i >= width && Self::has_neighbor(i - width, self.size) {
                neighbors.push(Neighbor(
                    Border::North as usize,
                    i - width,
                    Border::South as usize,
                ));
            }
            if Self::has_neighbor(i + 1, self.size) && row == ((i + 1) / width) {
                neighbors.push(Neighbor(
                    Border::East as usize,
                    i + 1,
                    Border::West as usize,
                ));
            }

            let num = if neighbors.len() > 0 {
                thread_rng().gen_range(0..neighbors.len())
            } else {
                0
            };

            if neighbors.get(num).is_some() {
                let neighbor = &neighbors[num];
                let border = neighbor.0;
                let neighbor_ind = neighbor.1;
                let neighbor_border = neighbor.2;

                self.grid[i] = Cell(self.grid[i].0 | border);
                self.grid[neighbor_ind] = Cell(self.grid[neighbor_ind].0 | neighbor_border);
            }
        }
    }

    pub fn get_maze(&self) -> *const Cell {
        self.grid.as_ptr()
    }
}

// wasm-pack build --target web
