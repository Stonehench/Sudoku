// This file is all the logic and tests pertaining to the X rule
// Author Katinka s224805

use super::{DynRule, Rule};
use crate::sudoku::Sudoku;
use bumpalo::Bump;
use integer_sqrt::IntegerSquareRoot;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct DiagonalRule;
impl DiagonalRule {
    pub fn new() -> DynRule {
        DynRule(Box::new(Self))
    }
}

impl Rule for DiagonalRule {
    fn updates<'buf>(
        &self,
        size: usize,
        index: usize,
        buffer: &'buf mut Vec<usize>,
    ) -> &'buf [usize] {
        buffer.clear();

        // Check if the index is on the first diagonal, and not on the second
        if index == 0
            || index == (size * size) - 1
            || !(index % (size - 1) == 0) && index % (size + 1) == 0
        {
            for i in (0..size).map(|i| i * (size + 1)) {
                buffer.push(i)
            }
        }

        // Check if the index is on the second diagonal, and not on the first
        if !(index % (size + 1) == 0) && index % (size - 1) == 0 {
            for i in (0..size).map(|i| (i + 1) * (size - 1)) {
                buffer.push(i)
            }
        }

        // In the rare case that the index is on the middle square in a sudoku of odd side-length
        if size % 2 == 1 && index == (size * size) / 2 {
            for i in (0..size).map(|i| i * (size + 1)) {
                buffer.push(i)
            }
            for i in (0..size).map(|i| (i + 1) * (size - 1)) {
                buffer.push(i)
            }
        }

        buffer
    }

    fn hidden_singles(&self, sudoku: &Sudoku) -> Option<(u16, usize)> {
        for value in 1..=sudoku.size as u16 {
            let mut found_position = None;

            // iterate over digonal from top left corner down
            for position in (0..sudoku.size).map(|i| i * (sudoku.size + 1)) {
                if sudoku.cells[position].available.contains(&value)
                    && !sudoku.cells[position].locked_in
                {
                    if found_position.is_some() {
                        found_position = None;
                        break;
                    } else {
                        found_position = Some(position);
                    }
                }
            }

            if let Some(position) = found_position {
                if !sudoku.cells[position].locked_in && sudoku.cells[position].available.contains(&value) {
                    return Some((value, position));
                }
            }

            // iterate over digonal from top right corner down
            for position in (0..sudoku.size).map(|i| (i + 1) * (sudoku.size - 1)) {
                if sudoku.cells[position].available.contains(&value)
                    && !sudoku.cells[position].locked_in
                {
                    if found_position.is_some() {
                        found_position = None;
                        break;
                    } else {
                        found_position = Some(position);
                    }
                }
            }

            if let Some(position) = found_position {
                if !sudoku.cells[position].locked_in && sudoku.cells[position].available.contains(&value){
                    return Some((value, position));
                }
            }
        }
        None
    }

    fn needs_square_for_locked(&self) -> bool {
        true
    }

    fn locked_candidate<'buf>(
        &self,
        sudoku: &Sudoku,
        buffer: &'buf mut Vec<usize>,
        _arena: &mut Bump,
    ) -> Option<(u16, &'buf [usize])> {
        let sub_s = sudoku.size.integer_sqrt();

        // keep track of wether or not a possible candidate has been found in the box
        let mut candidate_found: bool;

        // this only works if the square rule is also a part of the ruleset
        for value in 1..=sudoku.size as u16 {
            // look in the first diagonal
            // for there to be a locked candidate in a diagonal
            // all 'available' for a number in a box must be contained on the diagonal
            'find_box: for position in (0..sub_s).map(|i| (i * sub_s) * (sudoku.size + 1)) {
                candidate_found = false;
                buffer.clear();

                // calculate all indexes in the current box
                for box_pos in
                    (0..sudoku.size).map(|i| position + (i % sub_s) + (sudoku.size * (i / sub_s)))
                {
                    // if the box position is not on the diagonal and contains the value this is not a locked candidate
                    if box_pos % (sudoku.size + 1) != 0
                        && sudoku.cells[box_pos].available.contains(&value)
                    {
                        continue 'find_box;
                    // if the box position is on the diagonal and contains the value this, there is potential
                    } else if box_pos % (sudoku.size + 1) == 0
                        && sudoku.cells[box_pos].available.contains(&value)
                        && !sudoku.cells[box_pos].locked_in
                    {
                        candidate_found = true;
                    }
                }

                if candidate_found {
                    // push the indexes outside of the box to the buffer
                    // only indexes containing the value should be pushed

                    for remove_index in (0..(sudoku.size))
                        .map(|i| i * (sudoku.size + 1)) // indexes of the diagonal
                        .filter(|index| {
                            index
                                - (sudoku.size * ((index / sudoku.size) % sub_s))
                                - (index % sub_s)
                                != position
                        })
                    // but not in the box
                    {
                        if sudoku.cells[remove_index].available.contains(&value) && !sudoku.cells[remove_index].locked_in {
                            // only push indexes that contain the value
                            buffer.push(remove_index)
                        }
                    }
                    if !buffer.is_empty() {
                        return Some((value, buffer));
                    }
                }
            }

            // look in the second diagonal
            'find_box: for position in (1..(sub_s + 1))
                .map(|i| ((i * sub_s) * (sudoku.size - 1)) - (sub_s - 1) * sudoku.size)
            {
                candidate_found = false;
                buffer.clear();

                for box_pos in
                    (0..sudoku.size).map(|i| position + (i % sub_s) + (sudoku.size * (i / sub_s)))
                {
                    // if the box position is not on the diagonal and contains the value this is not a locked candidate
                    if box_pos % (sudoku.size - 1) != 0
                        && sudoku.cells[box_pos].available.contains(&value)
                    {
                        continue 'find_box;
                    // if the box position is on the diagonal and contains the value this, there is potential
                    } else if box_pos % (sudoku.size - 1) == 0
                        && sudoku.cells[box_pos].available.contains(&value)
                        && !sudoku.cells[box_pos].locked_in
                    {
                        candidate_found = true;
                    }
                }
                // if something was found and the rest of the diagonal is not already empty
                if candidate_found {
                    for remove_index in (1..(sudoku.size + 1))
                        .map(|i| i * (sudoku.size - 1)) // indexes of the diagonal
                        .filter(|index| {
                            index
                                - (sudoku.size * ((index / sudoku.size) % sub_s))
                                - (index % sub_s)
                                != position
                        })
                    // but not in the box
                    {
                        if sudoku.cells[remove_index].available.contains(&value) && !sudoku.cells[remove_index].locked_in {
                            // only push indexes that contain the value
                            buffer.push(remove_index)
                        }
                    }

                    if !buffer.is_empty() {
                        return Some((value, buffer));
                    }
                }
            }
        }
        None
    }
    fn boxed_clone(&self) -> DynRule {
        DynRule(Box::new(self.clone()))
    }

    fn get_name(&self) -> &'static str {
        "DiagonalRule"
    }

    fn priority(&self) -> super::ExecutionPriority {
        super::ExecutionPriority::High
    }
    fn print_self(&self) -> bool {
        print!("DiagonalRule");
        true
    }
}

