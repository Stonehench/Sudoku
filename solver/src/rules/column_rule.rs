use super::Rule;
use allocator_api2::vec::Vec as AlloVec;
use bumpalo::Bump;
use integer_sqrt::IntegerSquareRoot;
use std::cell::RefCell;
use std::fmt::Debug;

use crate::sudoku::{DynRule, Sudoku};

#[derive(Debug, Clone)]
pub struct ColumnRule {
    pub has_locked: RefCell<Option<bool>>,
}
impl ColumnRule {
    pub fn new() -> Box<dyn Rule + Send> {
        Box::new(ColumnRule {
            has_locked: RefCell::new(None),
        })
    }
}

impl Rule for ColumnRule {
    fn updates<'buf>(
        &self,
        size: usize,
        index: usize,
        buffer: &'buf mut Vec<usize>,
    ) -> &'buf [usize] {
        buffer.clear();
        let column = index % size;
        let size = size;

        for i in (0..size).map(|i| i * size + column) {
            buffer.push(i)
        }
        buffer
    }

    fn hidden_singles(&self, sudoku: &Sudoku) -> Option<(u16, usize)> {
        for column_number in 0..sudoku.size {
            'value: for value in 1..=sudoku.size as u16 {
                let mut found_position = None;

                for position in (0..sudoku.size).map(|i| i * sudoku.size + column_number) {
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
        // There are certain patterns of available numbers that may all eliminate a certain cell

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
        arena.reset();

        let sub_s = sudoku.size.integer_sqrt();

        let mut locations: AlloVec<usize, &Bump> = AlloVec::with_capacity_in(sudoku.size, &arena);

        for value in 1..=sudoku.size as u16 {
            for sq_y in 0..sub_s {
                for sq_x in 0..sub_s {
                    // reset all values from previous square
                    locations.clear();

                    // Tests all cells in square if they contain value
                    for l_x in 0..sub_s {
                        for l_y in 0..sub_s {
                            let x = l_x + sq_x * sub_s;
                            let y = l_y + sq_y * sub_s;
                            let i = x + y * sudoku.size;

                            if sudoku.cells[i].available.contains(&value) {
                                locations.push(l_x);
                            }
                        }
                    }

                    //Tjek om alle er på samme column

                    //verticalt. De har alle samme x koordinat
                    if !locations.is_empty() && locations.iter().all(|l_x| *l_x == locations[0]) {
                        buffer.clear();
                        let x = locations[0] + sq_x * sub_s;

                        for y in (0..sudoku.size)
                            .filter(|y| *y < sq_y * sub_s || *y >= (sq_y + 1) * sub_s)
                        {
                            let i = x + y * sudoku.size;
                            let cell = &sudoku.cells[i];
                            if !cell.locked_in && cell.available.contains(&value) {
                                buffer.push(i);
                            }
                        }

                        if !buffer.is_empty() {
                            return Some((value, buffer));
                        }
                    }
                }
            }
        }

        // look through every column
        // for there to be a locked candidate in a colums
        // all 'available' for a number in a box must be contained in that column

        // first check the square, then remove from the column
        // find all the top right corners of squares

        /*
        let mut column;
        for position in
            (0..sudoku.size).map(|i| i * sub_s + (sudoku.size * (sub_s - 1) * (i / sub_s)))
        {
            // reset all values from previous box
            box_indecies.clear();

            // calculate the current box indecies
            for i in (0..sudoku.size).map(|i| position + (i % sub_s) + (sudoku.size * (i / sub_s)))
            {
                box_indecies.push(i);
            }
            for value in 1..=sudoku.size as u16 {
                // looking for a new candidate clear any old data

                'sub_c: for sub_column in 0..sub_s {
                    candidate_found = false;
                    buffer.clear();

                    // get the true column number
                    column = position % sudoku.size + sub_column;

                    for &box_pos in box_indecies.iter() {
                        // if the box position is not in the same sub_column and contains the value this is not a locked candidate
                        if box_pos % sub_s != sub_column
                            && sudoku.cells[box_pos].available.contains(&value)
                        {
                            continue 'sub_c;
                        // if the box position is in the same coolumn and contains the value this, there is potential
                        } else if box_pos % sub_s == sub_column
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
                            .map(|i| (i * sudoku.size) + column) // indexes of the column
                            .filter(|index| !box_indecies.contains(index))
                        // but not in the box
                        {
                            if sudoku.cells[remove_index].available.contains(&value) {
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
        }
         */
        None
    }

    fn boxed_clone(&self) -> DynRule {
        Box::new(self.clone())
    }

    fn get_name(&self) -> &'static str {
        "ColumnRule"
    }
}

//########################### TEST ###############################

#[test]
fn locked_column_candidate() {
    let mut sudoku = Sudoku::new(9, vec![Box::new(crate::rules::square_rule::SquareRule)]);
    let column_rule = ColumnRule::new();
    let mut buffer = vec![];
    let mut arena = Bump::new();

    sudoku.set_cell(1, 0).unwrap();
    sudoku.set_cell(2, 25).unwrap();
    sudoku.set_cell(3, 9).unwrap();
    sudoku.set_cell(4, 11).unwrap();
    sudoku.set_cell(5, 2).unwrap();
    sudoku.set_cell(7, 20).unwrap();

    let mut res = column_rule.locked_candidate(&sudoku, &mut buffer, &mut arena);
    assert_eq!(res, Some((2, vec![28, 37, 46, 55, 64, 73].as_slice())));

    sudoku = Sudoku::new(9, vec![Box::new(crate::rules::square_rule::SquareRule)]);
    sudoku.set_cell(1, 1).unwrap();
    sudoku.set_cell(2, 25).unwrap();
    sudoku.set_cell(3, 10).unwrap();
    sudoku.set_cell(4, 11).unwrap();
    sudoku.set_cell(5, 2).unwrap();
    sudoku.set_cell(7, 20).unwrap();
    buffer = vec![];
    arena = Bump::new();
    res = column_rule.locked_candidate(&sudoku, &mut buffer, &mut arena);

    assert_eq!(res, Some((2, vec![27, 36, 45, 54, 63, 72].as_slice())));

    sudoku.set_cell(2, 42).unwrap();
    sudoku.set_cell(2, 48).unwrap();
    res = column_rule.locked_candidate(&sudoku, &mut buffer, &mut arena);
    assert_eq!(res, Some((2, vec![27, 54, 63, 72].as_slice())))
}

#[test]
fn column_test() {
    let sudoku = Sudoku::new(9, vec![]);

    let columnrule = ColumnRule::new();
    let mut buffer = vec![];
    let indexes = columnrule.updates(sudoku.size, 11, &mut buffer);
    println!("{indexes:?}");

    assert_eq!(indexes, vec![2, 11, 20, 29, 38, 47, 56, 65, 74])
}

#[test]
fn column_hidden_math_test() {
    let mut sudoku = Sudoku::new(9, vec![Box::new(crate::rules::square_rule::SquareRule)]);

    sudoku.set_cell(2, 9).unwrap();
    sudoku.set_cell(1, 24).unwrap();
    sudoku.set_cell(1, 28).unwrap();
    sudoku.set_cell(1, 56).unwrap();

    println!("\n\n{sudoku}");

    let columnrule = ColumnRule::new();
    let res = columnrule.hidden_singles(&sudoku);
    assert_eq!(res, Some((1, 0)))
}
