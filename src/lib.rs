mod utils;

#[cfg(debug_assertions)]
use utils::{log, Timer};

use std::fmt;

use wasm_bindgen::prelude::*;

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
    }
  }
}

#[wasm_bindgen]
pub struct Universe {
  width: u32,
  height: u32,
  cells: Vec<Cell>,
}

impl Universe {
  fn get_index(&self, row: u32, column: u32) -> usize {
    (row * self.width + column) as usize // flat array
  }

  fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
    let mut count = 0;

    for dx in [self.width - 1, 0, 1] {
      for dy in [self.height - 1, 0, 1] {
        if dy == 0 && dx == 0 {
          continue;
        }

        let n_row = (row + dy) % self.height;
        let n_col = (column + dx) % self.width;
        let idx = self.get_index(n_row, n_col);
        count += self.cells[idx] as u8;
      }
    }
    count
  }
}

#[wasm_bindgen]
impl Universe {
  pub fn new() -> Universe {
    utils::set_panic_hook();

    let width = 64;
    let height = 64;

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

  pub fn width(&self) -> u32 {
    self.width
  }

  /// NOTE: the width and height setters reset the universe
  pub fn set_width(&mut self, width: u32) {
    self.width = width;
    self.cells = (0..width * self.height).map(|_i| Cell::Dead).collect();
  }

  pub fn height(&self) -> u32 {
    self.height
  }

  /// NOTE: the width and height setters reset the universe
  pub fn set_height(&mut self, height: u32) {
    self.height = height;
    self.cells = (0..self.width * height).map(|_i| Cell::Dead).collect();
  }

  /// NOTE: this is part of the wasm api.
  /// to get a reference, use [`Self::get_cells()`]
  pub fn cells(&self) -> *const Cell {
    self.cells.as_ptr()
  }

  pub fn toggle_cell(&mut self, row: u32, col: u32) {
    let idx = self.get_index(row, col);
    self.cells[idx].toggle();
  }

  pub fn tick(&mut self) {
    #[cfg(debug_assertions)]
    let _timer = Timer::new("Universe::tick");
    let mut next = self.cells.clone();

    for row in 0..self.height {
      for col in 0..self.width {
        let idx = self.get_index(row, col);
        let cell = self.cells[idx];
        let live_neighbors = self.live_neighbor_count(row, col);

        #[cfg(debug_assertions)]
        log!("cell[{row}:{col}] initially {cell:?} with {live_neighbors} neighbors alive");

        let n_cell = match (cell, live_neighbors) {
          (Cell::Alive, x) if x < 2 => Cell::Dead, // underpopulation
          (Cell::Alive, x) if x > 3 => Cell::Dead, // overpopulation
          (Cell::Dead, 3) => Cell::Alive,          // reproduction
          (otherwise, _) => otherwise,
        };

        #[cfg(debug_assertions)]
        log!("cell[{row}:{col}] becomes {n_cell:?}");
        next[idx] = n_cell;
      }
    }
    self.cells = next;
  }
}

impl fmt::Display for Universe {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    for line in self.get_cells().chunks(self.width as usize) {
      for &cell in line {
        let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
        write!(f, "{symbol}")?;
      }
      write!(f, "\n")?;
    }
    Ok(())
  }
}

impl Universe {
  pub fn get_cells(&self) -> &[Cell] {
    &self.cells
  }

  pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
    for &(row, col) in cells {
      let idx = self.get_index(row, col);
      self.cells[idx] = Cell::Alive;
    }
  }
}
