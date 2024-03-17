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
        // TODO: in a cell where only othe parity remains, the opposite parity should only remain in the other cell
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
fn parity_update_test() {}

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
            parity_clue: vec![(1 as usize, 2 as usize),(4 as usize, 8 as usize)],
        };
        let mut sudoku = Sudoku::new(
            4,
            vec![super::square_rule::SquareRule::new(), parity_rule.boxed_clone()],
        );

        sudoku.set_cell(1, 1).unwrap();
        sudoku.set_cell(1, 8).unwrap();
        let res = x_rule.hidden_singles(&sudoku);

        assert_eq!(res, Some(3, (vec![2, 8].as_slice())));

/* The test sudoku a 4 x 4
=================
‖   | 1 Ø   |   ‖
-----------------
‖   |   ‖ 3 |   ‖
==Ø==============
‖ 1 |   ‖   |   ‖
-----------------
‖   |   ‖   |   ‖
=================
*/
        sudoku.set_cell(3, 6).unwrap();
        let res = x_rule.hidden_singles(&sudoku);
        
        // No odd numbers remain in the availible, so None should be retunded
        assert_eq!(res, None)
}
