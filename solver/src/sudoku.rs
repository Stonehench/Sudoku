use std::ops::Range;

use crate::rules::Rule;

pub struct Sudoku {
    pub sizex: usize,
    pub sizey: usize,
    pub cells: Vec<Cell>,
    pub rules: Vec<Box<dyn Rule>>,
}

impl Sudoku {
    pub fn new(sizex: usize, sizey: usize, rules: Vec<Box<dyn Rule>>) -> Self {
        let range = if sizex == sizey {
            0..(sizex as u16)
        } else {
            todo!("Hvad skal der være i cellerne når jeg ikke kan mat?")
        };

        Sudoku {
            sizex,
            sizey,
            cells: (0..sizex * sizey)
                .map(|_| Cell::new_with_range(range.clone()))
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
