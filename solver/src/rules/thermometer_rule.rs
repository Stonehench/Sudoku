use super::{DynRule, Rule};
use std::fmt::Debug;

use crate::sudoku::Sudoku;

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
            for (enumeration, index) in themometer.iter().enumerate() {
                // if the next element on the zipper is 2 this element must be 1
                if !sudoku.cells[*index].locked_in
                    && enumeration + 1 < themometer.len()
                    && index == &themometer[0]
                    && sudoku.cells[themometer[enumeration + 1]].locked_in
                    && sudoku.cells[themometer[enumeration + 1]].available[0] == 2
                    && sudoku.cells[*index].available.contains(&1)
                {
                    return Some(((1), *index));
                }

                // if the previous element is one less than sudoku.size this element is sudoku.size
                if !sudoku.cells[*index].locked_in
                    && enumeration > 0
                    && index == themometer.last().unwrap()
                    && sudoku.cells[themometer[enumeration - 1]].locked_in
                    && sudoku.cells[themometer[enumeration - 1]].available[0]
                        == sudoku.size as u16 - 1
                    && sudoku.cells[*index]
                        .available
                        .contains(&(sudoku.size as u16))
                {
                    return Some(((sudoku.size as u16), *index));
                }

                // if two indecies are surrounding one index are locked in with only one possible value left
                // it is basically a naked single so yeah...
                if !sudoku.cells[*index].locked_in
                    && enumeration < themometer.len() - 1
                    && enumeration > 0
                {
                    let prev_index = themometer[enumeration - 1];
                    let next_index = themometer[enumeration + 1];
                    if sudoku.cells[next_index].locked_in
                        && sudoku.cells[prev_index].locked_in
                        && sudoku.cells[next_index].available[0]
                            >= sudoku.cells[prev_index].available[0]
                        && sudoku.cells[next_index].available[0]
                            - sudoku.cells[prev_index].available[0]
                            == 2
                    {
                        return Some((
                            (sudoku.cells[themometer[enumeration - 1]].available[0] + 1),
                            *index,
                        ));
                    }
                }
            }
        }
        None
    }

    fn needs_square_for_locked(&self) -> bool {
        // there are no locked, so false
        false
    }

    fn multi_remove<'buf>(
        &self,
        sudoku: &Sudoku,
        big_buffer: &'buf mut Vec<(u16, usize)>,
    ) -> &'buf [(u16, usize)] {
        big_buffer.clear();

        for themometer in &self.themometer_clue {
            for (enumeration, index) in themometer.into_iter().enumerate() {
                if !sudoku.cells[*index].locked_in {
                    for value in 1..(enumeration + 1) as u16 {
                        if sudoku.cells[*index].available.contains(&value) {
                            big_buffer.push((value, *index));
                        }
                    }

                    for value in (sudoku.size - (themometer.len() - enumeration) + 2) as u16
                        ..(sudoku.size + 1) as u16
                    {
                        if sudoku.cells[*index].available.contains(&value) {
                            big_buffer.push((value, *index));
                        }
                    }
                }

                if sudoku.cells[*index].locked_in {
                    if let Some(value) = sudoku.cells[*index].available.get(0) {
                        for (inner_enumeration, inner_index) in themometer.into_iter().enumerate() {
                            if inner_enumeration > enumeration
                                && !sudoku.cells[*inner_index].locked_in
                            {
                                for i in 1..*value + 1 {
                                    if sudoku.cells[*inner_index].available.contains(&i) {
                                        big_buffer.push((i, *inner_index))
                                    }
                                }
                            } else if inner_enumeration < enumeration
                                && !sudoku.cells[*inner_index].locked_in
                            {
                                for i in *value..=sudoku.size as u16 {
                                    if sudoku.cells[*inner_index].available.contains(&i) {
                                        big_buffer.push((i, *inner_index))
                                    }
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

    let mut themometer_rule = ThermometerRule {
        themometer_clue: vec![vec![0 as usize, 1 as usize, 2 as usize, 3 as usize]],
    };

    let mut big_buffer = vec![];
    let mut indexes = themometer_rule.multi_remove(&sudoku, &mut big_buffer);

    // first run the multi-remove on an empty thermometer
    for (value, index) in indexes {
        sudoku.cells[*index].available.retain(|i| *i != *value);
    }
    assert_eq!(
        indexes,
        vec![
            (7, 0),
            (8, 0),
            (9, 0),
            (1, 1),
            (8, 1),
            (9, 1),
            (1, 2),
            (2, 2),
            (9, 2),
            (1, 3),
            (2, 3),
            (3, 3)
        ]
    );

    // set a cell on the thermometer
    sudoku.set_cell(3, 2).unwrap();
    indexes = themometer_rule.multi_remove(&sudoku, &mut big_buffer);

    assert_eq!(
        indexes,
        vec![(4, 0), (5, 0), (6, 0), (4, 1), (5, 1), (6, 1), (7, 1),]
    );

    sudoku = Sudoku::new(9, vec![]);
    sudoku.set_cell(3, 2).unwrap();

    println!("");
    themometer_rule = ThermometerRule {
        themometer_clue: vec![vec![3 as usize, 2 as usize, 1 as usize, 0 as usize]],
    };

    big_buffer.clear();
    let indexes = themometer_rule.multi_remove(&sudoku, &mut big_buffer);
    assert_eq!(
        indexes,
        vec![
            (7, 3),
            (8, 3),
            (9, 3),
            (4, 3),
            (5, 3),
            (6, 3),
            (7, 3),
            (8, 3),
            (9, 3),
            (1, 1),
            (2, 1),
            (1, 0),
            (2, 0),
            (1, 1),
            (2, 1),
            (9, 1),
            (1, 0),
            (2, 0)
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