use std::{ops::Range, str::FromStr};

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
}

impl FromStr for Sudoku {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let mut sudoku = Sudoku::new(9, vec![Box::new(RowRule), Box::new(ColumnRule)]);


        

        for part in s.split(',') {
            
        }


        todo!()
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
