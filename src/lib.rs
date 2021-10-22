mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    let message = format!("Hello, wasm-game-of-life, {}", name);
    alert(&message);
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

impl Cell {
    fn toggle(&mut self) {
        *self = match *self {
            Cell::Dead => Cell::Alive,
            Cell::Alive => Cell::Dead,
        };
    }
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

//METHOD EXPORTED TO JAVASCRIPT
#[wasm_bindgen]
impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_nightbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;

        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);

                count += self.cells[idx] as u8;
            }
        }
        count
    }

    pub fn new() -> Universe {
        utils::set_panic_hook();

        let width = 256;
        let height = 128;

        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_nightbors = self.live_nightbor_count(row, col);

                let next_cell = match (cell, live_nightbors) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead, //If less than 2 nieghtbor => Dead
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive, //if 2 or 3 neighbors => keep  alive
                    (Cell::Alive, x) if x > 3 => Cell::Dead, //if more than 3 neighbors => Dead
                    (Cell::Dead, 3) => Cell::Alive, //if dead and exactly 3 neighbors => Reproduction (alive)
                    (otherwise, _) => otherwise,    //nochange
                };
                next[idx] = next_cell;
            }
        }
        self.cells = next;
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = (0..width * self.height).map(|_i| Cell::Dead).collect();
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = (0..self.width * height).map(|_i| Cell::Dead).collect();
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn toggle_cell(&mut self, row: u32, col: u32) {
        let idx = self.get_index(row, col);
        self.cells[idx].toggle();
    }

    pub fn create_lightship(&mut self, row: u32, col: u32) {
        //Idx is the center of the ship
        //PATTERN Dead/Alive
        //DADDA
        //ADDDD
        //ADDDA
        //AAAAD

        //HARDCODED TO SEE THE SCHEME

        //L1
        let idx = self.get_index(row - 1, col - 2);
        self.cells[idx] = Cell::Dead;
        let idx = self.get_index(row - 1, col - 1);
        self.cells[idx] = Cell::Alive;
        let idx = self.get_index(row - 1, col);
        self.cells[idx] = Cell::Dead;
        let idx = self.get_index(row - 1, col + 1);
        self.cells[idx] = Cell::Dead;
        let idx = self.get_index(row - 1, col + 2);
        self.cells[idx] = Cell::Alive;
        //L2
        let idx = self.get_index(row, col - 2);
        self.cells[idx] = Cell::Alive;
        let idx = self.get_index(row, col - 1);
        self.cells[idx] = Cell::Dead;
        let idx = self.get_index(row, col);
        self.cells[idx] = Cell::Dead;
        let idx = self.get_index(row, col + 1);
        self.cells[idx] = Cell::Dead;
        let idx = self.get_index(row, col + 2);
        self.cells[idx] = Cell::Dead;
        //L3
        let idx = self.get_index(row + 1, col - 2);
        self.cells[idx] = Cell::Alive;
        let idx = self.get_index(row + 1, col - 1);
        self.cells[idx] = Cell::Dead;
        let idx = self.get_index(row + 1, col);
        self.cells[idx] = Cell::Dead;
        let idx = self.get_index(row + 1, col + 1);
        self.cells[idx] = Cell::Dead;
        let idx = self.get_index(row + 1, col + 2);
        self.cells[idx] = Cell::Alive;
        //L4
        let idx = self.get_index(row + 2, col - 2);
        self.cells[idx] = Cell::Alive;
        let idx = self.get_index(row + 2, col - 1);
        self.cells[idx] = Cell::Alive;
        let idx = self.get_index(row + 2, col);
        self.cells[idx] = Cell::Alive;
        let idx = self.get_index(row + 2, col + 1);
        self.cells[idx] = Cell::Alive;
        let idx = self.get_index(row + 2, col + 2);
        self.cells[idx] = Cell::Dead;
    }
}

impl Universe {
    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }

    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells[idx] = Cell::Alive;
        }
    }
}

use std::fmt;

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n");
        }
        Ok(())
    }
}
