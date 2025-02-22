use super::{BorderSide, Link, Maze};

use rand::Rng;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct BinaryTree;

#[wasm_bindgen]
impl BinaryTree {
    pub fn generate_maze(maze: &mut Maze) {
        for i in 0..maze.size {
            let mut neighbors: Vec<Link> = vec![];

            let neighbor_ind = i - maze.width;
            if i >= maze.width && neighbor_ind < maze.size {
                neighbors.push(Link {
                    border: BorderSide::North as usize,
                    neighbor_ind,
                    neighbor_border: BorderSide::South as usize,
                });
            }

            let row = i / maze.width;
            let neighbor_ind = i + 1;
            let neighbor_row = neighbor_ind / maze.width;
            if neighbor_ind > 0 && neighbor_ind < maze.size && row == neighbor_row {
                neighbors.push(Link {
                    border: BorderSide::East as usize,
                    neighbor_ind,
                    neighbor_border: BorderSide::West as usize,
                });
            }

            let num = rand::thread_rng().gen_range(0..neighbors.len().max(1));

            if neighbors.get(num).is_some() {
                let link = &neighbors[num];
                maze.grid[i].0 |= link.border;
                maze.grid[link.neighbor_ind].0 |= link.neighbor_border;
            }
        }
    }
}