//########################### TEST ###############################

#[test]
fn locked_diagonal_candidate() {
    use crate::rules::square_rule::SquareRule;
    let mut sudoku = Sudoku::new(9, vec![SquareRule::new()]);
    let diagonal_rule = DiagonalRule::new();

    sudoku.set_cell(2, 1).unwrap();
    sudoku.set_cell(3, 2).unwrap();
    sudoku.set_cell(4, 9).unwrap();
    sudoku.set_cell(5, 11).unwrap();
    sudoku.set_cell(6, 18).unwrap();
    sudoku.set_cell(7, 19).unwrap();
    let mut buffer = vec![];
    let mut arena = Bump::new();
    let res = diagonal_rule.locked_candidate(&sudoku, &mut buffer, &mut arena);
    assert_eq!(res, Some((1, vec![30, 40, 50, 60, 70, 80].as_slice())));

    sudoku = Sudoku::new(9, vec![SquareRule::new()]);

    sudoku.set_cell(2, 6).unwrap();
    sudoku.set_cell(3, 7).unwrap();
    sudoku.set_cell(4, 15).unwrap();
    sudoku.set_cell(5, 17).unwrap();
    sudoku.set_cell(6, 25).unwrap();
    sudoku.set_cell(7, 26).unwrap();
    let mut buffer = vec![];
    let mut arena = Bump::new();
    let res = diagonal_rule.locked_candidate(&sudoku, &mut buffer, &mut arena);
    assert_eq!(res, Some((1, vec![32, 40, 48, 56, 64, 72].as_slice())));

    sudoku = Sudoku::new(4, vec![SquareRule::new()]);

    sudoku.set_cell(2, 8).unwrap();
    sudoku.set_cell(3, 13).unwrap();

    let mut buffer = vec![];
    let mut arena = Bump::new();
    let res = diagonal_rule.locked_candidate(&sudoku, &mut buffer, &mut arena);
    assert_eq!(res, Some((1, vec![3, 6].as_slice())));

    // a locked candidate in the middle square
    sudoku = Sudoku::new(9, vec![SquareRule::new()]);

    sudoku.set_cell(2, 30).unwrap();
    sudoku.set_cell(3, 31).unwrap();
    sudoku.set_cell(4, 39).unwrap();
    sudoku.set_cell(5, 41).unwrap();
    sudoku.set_cell(6, 50).unwrap();
    sudoku.set_cell(7, 49).unwrap();
    let mut buffer = vec![];
    let mut arena = Bump::new();
    let res = diagonal_rule.locked_candidate(&sudoku, &mut buffer, &mut arena);
    assert_eq!(res, Some((1, vec![8, 16, 24, 56, 64, 72].as_slice())));

    // locked candidate 16x16
    sudoku = Sudoku::new(16, vec![SquareRule::new()]);

    sudoku.set_cell(1, 68).unwrap();
    sudoku.set_cell(2, 69).unwrap();
    sudoku.set_cell(3, 70).unwrap();
    sudoku.set_cell(4, 71).unwrap();
    sudoku.set_cell(5, 84).unwrap();
    sudoku.set_cell(6, 86).unwrap();
    sudoku.set_cell(7, 87).unwrap();
    sudoku.set_cell(8, 100).unwrap();
    sudoku.set_cell(9, 101).unwrap();
    sudoku.set_cell(10, 103).unwrap();
    sudoku.set_cell(11, 116).unwrap();
    sudoku.set_cell(12, 117).unwrap();
    sudoku.set_cell(13, 118).unwrap();
    sudoku.set_cell(14, 119).unwrap();

    let mut buffer = vec![];
    let mut arena = Bump::new();
    let res = diagonal_rule.locked_candidate(&sudoku, &mut buffer, &mut arena);
    assert_eq!(
        res,
        Some((
            15,
            vec![0, 17, 34, 51, 136, 153, 170, 187, 204, 221, 238, 255].as_slice()
        ))
    );

    // locked candidate 16x16
    sudoku = Sudoku::new(16, vec![SquareRule::new()]);

    let res = diagonal_rule.locked_candidate(&sudoku, &mut buffer, &mut arena);
    assert_eq!(res, None);
}

