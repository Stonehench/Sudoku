use super::{DynRule, Rule};
use bumpalo::Bump;
use std::fmt::Debug;

use crate::sudoku::Sudoku;

#[derive(Debug, Clone)]
pub struct ConsecutiveRule {
    pub consecutive_clue: Vec<(usize, usize)>,
}

impl ConsecutiveRule {
    pub fn new(x_clue: Vec<(usize, usize)>) -> DynRule {
        DynRule(Box::new(ConsecutiveRule { x_clue }))
    }
}

impl Rule for ConsecutiveRule {
    fn updates<'buf>(
        &self,
        _size: usize,
        _index: usize,
        buffer: &'buf mut Vec<usize>,
    ) -> &'buf [usize] {
        buffer.clear();

        // TODO: 

        buffer
    }

    fn hidden_singles(&self, sudoku: &Sudoku) -> Option<(u16, usize)> {
        // TODO: hidden singles should not really do anything
        // if for every 
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
        "ConsecutiveRule"
    }
}

//########################### TEST ###############################


#[test]
fn consecutive_update_test() {}

#[test]
fn consecutive_hidden() {
        let consecutive_rule = ConsecutiveRule {
            consecutive_clue: vec![(1 as usize, 2 as usize)],
        };
        let mut sudoku = Sudoku::new(
            4,
            vec![super::square_rule::SquareRule::new(), consecutive_rule.boxed_clone()],
        );

        sudoku.set_cell(1, 1).unwrap();

        let res = consecutive_rule.hidden_singles(&sudoku);
        assert_eq!(res, Some((2, 2)));


        let mut sudoku = Sudoku::new(
            4,
            vec![super::square_rule::SquareRule::new(), consecutive_rule.boxed_clone()],
        );

        sudoku.set_cell(3, 1).unwrap();
        sudoku.set_cell(4, 4).unwrap();

        let res = consecutive_rule.hidden_singles(&sudoku);
        assert_eq!(res, Some((2, 2)))
}

#[test]
fn locked_consecutive_candidate() {

/* The test sudoku a 4 x 4
=================
‖   | 1 O   |   ‖
-----------------
‖   |   ‖   |   ‖
==O==============
‖ 1 |   ‖   |   ‖
-----------------
‖   |   ‖   |   ‖
=================
*/

    let consecutive_rule = ConsecutiveRule {
        consecutive_clue: vec![(1 as usize, 2 as usize),(4 as usize, 8 as usize)],
    };
    let mut sudoku = Sudoku::new(
        4,
        vec![super::square_rule::SquareRule::new(), consecutive_rule.boxed_clone()],
    );

    sudoku.set_cell(1, 1).unwrap();
    sudoku.set_cell(1, 8).unwrap();
    let res = consecutive_rule.hidden_singles(&sudoku);

    // locked candidates should return that there can not be 3 in either of the dominos (index 2 and 4)
    // because 3 is not consecutive with 1
    assert_eq!(res, Some(3, (vec![2, 4].as_slice())));

/* The test sudoku a 4 x 4
=================
‖   | 1 O   |   ‖
-----------------
‖   |   ‖ 3 |   ‖
==O==============
‖ 1 |   ‖   |   ‖
-----------------
‖   |   ‖   |   ‖
=================
*/
    sudoku.set_cell(3, 6).unwrap();
    let res = consecutive_rule.hidden_singles(&sudoku);

    // locked candidates should return that there can not be 4 in either of the dominos (index 2 and 4)
    // because 4 is not consecutive with 1
    assert_eq!(res, Some(4, (vec![2, 4].as_slice())));
}
