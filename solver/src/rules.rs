use std::fmt::Debug;

use integer_sqrt::IntegerSquareRoot;

use crate::sudoku::Sudoku;

pub trait Rule: Debug {
    fn updates(&self, sudoku: &Sudoku, index: usize) -> Vec<usize>;
    fn is_legal(&self, sudoku: &Sudoku, index: usize, value: u16) -> bool {
        !self
            .updates(sudoku, index)
            .iter()
            .map(|i| &sudoku.cells[*i])
            .any(|c| c.is_single_eq(value))
    }
}

#[derive(Debug)]
pub struct SquareRule;

impl Rule for SquareRule {
    fn updates(&self, sudoku: &Sudoku, index: usize) -> Vec<usize> {
        //Burde gerne være ok med arbitær størrelse?
        let row = index / sudoku.size;
        let size = sudoku.size;
        let sub_size = sudoku.size.integer_sqrt();

        (0..sudoku.size)
            .map(|i| {
                (index - (index % sub_size)) - (size * (row % sub_size))
                    + (i % sub_size)
                    + (size * (i / sub_size))
            })
            .filter(|i| *i != index)
            .collect()
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

#[test]
fn square_test() {
    let sudoku = Sudoku::new(9, vec![]);

    let squarerule = SquareRule;
    let indexes = squarerule.updates(&sudoku, 11);
    println!("{indexes:?}");

    assert_eq!(indexes, vec![0, 1, 2, 9, 10, 18, 19, 20])
}