#[test]
fn diagonal_test() {
    let diagonalrule = DiagonalRule::new();
    let mut buffer = vec![];

    // testing 9x9
    let mut indexes = diagonalrule.updates(9, 11, &mut buffer);
    assert_eq!(indexes, vec![]);
    indexes = diagonalrule.updates(9, 80, &mut buffer);
    assert_eq!(indexes, vec![0, 10, 20, 30, 40, 50, 60, 70, 80]);
    indexes = diagonalrule.updates(9, 0, &mut buffer);
    assert_eq!(indexes, vec![0, 10, 20, 30, 40, 50, 60, 70, 80]);
    indexes = diagonalrule.updates(9, 16, &mut buffer);
    assert_eq!(indexes, vec![8, 16, 24, 32, 40, 48, 56, 64, 72]);
    indexes = diagonalrule.updates(9, 40, &mut buffer);
    assert_eq!(
        indexes,
        vec![0, 10, 20, 30, 40, 50, 60, 70, 80, 8, 16, 24, 32, 40, 48, 56, 64, 72]
    );
    indexes = diagonalrule.updates(9, 41, &mut buffer);
    assert_eq!(indexes, vec![]);

    // testing 4x4
    indexes = diagonalrule.updates(4, 0, &mut buffer);
    assert_eq!(indexes, vec![0, 5, 10, 15]);
    indexes = diagonalrule.updates(4, 1, &mut buffer);
    assert_eq!(indexes, vec![]);
    indexes = diagonalrule.updates(4, 2, &mut buffer);
    assert_eq!(indexes, vec![]);
    indexes = diagonalrule.updates(4, 3, &mut buffer);
    assert_eq!(indexes, vec![3, 6, 9, 12]);
    indexes = diagonalrule.updates(4, 4, &mut buffer);
    assert_eq!(indexes, vec![]);
    indexes = diagonalrule.updates(4, 5, &mut buffer);
    assert_eq!(indexes, vec![0, 5, 10, 15]);
    indexes = diagonalrule.updates(4, 6, &mut buffer);
    assert_eq!(indexes, vec![3, 6, 9, 12]);
    indexes = diagonalrule.updates(4, 7, &mut buffer);
    assert_eq!(indexes, vec![]);
    indexes = diagonalrule.updates(4, 8, &mut buffer);
    assert_eq!(indexes, vec![]);
    indexes = diagonalrule.updates(4, 9, &mut buffer);
    assert_eq!(indexes, vec![3, 6, 9, 12]);
    indexes = diagonalrule.updates(4, 10, &mut buffer);
    assert_eq!(indexes, vec![0, 5, 10, 15]);
    indexes = diagonalrule.updates(4, 11, &mut buffer);
    assert_eq!(indexes, vec![]);
    indexes = diagonalrule.updates(4, 12, &mut buffer);
    assert_eq!(indexes, vec![3, 6, 9, 12]);
    indexes = diagonalrule.updates(4, 13, &mut buffer);
    assert_eq!(indexes, vec![]);
    indexes = diagonalrule.updates(4, 14, &mut buffer);
    assert_eq!(indexes, vec![]);
    indexes = diagonalrule.updates(4, 15, &mut buffer);
    assert_eq!(indexes, vec![0, 5, 10, 15]);

    // 16x16
    indexes = diagonalrule.updates(16, 0, &mut buffer);
    assert_eq!(
        indexes,
        vec![0, 17, 34, 51, 68, 85, 102, 119, 136, 153, 170, 187, 204, 221, 238, 255]
    );
    indexes = diagonalrule.updates(16, 255, &mut buffer);
    assert_eq!(
        indexes,
        vec![0, 17, 34, 51, 68, 85, 102, 119, 136, 153, 170, 187, 204, 221, 238, 255]
    );
    indexes = diagonalrule.updates(16, 15, &mut buffer);
    assert_eq!(
        indexes,
        vec![15, 30, 45, 60, 75, 90, 105, 120, 135, 150, 165, 180, 195, 210, 225, 240]
    );
    indexes = diagonalrule.updates(16, 240, &mut buffer);
    assert_eq!(
        indexes,
        vec![15, 30, 45, 60, 75, 90, 105, 120, 135, 150, 165, 180, 195, 210, 225, 240]
    );
    indexes = diagonalrule.updates(16, 111, &mut buffer);
    assert_eq!(indexes, vec![]);
}

#[test]
fn diagonal_hidden_math_test() {
    use crate::rules::square_rule::SquareRule;
    let mut sudoku = Sudoku::new(9, vec![SquareRule::new(), DiagonalRule::new()]);

    sudoku.set_cell(1, 27).unwrap();
    sudoku.set_cell(1, 39).unwrap();
    sudoku.set_cell(1, 78).unwrap();
    sudoku.set_cell(1, 55).unwrap();

    println!("{sudoku}");

    let diagonalrule = DiagonalRule::new();
    let res = diagonalrule.hidden_singles(&sudoku);
    assert_eq!(res, Some((1, 20)))
}
