use std::{num::ParseIntError, ops::Range, str::FromStr};

use crate::rules::{ColumnRule, RowRule, Rule};

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
    fn set_cell(&mut self, n: u16, index: usize) {}
}

impl FromStr for Sudoku {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sudoku = Sudoku::new(9, vec![Box::new(RowRule), Box::new(ColumnRule)]);

        for (index, part) in s.split(',').enumerate() {
            let n = part.parse()?;
            if n != 0 {
                sudoku.set_cell(n, index);
            }
        }

        Ok(sudoku)
    }
}

pub struct Cell {
    pub available: Vec<u16>,
}

impl Cell {
    fn new_with_range(range: Range<u16>) -> Self {
        Self {
            available: range.collect(),
        }
    }
}
