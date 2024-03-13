use super::Rule;
use allocator_api2::vec::Vec as AlloVec;
use bumpalo::Bump;
use integer_sqrt::IntegerSquareRoot;
use std::cell::RefCell;
use std::fmt::Debug;

use crate::sudoku::{DynRule, Sudoku};

#[derive(Debug, Clone)]

pub struct RowRule {
    pub has_locked: RefCell<Option<bool>>,
}
impl RowRule {
    pub fn new() -> Box<dyn Rule + Send> {
        Box::new(RowRule {
            has_locked: RefCell::new(None),
        })
    }
}
impl Rule for RowRule {
    fn updates<'buf>(
        &self,
        size: usize,
        index: usize,
        buffer: &'buf mut Vec<usize>,
    ) -> &'buf [usize] {
        buffer.clear();
        let row = index / size;

        for i in (0..size).map(|i| i + row * size) {
            buffer.push(i)
        }
        buffer
    }

    fn hidden_singles(&self, sudoku: &Sudoku) -> Option<(u16, usize)> {
        for row_number in 0..sudoku.size {
            'value: for value in 1..=sudoku.size as u16 {
                let mut found_position = None;
                for position in (0..sudoku.size).map(|i| i + row_number * sudoku.size) {
                    if sudoku.cells[position].available.contains(&value) {
                        if found_position.is_some() {
                            continue 'value;
                        } else {
                            found_position = Some(position);
                        }
                    }
                }
                if let Some(position) = found_position {
                    if !sudoku.cells[position].locked_in {
                        return Some((value, position));
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
        arena: &mut Bump,
    ) -> Option<(u16, &'buf [usize])> {
        // locked candidate only really applies when square rule is in the ruleset

        //This NEEDS to be on a different line, since it has to drop the borrow BEFORE matching.
        let has_locked = *self.has_locked.borrow();

        match has_locked {
            None => {
                let has_squares = sudoku.rules.iter().any(|r| r.get_name() == "SquareRule");
                *self.has_locked.borrow_mut() = Some(has_squares);
                if !has_squares {
                    return None;
                }
            }
            Some(false) => return None,
            Some(true) => {}
        }
        //Hey kat hvis du har lyst til at bruge found_column position kan du gøre det nu!!
        arena.reset();

        let mut box_indecies: AlloVec<usize, &Bump> = AlloVec::with_capacity_in(100, &arena);

        let mut candidate_found: bool;
        let sub_s = sudoku.size.integer_sqrt();
        let mut row;

        // this is almost identical to the implementation for Coulumn go there for explanatory comments
        for position in
            (0..sudoku.size).map(|i| i * sub_s + (sudoku.size * (sub_s - 1) * (i / sub_s)))
        {
            // reset all values from previous box
            box_indecies.clear();

            // calculate the current box indecies
            // I'm not sure if this is the absolute best place to use the Allocated vector
            // but there for sure is potential
            for i in (0..sudoku.size).map(|i| position + (i % sub_s) + (sudoku.size * (i / sub_s)))
            {
                box_indecies.push(i);
            }

            for value in 1..=sudoku.size as u16 {
                // looking for a new candidate clear any old data
                'sub_r: for sub_row in 0..sub_s {
                    candidate_found = false;
                    buffer.clear();

                    row = (position / sudoku.size) + sub_row;

                    // this loop searces each cell in a given box to check if the value is present in the avaibleble
                    for &box_pos in box_indecies.iter() {
                        if box_pos / sudoku.size % sub_s != sub_row
                            && sudoku.cells[box_pos].available.contains(&value)
                        {
                            //println!("Found value {value} outside sub {sub_row} real row {row} at pos {box_pos} box {position}");
                            continue 'sub_r;
                        } else if box_pos / sudoku.size % sub_s == sub_row
                            && sudoku.cells[box_pos].available.contains(&value)
                            && !sudoku.cells[box_pos].locked_in
                        {
                            //println!("Candidate {value} in sub {sub_row} real row {row} at pos {box_pos} box {position}");
                            candidate_found = true;
                        }
                    }

                    if candidate_found {
                        // push the indexes outside of the box to the buffer
                        for remove_index in (0..(sudoku.size))
                            .map(|i| i + (sudoku.size * row)) // indexes of the row
                            .filter(|index| !box_indecies.contains(index))
                        // but not in the box
                        {
                            if sudoku.cells[remove_index].available.contains(&value) {
                                buffer.push(remove_index) // only push indexes that contain the value
                            }
                        }
                        if !buffer.is_empty() {
                            return Some((value, buffer));
                        }
                    }
                }
            }
        }
        None
    }
    fn boxed_clone(&self) -> DynRule {
        Box::new(self.clone())
    }

    fn get_name(&self) -> &'static str {
        "RowRule"
    }
}

//########################### TEST ###############################

#[test]
fn locked_row_candidate() {
    let mut sudoku = Sudoku::new(9, vec![Box::new(crate::rules::square_rule::SquareRule)]);
    let row_rule = RowRule::new();

    sudoku.set_cell(1, 9).unwrap();
    sudoku.set_cell(8, 18).unwrap();
    sudoku.set_cell(3, 10).unwrap();
    sudoku.set_cell(4, 11).unwrap();
    sudoku.set_cell(5, 19).unwrap();
    sudoku.set_cell(7, 20).unwrap();
    let mut buffer = vec![];
    let mut arena = Bump::new();
    let mut res = row_rule.locked_candidate(&sudoku, &mut buffer, &mut arena);

    assert_eq!(res, Some((2, vec![3, 4, 5, 6, 7, 8].as_slice())));

    sudoku = Sudoku::new(9, vec![Box::new(crate::rules::square_rule::SquareRule)]);

    sudoku.set_cell(1, 60).unwrap();
    sudoku.set_cell(8, 61).unwrap();
    sudoku.set_cell(3, 78).unwrap();
    sudoku.set_cell(4, 79).unwrap();
    sudoku.set_cell(2, 44).unwrap();

    println!("{sudoku}");
    res = row_rule.locked_candidate(&sudoku, &mut buffer, &mut arena);

    assert_eq!(res, Some((2, vec![63, 64, 65, 66, 67, 68].as_slice())))
}

#[test]
fn row_hidden_math_test() {
    let mut sudoku = Sudoku::new(9, vec![Box::new(crate::rules::square_rule::SquareRule)]);

    sudoku.set_cell(2, 1).unwrap();
    sudoku.set_cell(1, 56).unwrap();
    sudoku.set_cell(1, 12).unwrap();
    sudoku.set_cell(1, 24).unwrap();

    println!("{sudoku}");

    let rowrule = RowRule::new();
    let res = rowrule.hidden_singles(&sudoku);
    assert_eq!(res, Some((1, 0)))
}

#[test]
fn row_test() {
    let sudoku = Sudoku::new(9, vec![]);

    let rowrule = RowRule::new();
    let mut buffer = vec![];
    let indexes = rowrule.updates(sudoku.size, 11, &mut buffer);
    println!("{indexes:?}");

    assert_eq!(indexes, vec![9, 10, 11, 12, 13, 14, 15, 16, 17])
}
