pub struct Sudoku {
    sizex: usize,
    sizey: usize,
    cells: Vec<Cell>,
    rules: Vec<Box<dyn Rule>>,
}

pub struct Cell {
    available: Vec<u16>,
}

pub trait Rule {
    fn updates(&self, sudoku: &Sudoku, index: usize) -> Vec<usize>;
    fn is_legal(&self, sudoku: &Sudoku, index: usize, value: u16) -> bool;
}
