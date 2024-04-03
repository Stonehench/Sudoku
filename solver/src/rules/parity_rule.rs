use super::{DynRule, Rule};
use bumpalo::Bump;
use std::fmt::Debug;

use crate::sudoku::Sudoku;

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
    fn updates<'buf>(
        &self,
        _size: usize,
        _index: usize,
        buffer: &'buf mut Vec<usize>,
    ) -> &'buf [usize] {
        buffer.clear();
        buffer
    }

    fn multi_remove<'buf>(
        &self,
        _sudoku: &Sudoku,
        _big_buffer: &'buf mut Vec<(u16, usize)>,
    ) -> &'buf [(u16, usize)] {
        _big_buffer.clear();
        for (left_index, right_index) in &self.parity_clue {
            if _sudoku.cells[*left_index].locked_in && !_sudoku.cells[*right_index].locked_in {
                if let Some(value) = _sudoku.cells[*left_index].available.get(0) {
                    if value & 1 == 0 {
                        for i in 1..=(_sudoku.size as u16 / 2) {
                            _big_buffer.push((i * 2, *right_index));
                        }
                    } else if value & 1 == 1 {
                        for i in 0..=(_sudoku.size as u16 / 2) {
                            _big_buffer.push(((i * 2 + 1), *right_index));
                        }
                    }
                }
            }
            if _sudoku.cells[*right_index].locked_in && !_sudoku.cells[*left_index].locked_in {
                if let Some(value) = _sudoku.cells[*right_index].available.get(0) {
                    if value & 1 == 0 {
                        for i in 1..=(_sudoku.size as u16 / 2) {
                            _big_buffer.push((i * 2, *left_index));
                        }
                    } else if value & 1 == 1 {
                        for i in 0..=(_sudoku.size as u16 / 2) {
                            _big_buffer.push(((i * 2 + 1), *left_index));
                        }
                    }
                }
            }
        }
        _big_buffer
    }

    fn hidden_singles(&self, sudoku: &Sudoku) -> Option<(u16, usize)> {
        None
    }

    fn locked_candidate<'buf>(
        &self,
        sudoku: &Sudoku,
        buffer: &'buf mut Vec<usize>,
        _arena: &mut Bump,
    ) -> Option<(u16, &'buf [usize])> {
        // TODO: in a cell where only one parity remains, the opposite parity should only remain in the other cell
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
fn parity_multi_remove_test() {
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

    sudoku.set_cell(1, 1).unwrap();
    sudoku.set_cell(1, 8).unwrap();
    let res = parity_rule.multi_remove(&sudoku, &mut big_buffer);

    assert_eq!(res, vec![(1, 2), (3, 2), (1,4), (3,4)].as_slice());

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
    assert_eq!(res, vec![(1,2), (1,4)].as_slice())
}

#[test]
fn locked_parity_candidate() {
    let mut buffer = vec![];
    let mut arena = Bump::new();
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

    sudoku.set_cell(1, 1).unwrap();
    sudoku.set_cell(1, 8).unwrap();
    let res = parity_rule.locked_candidate(&sudoku, &mut buffer, &mut arena);

    assert_eq!(res, Some((3, (vec![2, 8].as_slice()))));

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
    let res = parity_rule.locked_candidate(&sudoku, &mut buffer, &mut arena);

    // No odd numbers remain in the availible, so None should be retunded
    assert_eq!(res, None)
}
