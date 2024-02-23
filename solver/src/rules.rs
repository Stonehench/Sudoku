use std::fmt::Debug;

use crate::sudoku::Sudoku;

pub trait Rule: Debug {
    fn updates(&self, sudoku: &Sudoku, index: usize) -> Vec<usize>;
    fn is_legal(&self, sudoku: &Sudoku, index: usize, value: u16) -> bool;
}


#[derive(Debug)]
pub struct SquareRule;

impl Rule for SquareRule {
    fn updates(&self, sudoku: &Sudoku, index: usize) -> Vec<usize> {
        let row = index / sudoku.size;

        (0..sudoku.size)
            .map(|i| (index - (index % 3)) - (9 * (row % 3)) + (i % 3) + (9 * (i/3)))
            .filter(|i| *i != index)
            .collect()
    }

    fn is_legal(&self, sudoku: &Sudoku, index: usize, value: u16) -> bool {
        let row = index / sudoku.size;
        todo!()
    }
}

#[derive(Debug)]
pub struct RowRule;

impl Rule for RowRule {
    fn updates(&self, sudoku: &Sudoku, index: usize) -> Vec<usize> {
        let column = index / sudoku.size;
        (0..sudoku.size)
            .map(|i| i + column * sudoku.size)
            .filter(|i| *i != index)
            .collect()
    }

    fn is_legal(&self, sudoku: &Sudoku, index: usize, value: u16) -> bool {
        let column = index / sudoku.size;
        todo!()
    }
}

#[derive(Debug)]
pub struct ColumnRule;

impl Rule for ColumnRule {
    fn updates(&self, sudoku: &Sudoku, index: usize) -> Vec<usize> {
        let row = index % sudoku.size;
        (0..sudoku.size)
            .map(|i| i * sudoku.size + row)
            .filter(|i| *i != index)
            .collect()
    }

    fn is_legal(&self, sudoku: &Sudoku, index: usize, value: u16) -> bool {
        todo!()
    }
}

#[test]
fn row_test() {
    let sudoku = Sudoku::new(9, vec![]);

    let rowrule = RowRule;
    let indexes = rowrule.updates(&sudoku, 11);
    println!("{indexes:?}");

    assert_eq!(indexes, vec![9, 10, 12, 13, 14, 15, 16, 17])
}

#[test]
fn column_test() {
    let sudoku = Sudoku::new(9, vec![]);

    let columnrule = ColumnRule;
    let indexes = columnrule.updates(&sudoku, 11);
    println!("{indexes:?}");

    assert_eq!(indexes, vec![2, 20, 29, 38, 47, 56, 65, 74])
}
