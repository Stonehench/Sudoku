// This file is all the logic and tests pertaining to the X rule
// Author Katinka s224805 and Kasper s224776

use super::{DynRule, Rule};
use bumpalo::Bump;
use rand::random;
use std::fmt::Debug;

use crate::sudoku::Sudoku;

#[derive(Debug, Clone)]
pub struct XRule {
    pub x_clue: Vec<(usize, usize)>,
}

impl XRule {
    pub fn new(x_clue: Vec<(usize, usize)>) -> DynRule {
        DynRule(Box::new(XRule { x_clue }))
    }
}

impl Rule for XRule {
    fn updates<'buf>(
        &self,
        _size: usize,
        _index: usize,
        buffer: &'buf mut Vec<usize>,
    ) -> &'buf [usize] {
        // Updates for domino rules do not affect other cells
        buffer.clear();
        buffer
    }

    fn hidden_singles(&self, sudoku: &Sudoku) -> Option<(u16, usize)> {
        // is the index in the list of indexes that are part of X-clues

        // Either don't return anything
        // Or return the corrisponding index to the other half of X

        for (left_index, right_index) in &self.x_clue {
            // if the left side of the domino it locked in, the other half is a hidden single
            if sudoku.cells[*left_index].locked_in && !sudoku.cells[*right_index].locked_in {
                if let Some(value) = sudoku.cells[*left_index].available.get(0) {

                    if sudoku.cells[*right_index]
                        .available
                        .contains(&((sudoku.size + 1) as u16 - value))
                    {
                        return Some((((sudoku.size + 1) as u16 - value), *right_index));
                    }
                }
            }
            // if the right side of the domino it locked in, the other half is a hidden single
            if sudoku.cells[*right_index].locked_in && !sudoku.cells[*left_index].locked_in {
                if let Some(value) = sudoku.cells[*right_index].available.get(0) {

                    if sudoku.cells[*left_index]
                        .available
                        .contains(&((sudoku.size + 1) as u16 - value))
                    {
                        return Some((((sudoku.size + 1) as u16 - value), *left_index));
                    }
                }
            }
        }

        None
    }

    fn locked_candidate<'buf>(
        &self,
        sudoku: &Sudoku,
        buffer: &'buf mut Vec<usize>,
        _arena: &mut Bump,
    ) -> Option<(u16, &'buf [usize])> {
        // finds a locked candidate by checking all numbers and pairs in the x-clue
        // order of:
        // for all numbers in the sudoku 
        //    for all pairs in the X-clue
        for i in 1..(sudoku.size + 1) as u16 {
            buffer.clear();

            for (left_index, right_index) in &self.x_clue {
                // if neither side of the pair is locked in and the number is avalible in left but the counter part is not avalible in right
                if !sudoku.cells[*left_index].locked_in
                    && !sudoku.cells[*right_index].locked_in
                    && sudoku.cells[*left_index].available.contains(&i)
                    && !sudoku.cells[*right_index]
                        .available
                        .contains(&((sudoku.size + 1) as u16 - i))
                {
                    buffer.push(*left_index);
                }
                if !sudoku.cells[*left_index].locked_in
                    && !sudoku.cells[*right_index].locked_in
                    && sudoku.cells[*right_index].available.contains(&i)
                    && !sudoku.cells[*left_index]
                        .available
                        .contains(&((sudoku.size + 1) as u16 - i))
                {
                    buffer.push(*right_index);
                }
            }
            if !buffer.is_empty() {
                return Some((i, buffer));
            }
        }

        None
    }

    // The create clue function
    fn create_clue(&mut self, cells: &Vec<crate::sudoku::Cell>, size: usize) {

        // first find all the places where the x-rule is satisfied:
        for index in 0..cells.len() {
            if let Some(current) = cells[index].available.get(0) {
                if index + 1 >= cells.len() {
                    continue;
                }
                // check to the left 
                if let Some(left) = cells[index + 1].available.get(0) {
                    if current + left == size as u16 + 1 && (index + 1) % size != 0 {
                        // x rule should have (index , left)
                        self.x_clue.push((index, index + 1));
                    }
                }
                // if the bottom of the sudoku has been reached, do not check out of bounds
                if index + size >= cells.len() {
                    continue;
                }
                // check below
                if let Some(below) = cells[index + size].available.get(0) {
                    if current + below == size as u16 + 1 && index + size < cells.len() {
                        // x rule should have (index , below)
                        self.x_clue.push((index, index + size));
                    }
                }
            }
        }

        // randomly remove x-clues until only some remain
        let count = self.x_clue.len();
        if count > size * 2 {
            for i in 0..count - (size * 3) / 2 {
                self.x_clue.remove(random::<usize>() % (count - i));
            }
        }
    }

    fn boxed_clone(&self) -> DynRule {
        DynRule(Box::new(self.clone()))
    }

    fn get_name(&self) -> &'static str {
        "XRule"
    }

    fn to_x_rule(&mut self) -> Option<&mut XRule> {
        Some(self)
    }

    fn no_of_clues(&self) -> usize {
        return self.x_clue.len();
    }

    fn print_self(&self) -> bool {
        print!("XRule");
        for (x, y) in &self.x_clue {
            print!(" ;{x},{y}");
        }
        true
    }
}

//########################### TEST ###############################

#[test]
fn x_hidden_math_test() {
    let x_rule = XRule {
        x_clue: vec![(1 as usize, 2 as usize)],
    };
    let mut sudoku = Sudoku::new(
        4,
        vec![super::square_rule::SquareRule::new(), x_rule.boxed_clone()],
    );

    sudoku.set_cell(1, 1).unwrap();
    println!("{sudoku}");

    let res = x_rule.hidden_singles(&sudoku);
    // value , index
    assert_eq!(res, Some((4, 2)))
}

#[test]
fn locked_x_candidate() {
    let mut sudoku = Sudoku::new(4, vec![super::square_rule::SquareRule::new()]);
    let mut x_rule = XRule {
        x_clue: vec![(1 as usize, 2 as usize)],
    };

    sudoku.set_cell(1, 5).unwrap();
    println!("{sudoku}");
    let mut buffer = vec![];
    let mut arena = Bump::new();
    let res = x_rule.locked_candidate(&sudoku, &mut buffer, &mut arena);
    assert_eq!(res, Some((4, vec![2].as_slice())));

    sudoku = Sudoku::new(4, vec![super::square_rule::SquareRule::new()]);
    x_rule = XRule {
        x_clue: vec![(5 as usize, 6 as usize), (5 as usize, 9 as usize)],
    };

    sudoku.set_cell(1, 0).unwrap();
    println!("{sudoku}");

    let res = x_rule.locked_candidate(&sudoku, &mut buffer, &mut arena);
    assert_eq!(res, Some((4, vec![6, 9].as_slice())))
}
