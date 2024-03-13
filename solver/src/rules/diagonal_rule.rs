use super::Rule;
use allocator_api2::vec::Vec as AlloVec;
use bumpalo::Bump;
use integer_sqrt::IntegerSquareRoot;
use std::cell::RefCell;
use std::{fmt::Debug, str::FromStr};

use crate::sudoku::{DynRule, Sudoku};

#[derive(Debug, Clone)]
pub struct DiagonalRule;

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
                if !sudoku.cells[position].locked_in {
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
                if !sudoku.cells[position].locked_in {
                    return Some((value, position));
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
        let sub_s = sudoku.size.integer_sqrt();

        // keep track of wether or not a possible candidate has been found in the box
        let mut candidate_found: bool;

        // TODO this only works if the square rule is also a part of the ruleset
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
        None
    }
    fn boxed_clone(&self) -> DynRule {
        Box::new(self.clone())
    }

    fn get_name(&self) -> &'static str {
        "DiagonalRule"
    }
}
