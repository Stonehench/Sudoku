use crate::sudoku::Sudoku;

pub trait Rule {
    fn updates(&self, sudoku: &Sudoku, index: usize) -> Vec<usize>;
    fn is_legal(&self, sudoku: &Sudoku, index: usize, value: u16) -> bool;
}

pub struct RowColumnRule;

impl Rule for RowColumnRule {
    fn updates(&self, sudoku: &Sudoku, index: usize) -> Vec<usize> {

        //let rowindex = index % sudoku.

        todo!()
    }

    fn is_legal(&self, sudoku: &Sudoku, index: usize, value: u16) -> bool {
        todo!()
    }
}