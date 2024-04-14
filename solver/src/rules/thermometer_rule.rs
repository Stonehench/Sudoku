use super::{DynRule, Rule};
use bumpalo::Bump;
use integer_sqrt::IntegerSquareRoot;
use std::fmt::Debug;

use crate::sudoku::{self, Sudoku};

#[derive(Debug, Clone)]
pub struct ThermometerRule {
    // vector of themometers contaning indexes in order
    pub themometer_clue: Vec<Vec<usize>>,
}

impl ThermometerRule {
    pub fn new(themometer_clue: Vec<Vec<usize>>) -> DynRule {
        DynRule(Box::new(ThermometerRule { themometer_clue }))
    }
}

impl Rule for ThermometerRule {
    fn updates<'buf>(
        &self,
        _size: usize,
        index: usize,
        buffer: &'buf mut Vec<usize>,
    ) -> &'buf [usize] {
        buffer.clear();

        for themometer in &self.themometer_clue {
            if themometer.into_iter().any(|e| e == &index) {
                for element in themometer {
                    buffer.push(*element);
                }
            }
        }
        buffer
    }

    fn hidden_singles(&self, sudoku: &Sudoku) -> Option<(u16, usize)> {
        for themometer in &self.themometer_clue {
            for (index, element) in themometer.iter().enumerate() {
                if !sudoku.cells[*element].locked_in
                    && element == &themometer[0]
                    && sudoku.cells[themometer[index + 1]].locked_in
                    && sudoku.cells[themometer[index + 1]].available[0] == 2
                {
                    return Some(((1), *element));
                } else if !sudoku.cells[*element].locked_in
                    && element == themometer.last().unwrap()
                    && sudoku.cells[themometer[index - 1]].locked_in
                    && sudoku.cells[themometer[index - 1]].available[0] == sudoku.size as u16 - 1
                {
                    return Some(((sudoku.size as u16), *element));
                } else if !sudoku.cells[*element].locked_in
                    && element != &themometer[0]
                    && element != themometer.last().unwrap()
                    && sudoku.cells[*element - 1].locked_in
                    && sudoku.cells[*element + 1].locked_in
                {
                    let previous = &sudoku.cells[themometer[element - 1]];
                    let next = &sudoku.cells[themometer[element + 1]];

                    if next.available[0] - previous.available[0] == 2 {
                        return Some(((previous.available[0] + 1), *element));
                    }
                }
            }
        }
        None
    }

    fn needs_square_for_locked(&self) -> bool {
        true
    }

    fn multi_remove<'buf>(
        &self,
        sudoku: &Sudoku,
        big_buffer: &'buf mut Vec<(u16, usize)>,
    ) -> &'buf [(u16, usize)] {
        big_buffer.clear();

        for themometer in &self.themometer_clue {
            for element in themometer {
                if sudoku.cells[*element].locked_in {
                    if let Some(value) = sudoku.cells[*element].available.get(0) {
                        for change in themometer {
                            if change < element {
                                for i in 1..*value {
                                    big_buffer.push((i, *change))
                                }
                            } else if change > element {
                                for i in *value + 1..=sudoku.size as u16 {
                                    big_buffer.push((i, *change))
                                }
                            }
                        }
                    }
                }
            }
        }

        big_buffer
    }

    fn boxed_clone(&self) -> DynRule {
        DynRule(Box::new(self.clone()))
    }

    fn get_name(&self) -> &'static str {
        "ThemometerRule"
    }

    fn to_thermometer_rule(&mut self) -> Option<&mut ThermometerRule> {
        Some(self)
    }
}

//########################### TEST ###############################

#[test]
fn themometer_test() {
    let sudoku = Sudoku::new(9, vec![]);

    let themometer_rule = ThermometerRule {
        themometer_clue: vec![vec![0 as usize, 1 as usize, 2 as usize]],
    };

    let mut buffer = vec![];
    let mut indexes = themometer_rule.updates(sudoku.size, 1, &mut buffer);

    assert_eq!(indexes, vec![0, 1, 2]);

    indexes = themometer_rule.updates(sudoku.size, 5, &mut buffer);

    assert_eq!(indexes, vec![]);
}

#[test]
fn themometer_multi_remove_test() {
    let mut sudoku = Sudoku::new(9, vec![]);

    let themometer_rule = ThermometerRule {
        themometer_clue: vec![vec![0 as usize, 1 as usize, 2 as usize, 3 as usize]],
    };

    sudoku.set_cell(3, 2).unwrap();

    let mut big_buffer = vec![];
    let mut indexes = themometer_rule.multi_remove(&sudoku, &mut big_buffer);

    assert_eq!(
        indexes,
        vec![
            (1, 0),
            (2, 0),
            (1, 1),
            (2, 1),
            (4, 3),
            (5, 3),
            (6, 3),
            (7, 3),
            (8, 3),
            (9, 3)
        ]
    );
}

#[test]
fn themometer_hidden_single_test() {
    // test 1
    let mut sudoku = Sudoku::new(9, vec![]);

    let themometer_rule = ThermometerRule {
        themometer_clue: vec![vec![0 as usize, 1 as usize, 2 as usize]],
    };

    sudoku.set_cell(1, 0).unwrap();
    sudoku.set_cell(3, 2).unwrap();

    let result = themometer_rule.hidden_singles(&sudoku);

    assert_eq!(result, Some((2, 1)));

    // test 2
    let mut sudoku = Sudoku::new(9, vec![]);

    let themometer_rule = ThermometerRule {
        themometer_clue: vec![vec![
            0 as usize, 1 as usize, 2 as usize, 3 as usize, 4 as usize,
        ]],
    };

    sudoku.set_cell(1, 0).unwrap();
    sudoku.set_cell(3, 1).unwrap();
    sudoku.set_cell(7, 3).unwrap();
    sudoku.set_cell(9, 4).unwrap();

    let result = themometer_rule.hidden_singles(&sudoku);

    assert_eq!(result, None);

    // test 3
    let mut sudoku = Sudoku::new(9, vec![]);

    let themometer_rule = ThermometerRule {
        themometer_clue: vec![vec![0 as usize, 1 as usize, 2 as usize]],
    };

    sudoku.set_cell(2, 1).unwrap();

    let result = themometer_rule.hidden_singles(&sudoku);

    assert_eq!(result, Some((1, 0)));

    // test 4
    let mut sudoku = Sudoku::new(9, vec![]);

    let themometer_rule = ThermometerRule {
        themometer_clue: vec![vec![0 as usize, 1 as usize, 2 as usize]],
    };

    sudoku.set_cell(1, 0).unwrap();
    sudoku.set_cell(8, 1).unwrap();

    let result = themometer_rule.hidden_singles(&sudoku);

    assert_eq!(result, Some((9, 2)));
}
