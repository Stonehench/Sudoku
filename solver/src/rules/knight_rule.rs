// This file is all the logic and tests pertaining to the Knight rule
// Author Katinka s224805

use super::{DynRule, Rule};
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct KnightRule;

impl KnightRule {
    pub fn new() -> DynRule {
        DynRule(Box::new(Self))
    }
}

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

        // . 1 . 2 .
        // 3 . . . 4
        // . . X . .
        // 5 . . . 6
        // . 7 . 8 .


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
        DynRule(Box::new(self.clone()))
    }

    fn get_name(&self) -> &'static str {
        "KnightRule"
    }
    fn print_self(&self) -> bool {
        print!("KnightsMove");
        true
    }
}

//########################### TEST ###############################

#[test]
fn knight_test() {
    let sudoku = crate::sudoku::Sudoku::new(9, vec![]);

    let knightrule = KnightRule;
    let mut buffer = vec![];
    let indexes = knightrule.updates(sudoku.size, 11, &mut buffer);
    assert_eq!(indexes, vec![0, 4, 18, 22, 28, 30]);

    let indexes = knightrule.updates(sudoku.size, 40, &mut buffer);
    assert_eq!(indexes, vec![21, 23, 29, 33, 47, 51, 57, 59]);

    let indexes = knightrule.updates(9, 0, &mut buffer);
    assert_eq!(indexes, vec![11, 19]);

    let indexes = knightrule.updates(16, 0, &mut buffer);
    assert_eq!(indexes, vec![18, 33]);

    let indexes = knightrule.updates(16, 255, &mut buffer);
    assert_eq!(indexes, vec![(254 - 16 - 16), (253 - 16)]);

    let indexes = knightrule.updates(4, 0, &mut buffer);
    assert_eq!(indexes, vec![6, 9]);

    let indexes = knightrule.updates(4, 15, &mut buffer);
    assert_eq!(indexes, vec![(14 - 8), (13 - 4)]);

    let indexes = knightrule.updates(4, 3, &mut buffer);
    assert_eq!(indexes, vec![5, 10]);

    let indexes = knightrule.updates(4, 12, &mut buffer);
    assert_eq!(indexes, vec![(13 - 8), (14 - 4)]);
}
