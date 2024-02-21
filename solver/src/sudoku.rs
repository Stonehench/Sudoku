use std::ops::Range;

use crate::rules::Rule;

pub struct Sudoku {
    pub size: usize,
    pub cells: Vec<Cell>,
    pub rules: Vec<Box<dyn Rule>>,
}

impl Sudoku {
    pub fn new(size: usize, rules: Vec<Box<dyn Rule>>) -> Self {
        Self {
            size,
            cells: (0..size * size)
                .map(|_| Cell::new_with_range(0..(size as u16)))
                .collect(),
            rules,
        }
    }
}

pub struct Cell {
    available: Vec<u16>,
}

impl Cell {
    fn new_with_range(range: Range<u16>) -> Self {
        Self {
            available: range.collect(),
        }
    }
}
