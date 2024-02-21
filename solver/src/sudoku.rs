use std::{fs, num::ParseIntError, ops::Range, process::Command, str::FromStr};

use crate::rules::{ColumnRule, RowRule, Rule};

#[derive(Debug)]
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
                .map(|_| Cell::new_with_range(1..(size as u16 + 1)))
                .collect(),
            rules,
        }
    }
    fn set_cell(&mut self, n: u16, index: usize) {
        self.cells[index] = Cell::single(n);
        for rule in &self.rules {
            for inner_index in rule.updates(&self, index) {
                self.cells[inner_index].remove(n);
            }
        }
    }
}

impl FromStr for Sudoku {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sudoku = Sudoku::new(9, vec![Box::new(RowRule), Box::new(ColumnRule)]);

        let (uløst, _løsning) = s.split_once("\n\n").unwrap();

        for (index, part) in uløst.split(',').map(str::trim).enumerate() {
            let n = part.parse()?;
            if n != 0 {
                sudoku.set_cell(n, index);
            }
        }

        Ok(sudoku)
    }
}
#[derive(Debug)]
pub struct Cell {
    pub available: Vec<u16>,
}

impl Cell {
    fn single(n: u16) -> Self {
        Self { available: vec![n] }
    }
    fn new_with_range(range: Range<u16>) -> Self {
        Self {
            available: range.collect(),
        }
    }
    fn remove(&mut self, n: u16) {
        self.available.retain(|i| *i != n);
    }
}

#[test]
fn read_file_test() {
    let file_str = fs::read_to_string("./sudoku2").unwrap();
    let sudoku: Sudoku = file_str.parse().unwrap();

    println!("{sudoku:#?}");
}
