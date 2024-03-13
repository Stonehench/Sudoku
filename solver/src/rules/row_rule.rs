use super::Rule;
use allocator_api2::vec::Vec as AlloVec;
use bumpalo::Bump;
use integer_sqrt::IntegerSquareRoot;
use std::fmt::Debug;

use crate::sudoku::{DynRule, Sudoku};

#[derive(Debug, Clone)]

pub struct RowRule;
impl RowRule {
    pub fn new() -> Box<dyn Rule + Send> {
        Box::new(RowRule)
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

        let sub_s = sudoku.size.integer_sqrt();

        let mut locations: AlloVec<usize, &Bump> = AlloVec::with_capacity_in(sudoku.size, &arena);

        for value in 1..=sudoku.size as u16 {
            for sq_y in 0..sub_s {
                for sq_x in 0..sub_s {
                    locations.clear();

                    for l_x in 0..sub_s {
                        for l_y in 0..sub_s {
                            let x = l_x + sq_x * sub_s;
                            let y = l_y + sq_y * sub_s;
                            let i = x + y * sudoku.size;

                            if sudoku.cells[i].available.contains(&value) {
                                locations.push(l_y);
                            }
                        }
                    }

                    //Tjek om alle er på samme row

                    //verticalt. De har alle samme x koordinat
                    if !locations.is_empty() && locations.iter().all(|l_y| *l_y == locations[0]) {
                        buffer.clear();
                        let y = locations[0] + sq_y * sub_s;

                        for x in (0..sudoku.size)
                            .filter(|x| *x < sq_x * sub_s || *x >= (sq_x + 1) * sub_s)
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
