use crate::sudoku::Sudoku;

pub trait Rule {
    fn updates(&self, sudoku: &Sudoku, index: usize) -> Vec<usize>;
    fn is_legal(&self, sudoku: &Sudoku, index: usize, value: u16) -> bool;
}

pub struct RowRule;

impl Rule for RowRule {
    fn updates(&self, sudoku: &Sudoku, index: usize) -> Vec<usize> {
        let column = index / sudoku.size;
        (0..sudoku.size)
            .map(|i| i + column)
            .filter(|i| *i != index)
            .collect()
    }

    fn is_legal(&self, sudoku: &Sudoku, index: usize, value: u16) -> bool {
        todo!()
    }
}

pub struct ColumnRule;

impl Rule for ColumnRule {
    fn updates(&self, sudoku: &Sudoku, index: usize) -> Vec<usize> {
        let row = index % sudoku.size;
        (0..sudoku.size)
            .map(|i| i + row)
            .filter(|i| *i != index)
            .collect()
    }

    fn is_legal(&self, sudoku: &Sudoku, index: usize, value: u16) -> bool {
        todo!()
    }
}
