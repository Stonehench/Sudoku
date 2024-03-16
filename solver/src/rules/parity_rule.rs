use super::{DynRule, Rule};
use bumpalo::Bump;
use std::fmt::Debug;

use crate::sudoku::Sudoku;

#[derive(Debug, Clone)]
pub struct ParityRule {
    pub parity_clue: Vec<(usize, usize)>,
}

impl ParityRule {
    pub fn new(x_clue: Vec<(usize, usize)>) -> DynRule {
        DynRule(Box::new(ParityRule { x_clue }))
    }
}

impl Rule for ParityRule {
    fn updates<'buf>(
        &self,
        _size: usize,
        _index: usize,
        buffer: &'buf mut Vec<usize>,
    ) -> &'buf [usize] {
        buffer.clear();

        // TODO: the other half should only have the opposite patiry left
        // There will have to be done some logic in the solver

        buffer
    }

    fn hidden_singles(&self, sudoku: &Sudoku) -> Option<(u16, usize)> {
        // TODO: hidden singles should not really do anything
        // if 
        None
    }

    fn locked_candidate<'buf>(
        &self,
        sudoku: &Sudoku,
        buffer: &'buf mut Vec<usize>,
        _arena: &mut Bump,
    ) -> Option<(u16, &'buf [usize])> {
        // TODO:
        None
    }

    fn boxed_clone(&self) -> DynRule {
        DynRule(Box::new(self.clone()))
    }

    fn get_name(&self) -> &'static str {
        "ParityRule"
    }
}

//########################### TEST ###############################


#[test]
fn parity_update_test() {
        let parity_rule = ParityRule {
            parity_clue: vec![(1 as usize, 2 as usize)],
        };
        let mut sudoku = Sudoku::new(
            4,
            vec![super::square_rule::SquareRule::new(), parity_rule.boxed_clone()],
        );

        sudoku.set_cell(1, 1).unwrap();
        let res = x_rule.hidden_singles(&sudoku);

        // suggestion there could be created an enum that can differentiate even and odd
        // This might have to be a locked candidate kinda this
        // TODO: even???
        assert_eq!(res, Some(Even, (vec![2].as_slice())))
}

#[test]
fn parity_hidden() {
        let parity_rule = ParityRule {
            parity_clue: vec![(1 as usize, 2 as usize)],
        };
        let mut sudoku = Sudoku::new(
            4,
            vec![super::square_rule::SquareRule::new(), x_rule.boxed_clone()],
        );

        sudoku.set_cell(1, 1).unwrap();
        println!("{sudoku}");

        let res = x_rule.hidden_singles(&sudoku);
        assert_eq!(res, Some((4, 2)))
}

#[test]
fn locked_parity_candidate() {
    // TODO: 
}
