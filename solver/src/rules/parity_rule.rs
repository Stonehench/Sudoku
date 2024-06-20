// This file is all the logic and tests pertaining to the Parity rule
// Author Peter s224759
use super::{DynRule, Rule};
use crate::sudoku::Sudoku;
use rand::random;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct ParityRule {
    pub parity_clue: Vec<(usize, usize)>,
}

impl ParityRule {
    pub fn new(parity_clue: Vec<(usize, usize)>) -> DynRule {
        DynRule(Box::new(ParityRule { parity_clue }))
    }
}

impl Rule for ParityRule {
    // the updates function does not affect aything for domino rules 
    // therefore it cleans and returns an empty buffer
    fn updates<'buf>(
        &self,
        _size: usize,
        _index: usize,
        buffer: &'buf mut Vec<usize>,
    ) -> &'buf [usize] {
        buffer.clear();
        buffer
    }

    // The multi remove function handles all the rules logic for the parity rule
    fn multi_remove<'buf>(
        &self,
        sudoku: &Sudoku,
        big_buffer: &'buf mut Vec<(u16, usize)>,
    ) -> &'buf [(u16, usize)] {
        big_buffer.clear();
        for (left_index, right_index) in &self.parity_clue {
            if sudoku.cells[*left_index].locked_in && !sudoku.cells[*right_index].locked_in {
                // For all left indices, if the value is known
                let value = sudoku.cells[*left_index].available[0];
                // If even, remove the even numbers from the other half
                if value & 1 == 0 {
                    for i in 1..=(sudoku.size as u16 / 2) {
                        if sudoku.cells[*right_index].available.contains(&(i * 2)) {
                            big_buffer.push((i * 2, *right_index));
                        }
                    }
                // else if odd, remove all the odd numbers from the other half
                } else {
                    for i in 0..=(sudoku.size as u16 / 2) {
                        if sudoku.cells[*right_index].available.contains(&(i * 2 + 1)) {
                            big_buffer.push(((i * 2 + 1), *right_index));
                        }
                    }
                }
            // for right indices, if the value is known
            } else if sudoku.cells[*right_index].locked_in && !sudoku.cells[*left_index].locked_in {
                let value = sudoku.cells[*right_index].available[0];
                // If even, remove the even numbers from the other half
                if value & 1 == 0 {
                    for i in 1..=(sudoku.size as u16 / 2) {
                        if sudoku.cells[*left_index].available.contains(&(i * 2)) {
                            big_buffer.push((i * 2, *left_index));
                        }
                    }
                 // else if odd, remove all the odd numbers from the other half
                } else if value & 1 == 1 {
                    for i in 0..=(sudoku.size as u16 / 2) {
                        if sudoku.cells[*left_index].available.contains(&(i * 2 + 1)) {
                            big_buffer.push(((i * 2 + 1), *left_index));
                        }
                    }
                }
            } else if !sudoku.cells[*left_index].locked_in && !sudoku.cells[*right_index].locked_in
            // if the values is not known
            {
                // test if all avalible values in a cell has the same parity
                // if so the other half must only have the other parity
                let mut same_parity_left = true;
                if sudoku.cells[*left_index].available[0] & 1 == 0 && same_parity_left {
                    for i in sudoku.cells[*left_index].available.iter() {
                        if *i & 1 != 0 {
                            same_parity_left = false;
                            break;
                        }
                    }
                    if same_parity_left {
                        for i in 1..=(sudoku.size as u16 / 2) {
                            if sudoku.cells[*right_index].available.contains(&(i * 2)) {
                                big_buffer.push((i * 2, *right_index));
                            }
                        }
                    }
                } else if sudoku.cells[*left_index].available[0] & 1 == 1 && same_parity_left {
                    for i in sudoku.cells[*left_index].available.iter() {
                        if *i & 1 != 1 {
                            same_parity_left = false;
                            break;
                        }
                    }
                    if same_parity_left {
                        for i in 0..=(sudoku.size as u16 / 2) {
                            if sudoku.cells[*right_index].available.contains(&(i * 2 + 1)) {
                                big_buffer.push(((i * 2 + 1), *right_index));
                            }
                        }
                    }
                }
                let mut same_parity_right = true;
                if sudoku.cells[*right_index].available[0] & 1 == 0
                    && same_parity_right
                    && !same_parity_left
                {
                    for i in sudoku.cells[*right_index].available.iter() {
                        if *i & 1 != 0 {
                            same_parity_right = false;
                            break;
                        }
                    }
                    if same_parity_right {
                        for i in 1..=(sudoku.size as u16 / 2) {
                            if sudoku.cells[*left_index].available.contains(&(i * 2)) {
                                big_buffer.push((i * 2, *left_index));
                            }
                        }
                    }
                } else if sudoku.cells[*right_index].available[0] & 1 == 1
                    && same_parity_right
                    && !same_parity_left
                {
                    for i in sudoku.cells[*right_index].available.iter() {
                        if *i & 1 != 1 {
                            same_parity_right = false;
                            break;
                        }
                    }
                    if same_parity_right {
                        for i in 0..=(sudoku.size as u16 / 2) {
                            if sudoku.cells[*left_index].available.contains(&(i * 2 + 1)) {
                                big_buffer.push(((i * 2 + 1), *left_index));
                            }
                        }
                    }
                }
            }
        }
        big_buffer
    }

    fn create_clue(&mut self, cells: &Vec<crate::sudoku::Cell>, size: usize) {
        for index in 0..cells.len() {
            if let Some(current) = cells[index].available.get(0) {
                if index + 1 >= cells.len() {
                    continue;
                }
                if let Some(right) = cells[index + 1].available.get(0) {
                    if ((current & 1) == 0 && (right & 1) != 0)
                        || ((current & 1) != 0 && (right & 1) == 0)
                    {
                        // parity rule should have (current , right)
                        self.parity_clue.push((index, index + 1));
                    }
                }
                if index + size >= cells.len() {
                    continue;
                }
                if let Some(below) = cells[index + size].available.get(0) {
                    if (current & 1 == 0 && below & 1 != 0) || (current & 1 != 0 && below & 1 == 0)
                    {
                        // parity rule should have (index , below)
                        self.parity_clue.push((index, index + size));
                    }
                }
            }
        }
        let count = self.parity_clue.len();
        if count > size * 2 {
            for i in 0..count - (size * 3)/2 {
                self.parity_clue.remove(random::<usize>() % (count - i));
            }
        }
    }

    fn boxed_clone(&self) -> DynRule {
        DynRule(Box::new(self.clone()))
    }

    fn get_name(&self) -> &'static str {
        "ParityRule"
    }

    fn to_parity_rule(&mut self) -> Option<&mut ParityRule> {
        Some(self)
    }

    fn no_of_clues(&self) -> usize {
        return self.parity_clue.len();
    }
    fn print_self(&self) -> bool {
        print!("ParityRule");
        for (x,y) in &self.parity_clue {
            print!(" ; {x},{y}")
        }
        true
    }
}

