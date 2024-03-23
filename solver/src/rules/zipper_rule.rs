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
        _size: usize,
        index: usize,
        buffer: &'buf mut Vec<usize>,
    ) -> &'buf [usize] {
        buffer.clear();

        // if the center of a zipper is set to a number, that number can no longer appear on the zipper,
        // since you would have to add it with 0 to get the center digit which does not make any sense as 0 is not a valid digit
        for (i, rest) in &self.zipper_clue{
            if *i == index {
                for (left, right) in rest {
                    buffer.push(*left);
                    buffer.push(*right);
                }
            }
        }

        buffer
    }

    fn hidden_singles(&self, sudoku: &Sudoku) -> Option<(u16, usize)> {
        // if the center is unknown, it could be a hidden sigle if two indecies at equal distance is filled

        // an index on the zipper might be a hidden single
        // if the center is known and the opposite side of the zipper is known

        for (center, rest) in &self.zipper_clue {
            if sudoku.cells[*center].locked_in {
                // if the center is known
                // loop through all the pairs and see if some half of a pair can be calculated
                for (left, right) in rest {
                    // the left side is known, calculate the right
                    if sudoku.cells[*left].locked_in && !sudoku.cells[*right].locked_in {
                        return Some((sudoku.cells[*center].available[0] - sudoku.cells[*left].available[0], *right));
                    }
                    // the right side is known, calculate the left
                    if sudoku.cells[*right].locked_in && !sudoku.cells[*left].locked_in {
                        return Some((sudoku.cells[*center].available[0] - sudoku.cells[*right].available[0], *left));
                    }

                }                    
            } else {
                // if the center is unknown
                // loop through all the pairs and see if the center can be calculated
                for (left, right) in rest {
                    if sudoku.cells[*left].locked_in && sudoku.cells[*right].locked_in {
                        return Some((sudoku.cells[*left].available[0] + sudoku.cells[*right].available[0], *center));
                    }
                } 
            }
        }

        None
    }

    fn needs_square_for_locked(&self) -> bool {
        false
    }

    fn locked_candidate<'buf>(
        &self,
        sudoku: &Sudoku,
        buffer: &'buf mut Vec<usize>,
        arena: &mut Bump,
    ) -> Option<(u16, &'buf [usize])> {
        arena.reset();

        // if the center is know, anything greater than or equal to the center value
        // is no longler possible

        // for values ei. from 0 .. 9
        for value in 1..(sudoku.size + 1) as u16{
            buffer.clear();

            // for each zipper 
            for (center, rest) in &self.zipper_clue {
                // if the center cell is known and the center value is less than or equal the candidate value
                if sudoku.cells[*center].locked_in && sudoku.cells[*center].available[0] <= value {                 
                    // for all pairs in the current zipper
                    for (left, right) in rest {
                        // left index contains the value
                        if sudoku.cells[*left].available.contains(&value) {
                            buffer.push(*left);
                        }
                        // right index contains the value
                        if sudoku.cells[*right].available.contains(&value) {
                            buffer.push(*right);
                        }
                    }
                }
            }
            if !buffer.is_empty() {
                return Some((value, buffer));
            }
        }


        // many more wierd deductive things can be done

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
    let mut sudoku = Sudoku::new(4, vec![super::square_rule::SquareRule::new()]);
    let mut zipper_rule = ZipperRule {
        zipper_clue: vec![(1 as usize, vec![(0 as usize, 2 as usize),(4 as usize, 3 as usize)])],
    };

    sudoku.set_cell(3, 1).unwrap();
    println!("{sudoku}");
    let mut buffer = vec![];
    let mut arena = Bump::new();
    let res = zipper_rule.locked_candidate(&sudoku, &mut buffer, &mut arena);
    assert_eq!(res, Some((4, vec![0, 2, 4, 3].as_slice())));

    sudoku = Sudoku::new(4, vec![super::square_rule::SquareRule::new()]);
    zipper_rule = ZipperRule {
        zipper_clue: vec![(1 as usize, vec![(0 as usize, 2 as usize),(4 as usize, 3 as usize)]), (15 as usize, vec![(11 as usize, 14 as usize),(7 as usize, 13 as usize)])],
    };

    sudoku.set_cell(3, 1).unwrap();
    sudoku.set_cell(3, 15).unwrap();
    println!("{sudoku}");

    let res = zipper_rule.locked_candidate(&sudoku, &mut buffer, &mut arena);
    assert_eq!(res, Some((4, vec![0, 2, 4, 3, 11, 14, 7, 13].as_slice())))
}

#[test]
fn zipper_test() {
    let sudoku = Sudoku::new(9, vec![]);

    let zipper_rule = ZipperRule {
        zipper_clue: vec![(1 as usize, vec![(0 as usize, 2 as usize)])],
    };

    let mut buffer = vec![];
    let indexes = zipper_rule.updates(sudoku.size, 1, &mut buffer);
    //println!("{indexes:?}");

    assert_eq!(indexes, vec![0, 2])
}

#[test]
fn zipper_hidden_single_test() {
    let zipper_rule = ZipperRule {
        zipper_clue: vec![(1 as usize, vec![(0 as usize, 2 as usize)])],
    };

    let mut sudoku = Sudoku::new(
        4,
        vec![super::square_rule::SquareRule::new(), zipper_rule.boxed_clone()],
    );

    sudoku.set_cell(4, 1).unwrap();
    sudoku.set_cell(1, 0).unwrap();
    //println!("{sudoku}");

    let res = zipper_rule.hidden_singles(&sudoku);
    // value , index
    assert_eq!(res, Some((3, 2)));

    sudoku = Sudoku::new(
        9,
        vec![super::square_rule::SquareRule::new(), zipper_rule.boxed_clone()],
    );

    let zipper_rule = ZipperRule {
        zipper_clue: vec![(1 as usize, vec![(0 as usize, 2 as usize),(9 as usize, 3 as usize)])],
    };
    sudoku.set_cell(4, 1).unwrap();
    sudoku.set_cell(1, 0).unwrap();
    sudoku.set_cell(3, 2).unwrap();
    sudoku.set_cell(2, 9).unwrap();
    //println!("{sudoku}");

    let res = zipper_rule.hidden_singles(&sudoku);
    // value , index
    assert_eq!(res, Some((2, 3)));
}
