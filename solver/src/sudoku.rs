use crate::rules::Rule;

pub struct Sudoku {
    sizex: usize,
    sizey: usize,
    cells: Vec<Cell>,
    rules: Vec<Box<dyn Rule>>,
}

pub struct Cell {
    available: Vec<u16>,
}

