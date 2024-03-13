use super::Rule;
use allocator_api2::vec::Vec as AlloVec;
use bumpalo::Bump;
use integer_sqrt::IntegerSquareRoot;
use std::cell::RefCell;
use std::{fmt::Debug, str::FromStr};

use crate::sudoku::{DynRule, Sudoku};

#[derive(Debug, Clone)]
pub struct XRule {
    pub x_clue: Vec<(usize, usize)>,
}

impl Rule for XRule {
    fn updates<'buf>(
        &self,
        _size: usize,
        _index: usize,
        buffer: &'buf mut Vec<usize>,
    ) -> &'buf [usize] {
        buffer.clear();

        // Doesen't really affect stuff???

        buffer
    }

    fn hidden_singles(&self, sudoku: &Sudoku) -> Option<(u16, usize)> {
        // is the index in the list of indexes that are part of X-clues

        // Either don't return anything
        // Or return the corrisponding index to the other half of X

        for (left_index, right_index) in &self.x_clue {
            if sudoku.cells[*left_index].locked_in && !sudoku.cells[*right_index].locked_in {
                if let Some(value) = sudoku.cells[*left_index].available.get(0) {
                    return Some((((sudoku.size + 1) as u16 - value), *right_index));
                }
            }
            if sudoku.cells[*right_index].locked_in && !sudoku.cells[*left_index].locked_in {
                if let Some(value) = sudoku.cells[*right_index].available.get(0) {
                    return Some((((sudoku.size + 1) as u16 - value), *left_index));
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
        //let mut found_candidate: Option<(u16, Vec<usize>)> = None;
        //let mut found_positions: Vec<usize> = vec![];
        // for all numbers in the sudoku
        // for all pairs in the X-clue
        for i in 1..(sudoku.size + 1) as u16 {
            //found_positions.clear();
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

    fn boxed_clone(&self) -> DynRule {
        Box::new(self.clone())
    }

    fn get_name(&self) -> &'static str {
        "XRule"
    }

    fn to_x_rule(&mut self) -> Option<&mut XRule> {
        Some(self)
    }
}
