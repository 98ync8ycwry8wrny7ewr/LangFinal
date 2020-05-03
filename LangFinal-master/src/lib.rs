mod utils;

extern crate js_sys;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use std::fmt;
use web_sys::console;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen] // indicates that a function is exposed via Javascript
#[repr(u8)] // Forces each cell to be represteed by single byte
// Derivable copy/clone traits and comparision traits
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
	Dead = 0,
	Alive = 1,
}


#[wasm_bindgen]
pub struct GameBoard {
	width: u32,
	height: u32,
	cells: Vec<Cell>
}

#[wasm_bindgen]
impl GameBoard {

	fn find_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;

        // Check surrounding cells
        // Notice how it is both low level and functional
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.find_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }

    pub fn tick(&mut self) {


        let mut new_board = self.cells.clone();

        // .. used in common expand/to type syntax
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.find_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                // Rust uses matching instead of switch case
                // This is much cleaner
                let next_cell = match (cell, live_neighbors) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise,
                };

                new_board[idx] = next_cell;
            }
        }

        self.cells = new_board;
    }


    pub fn new(x: u32, y: u32, start_state: &js_sys::Set) -> GameBoard {

    	console::log_1(&"GameBoard Created From Within WASM".into());


    	// Gets size of array
    	let mut start_size: u32 = 0;
    	for x in start_state.keys() {
    		let elem = x.unwrap();
    		start_size += 1;
    		console::log_1(&elem.into());
    	}


        let width = x;
        let height = y;

        let cells = (0..width * height);
        let cells_size = width * height;
        console::log_1(&cells_size.into());
        console::log_1(&start_size.into());

        console::log_1(&cells.len().to_string().into());

        let cells = cells
            .map(|i| {
            	if i > (&cells_size / 2 ) {
            		Cell::Alive

            	} else {
            		Cell::Dead
            	}
            })
            .collect();

        GameBoard {
            width,
            height,
            cells,
        }
    }

    // Exposes width to JS
    pub fn width(&self) -> u32 {
    	self.width
    }

    // Exposes height to JS
    pub fn height(&self) -> u32 {
    	self.height
    }

    // Exposes cell memory pointer to CJ
    pub fn cells(&self) -> *const Cell {
    	self.cells.as_ptr()
    }
}