//########################### TEST ###############################

#[test]
fn parity_multi_remove_test() {
    use crate::sudoku::Sudoku;
    let mut big_buffer = vec![];
    /* The test sudoku a 4 x 4
    =================
    ‖   | 1 Ø   |   ‖
    -----------------
    ‖   |   ‖   |   ‖
    ==Ø==============
    ‖ 1 |   ‖   |   ‖
    -----------------
    ‖   |   ‖   |   ‖
    =================
    */

    let parity_rule = ParityRule {
        parity_clue: vec![(1 as usize, 2 as usize), (4 as usize, 8 as usize)],
    };
    let mut sudoku = Sudoku::new(
        4,
        vec![
            super::square_rule::SquareRule::new(),
            parity_rule.boxed_clone(),
        ],
    );

    sudoku.cells[1] = crate::sudoku::Cell::single(1);
    sudoku.cells[8] = crate::sudoku::Cell::single(1);
    let res = parity_rule.multi_remove(&sudoku, &mut big_buffer);

    assert_eq!(res, vec![(1, 2), (3, 2), (1, 4), (3, 4)].as_slice());

    /* The test sudoku a 4 x 4
    =================
    ‖   | 1 Ø   |   ‖
    -----------------
    ‖   |   ‖ 3 |   ‖
    ==Ø==============
    ‖ 1 |   ‖   |   ‖
    -----------------
    ‖   |   ‖   |   ‖
    */
    sudoku.set_cell(3, 6).unwrap();
    let res = parity_rule.multi_remove(&sudoku, &mut big_buffer);

    // Only needs to remove 1 odd
    assert_eq!(res, vec![(1, 2), (1, 4)].as_slice())
}

#[test]
fn extended_parity_multi_remove_test() {
    use crate::sudoku::Sudoku;
    let mut big_buffer = vec![];
    /* The test sudoku a 4 x 4
    =================
    ‖   |   Ø   |   ‖
    -----------------
    ‖   | 2 ‖   |   ‖
    ==Ø==============
    ‖   | 4 ‖ 2 |   ‖
    -----------------
    ‖   |   ‖   |   ‖
    =================
    */

    let parity_rule = ParityRule {
        parity_clue: vec![(1 as usize, 2 as usize), (4 as usize, 8 as usize)],
    };
    let mut sudoku = Sudoku::new(
        4,
        vec![
            super::square_rule::SquareRule::new(),
            parity_rule.boxed_clone(),
        ],
    );

    sudoku.set_cell(2, 5).unwrap();
    sudoku.set_cell(4, 9).unwrap();
    sudoku.set_cell(2, 10).unwrap();
    let res = parity_rule.multi_remove(&sudoku, &mut big_buffer);

    assert_eq!(res, vec![(1, 2), (3, 2), (1, 4), (3, 4)].as_slice());
}
