use super::Rule;
use std::fmt::Debug;

use crate::sudoku::{DynRule, Sudoku};

#[derive(Debug, Clone)]
pub struct KnightRule;

impl Rule for KnightRule {
    fn updates<'buf>(
        &self,
        size: usize,
        index: usize,
        buffer: &'buf mut Vec<usize>,
    ) -> &'buf [usize] {
        buffer.clear();

        let isize = size as i64;
        let iindex = index as i64;

        let max_index = isize * isize - 1;

        let mut temp_index = iindex - 2 * isize - 1;

        // no matter the size of the sudoku you always get 8 cells
        // therefore the length of the returned buffer is always the same
        // if there is a way to do it in a loop or a range or something
        // fell free to change it
        // do remember that it migth give negative values out..

        if temp_index >= 0 && max_index > temp_index && temp_index / isize + 3 != iindex / isize {
            // is it faster to cast to usize or just do the math? I have no idea...
            buffer.push(temp_index as usize);
        } // 1

        temp_index = iindex - 2 * isize + 1;
        if temp_index >= 0 && max_index > temp_index && temp_index / isize + 1 != iindex / isize {
            buffer.push(temp_index as usize);
        } // 2

        temp_index = iindex - 1 * isize - 2;
        if temp_index >= 0 && max_index > temp_index && temp_index / isize != iindex / isize - 2 {
            buffer.push(temp_index as usize);
        } // 3

        temp_index = iindex - 1 * isize + 2;
        if temp_index >= 0 && max_index > temp_index && temp_index / isize != iindex / isize {
            buffer.push(temp_index as usize);
        } // 4

        temp_index = iindex + 1 * isize - 2;
        if temp_index >= 0 && max_index > temp_index && temp_index / isize != iindex / isize {
            buffer.push(temp_index as usize);
        } // 5

        temp_index = iindex + 1 * isize + 2;
        if temp_index >= 0 && max_index > temp_index && temp_index / isize != iindex / isize + 2 {
            buffer.push(temp_index as usize);
        } // 6

        temp_index = iindex + 2 * isize - 1;
        if temp_index >= 0 && max_index > temp_index && temp_index / isize != iindex / isize + 1 {
            buffer.push(temp_index as usize);
        } // 7

        temp_index = iindex + 2 * isize + 1;
        if temp_index >= 0 && max_index > temp_index && temp_index / isize != iindex / isize + 3 {
            buffer.push(temp_index as usize);
        } // 8

        buffer
    }

    fn boxed_clone(&self) -> DynRule {
        Box::new(self.clone())
    }

    fn get_name(&self) -> &'static str {
        "KnightRule"
    }
}


//########################### TEST ###############################


#[test]
fn knight_test() {
    let sudoku = Sudoku::new(9, vec![]);

    let knightrule = KnightRule;
    let mut buffer = vec![];
    let indexes = knightrule.updates(sudoku.size, 11, &mut buffer);
    println!("{indexes:?}");

    assert_eq!(indexes, vec![0, 4, 18, 22, 28, 30]);

    let indexes = knightrule.updates(sudoku.size, 40, &mut buffer);
    println!("{indexes:?}");

    assert_eq!(indexes, vec![21, 23, 29, 33, 47, 51, 57, 59])
}

