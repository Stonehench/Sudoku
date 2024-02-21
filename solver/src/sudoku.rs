pub struct Sudoku {
    sizex: usize,
    sizey: usize,
    cells: Vec<Cell>,
    rules: Vec<RuleInstance>
}

pub struct Cell {
    available: Vec<u16>,
}

pub trait Rule {
    fn updates(&self, sudoku: &Sudoku, index: usize) -> Vec<usize>;
}

pub struct RuleInstance(Box<dyn Rule>);
