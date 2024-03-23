use super::{DynRule, Rule};
use bumpalo::Bump;
use std::fmt::Debug;

use crate::sudoku::Sudoku;

#[derive(Debug, Clone)]
pub struct ZipperRule {
    pub zipper_clue: Vec<(usize, Vec<(usize, usize)>)>, 
    // zippers are touples of (centerIndex, Vec<Index, Index>), 
    // such that the left part of the touple is the central index, and the value which the zipper should add to,
    // and the right apart of the touple us a vektor of sets on the zipper that ar equal lengths from the center
}

impl ZipperRule {
    pub fn new(zipper_clue: Vec<(usize, Vec<(usize, usize)>)>) -> DynRule {
        DynRule(Box::new(ZipperRule { zipper_clue }))
    }
}


impl Rule for ZipperRule {
    fn updates<'buf>(
        &self,
        size: usize,
        index: usize,
        buffer: &'buf mut Vec<usize>,
    ) -> &'buf [usize] {
        buffer.clear();

        // if the center of a zipper is set to a number, that number can no longer appear on the sipper,
        // since you would have to add it with 0 to get the center digit which does not make any sense as 0 is not a valid digit


        buffer
    }

    fn hidden_singles(&self, sudoku: &Sudoku) -> Option<(u16, usize)> {
        None
    }

    fn needs_square_for_locked(&self) -> bool {
        true
    }

    fn locked_candidate<'buf>(
        &self,
        sudoku: &Sudoku,
        buffer: &'buf mut Vec<usize>,
        arena: &mut Bump,
    ) -> Option<(u16, &'buf [usize])> {
        arena.reset();
        None
    }

    fn boxed_clone(&self) -> DynRule {
        DynRule(Box::new(self.clone()))
    }

    fn get_name(&self) -> &'static str {
        "ZipperRule"
    }
}

//########################### TEST ###############################

#[test]
fn locked_zipper_candidate() {

}

#[test]
fn zipper_test() {
    let sudoku = Sudoku::new(9, vec![]);

    let zipper_rule = ZipperRule {
        zipper_clue: vec![(1 as usize, vec![(0 as usize, 2 as usize)])],
    };

    let mut buffer = vec![];
    let indexes = zipper_rule.updates(sudoku.size, 11, &mut buffer);
    println!("{indexes:?}");

    assert_eq!(indexes, vec![0, 2])
}

#[test]
fn zipper_hidden_single_test() {

}
