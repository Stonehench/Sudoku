use integer_sqrt::IntegerSquareRoot;
use std::{fmt::Debug, str::FromStr};

use crate::sudoku::{DynRule, Sudoku};

pub trait Rule: Debug {
    fn updates<'buf>(
        &self,
        size: usize,
        index: usize,
        buffer: &'buf mut Vec<usize>,
    ) -> &'buf [usize];
    fn is_legal(&self, sudoku: &Sudoku, index: usize, value: u16, buffer: &mut Vec<usize>) -> bool {
        !self
            .updates(sudoku.size, index, buffer)
            .iter()
            .map(|i| &sudoku.cells[*i])
            .any(|c| c.is_single_eq(value))
    }

    fn hidden_singles(&self, sudoku: &Sudoku) -> Option<(u16, usize)>;
    fn boxed_clone(&self) -> DynRule;
    fn get_name(&self) -> &'static str;

    fn to_x_rule(& mut self) -> Option<& mut XRule> {
        None
    }
}

impl FromStr for DynRule {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SquareRule" => Ok(Box::new(SquareRule)),
            "KnightsMove" => Ok(Box::new(KnightRule)),
            "DiagonalRule" => Ok(Box::new(DiagonalRule)),
            _ => {
                let mut rule_params = s.split(';').map(str::trim);
                match rule_params.next() {
                    Some("XRule") => Ok(Box::new(XRule {
                        x_clue: rule_params
                            .map(|s| {
                                let Some((l, r)) = s.split_once(',') else {
                                    return Err(format!("Failed to split {s} on ,"));
                                };
                                let l = l.parse().map_err(|e| format!("{e:?}"))?;
                                let r = r.parse().map_err(|e| format!("{e:?}"))?;

                                Ok((l, r))
                            })
                            .collect::<Result<_, _>>()?,
                    })),
                    _ => return Err(s.to_owned()),
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct SquareRule;

impl Rule for SquareRule {
    fn updates<'buf>(
        &self,
        size: usize,
        index: usize,
        buffer: &'buf mut Vec<usize>,
    ) -> &'buf [usize] {
        buffer.clear();

        let sub_s = size.integer_sqrt();

        let target_x = index % size;
        let target_y = index / size;
        let sq_x = target_x / sub_s;
        let sq_y = target_y / sub_s;

        for l_y in 0..sub_s {
            for l_x in 0..sub_s {
                let x = l_x + sq_x * sub_s;
                let y = l_y + sq_y * sub_s;
                let i = x + y * size;
                buffer.push(i);
            }
        }
        buffer
    }

    fn hidden_singles(&self, sudoku: &Sudoku) -> Option<(u16, usize)> {
        let sub_s = sudoku.size.integer_sqrt();
        for sq_y in 0..sub_s {
            for sq_x in 0..sub_s {
                'value: for value in 1..=sudoku.size as u16 {
                    let mut found_position = None;
                    for l_y in 0..sub_s {
                        for l_x in 0..sub_s {
                            let x = l_x + sq_x * sub_s;
                            let y = l_y + sq_y * sub_s;
                            let i = x + y * sudoku.size;
                            if sudoku.cells[i].available.contains(&value) {
                                if found_position.is_some() {
                                    continue 'value;
                                } else {
                                    found_position = Some(i);
                                }
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
        }
        None
    }
    fn boxed_clone(&self) -> DynRule {
        Box::new(self.clone())
    }

    fn get_name(&self) -> &'static str {
        "SquareRule"
    }
}
#[derive(Debug, Clone)]
pub struct RowRule;

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
    fn boxed_clone(&self) -> DynRule {
        Box::new(self.clone())
    }

    fn get_name(&self) -> &'static str {
        "RowRule"
    }
}

#[derive(Debug, Clone)]
pub struct ColumnRule;

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
    fn boxed_clone(&self) -> DynRule {
        Box::new(self.clone())
    }

    fn get_name(&self) -> &'static str {
        "ColumnRule"
    }
}

#[derive(Debug, Clone)]
pub struct XRule {
    pub x_clue: Vec<(usize, usize)>,
}

impl Rule for XRule {
    fn updates<'buf>(
        &self,
        _size: usize,
        _index: usize,
        buffer: &'buf mut Vec<usize>,
    ) -> &'buf [usize] {
        buffer.clear();

        // Doesen't really affect stuff???

        buffer
    }

    fn hidden_singles(&self, sudoku: &Sudoku) -> Option<(u16, usize)> {
        // is the index in the list of indexes that are part of X-clues

        // Either don't return anything
        // Or return the corrisponding index to the other half of X

        for (left_index, right_index) in &self.x_clue {
            if sudoku.cells[*left_index].locked_in && !sudoku.cells[*right_index].locked_in {
                if let Some(value) = sudoku.cells[*left_index].available.get(0) {
                    return Some((((sudoku.size + 1) as u16 - value), *right_index));
                }
            }
            if sudoku.cells[*right_index].locked_in && !sudoku.cells[*left_index].locked_in {
                if let Some(value) = sudoku.cells[*right_index].available.get(0) {
                    return Some((((sudoku.size + 1) as u16 - value), *left_index));
                }
            }
        }

        None
    }

    fn boxed_clone(&self) -> DynRule {
        Box::new(self.clone())
    }

    fn get_name(&self) -> &'static str {
        "XRule"
    }

    fn to_x_rule(& mut self) -> Option<& mut XRule> {
        Some(self)
    }
}

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
        if index == 0 || index == (size * size) - 1 || !(index % (size - 1) == 0) && index % (size + 1) == 0 {
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
        if size % 2 == 1 && index == (size *size) / 2{
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

        'value: for value in 1..=sudoku.size as u16 {
            let mut found_position = None;

            // iterate over digonal from top left corner down
            for position in (0..sudoku.size).map(|i| i * (sudoku.size + 1)) {
                if sudoku.cells[position].available.contains(&value) && !sudoku.cells[position].locked_in {
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
                if sudoku.cells[position].available.contains(&value) && !sudoku.cells[position].locked_in {
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
    fn boxed_clone(&self) -> DynRule {
        Box::new(self.clone())
    }

    fn get_name(&self) -> &'static str {
        "DiagonalRule"
    }
}

#[derive(Debug, Clone)]
pub struct KnightRule;

impl Rule for KnightRule {
    fn updates<'buf>(
        &self,
        size: usize,
        index: usize,
        buffer: &'buf mut Vec<usize>,
    ) -> &'buf [usize] {
        buffer.clear();

        let isize = size as i64;
        let iindex = index as i64;

        let max_index = isize * isize - 1;

        let mut temp_index = iindex - 2 * isize - 1;

        // no matter the size of the sudoku you always get 8 cells
        // therefore the length of the returned buffer is always the same
        // if there is a way to do it in a loop or a range or something
        // fell free to change it
        // do remember that it migth give negative values out..

        if temp_index >= 0 && max_index > temp_index && temp_index / isize + 3 != iindex / isize {
            // is it faster to cast to usize or just do the math? I have no idea...
            buffer.push(temp_index as usize);
        } // 1

        temp_index = iindex - 2 * isize + 1;
        if temp_index >= 0 && max_index > temp_index && temp_index / isize + 1 != iindex / isize {
            buffer.push(temp_index as usize);
        } // 2

        temp_index = iindex - 1 * isize - 2;
        if temp_index >= 0 && max_index > temp_index && temp_index / isize != iindex / isize - 2 {
            buffer.push(temp_index as usize);
        } // 3

        temp_index = iindex - 1 * isize + 2;
        if temp_index >= 0 && max_index > temp_index && temp_index / isize != iindex / isize {
            buffer.push(temp_index as usize);
        } // 4

        temp_index = iindex + 1 * isize - 2;
        if temp_index >= 0 && max_index > temp_index && temp_index / isize != iindex / isize {
            buffer.push(temp_index as usize);
        } // 5

        temp_index = iindex + 1 * isize + 2;
        if temp_index >= 0 && max_index > temp_index && temp_index / isize != iindex / isize + 2 {
            buffer.push(temp_index as usize);
        } // 6

        temp_index = iindex + 2 * isize - 1;
        if temp_index >= 0 && max_index > temp_index && temp_index / isize != iindex / isize + 1 {
            buffer.push(temp_index as usize);
        } // 7

        temp_index = iindex + 2 * isize + 1;
        if temp_index >= 0 && max_index > temp_index && temp_index / isize != iindex / isize + 3 {
            buffer.push(temp_index as usize);
        } // 8

        buffer
    }

    fn hidden_singles(&self, _sudoku: &Sudoku) -> Option<(u16, usize)> {
        // Hidden singles are not a thing for the knights rule
        None
    }
    fn boxed_clone(&self) -> DynRule {
        Box::new(self.clone())
    }

    fn get_name(&self) -> &'static str {
        "KnightRule"
    }
}

#[test]
fn diagonal_test() {
    let sudoku = Sudoku::new(9, vec![]);

    let diagonalrule = DiagonalRule;
    let mut buffer = vec![];

    let mut indexes = diagonalrule.updates(sudoku.size, 11, &mut buffer);
    assert_eq!(indexes, vec![]);

    indexes = diagonalrule.updates(sudoku.size, 80, &mut buffer);
    assert_eq!(indexes, vec![0, 10, 20, 30, 40, 50, 60, 70, 80]);

    indexes = diagonalrule.updates(sudoku.size, 0, &mut buffer);
    assert_eq!(indexes, vec![0, 10, 20, 30, 40, 50, 60, 70, 80]);

    indexes = diagonalrule.updates(sudoku.size, 16, &mut buffer);
    assert_eq!(indexes, vec![8, 16, 24, 32, 40, 48, 56, 64, 72]);

    indexes = diagonalrule.updates(sudoku.size, 40, &mut buffer);
    assert_eq!(
        indexes,
        vec![0, 10, 20, 30, 40, 50, 60, 70, 80, 8, 16, 24, 32, 40, 48, 56, 64, 72]
    );

    let sudokuSmall = Sudoku::new(4, vec![]);

    indexes = diagonalrule.updates(sudokuSmall.size, 0, &mut buffer);
    assert_eq!(indexes, vec![0, 5, 10, 15]);
    indexes = diagonalrule.updates(sudokuSmall.size, 1, &mut buffer);
    assert_eq!(indexes, vec![]);
    indexes = diagonalrule.updates(sudokuSmall.size, 2, &mut buffer);
    assert_eq!(indexes, vec![]);
    indexes = diagonalrule.updates(sudokuSmall.size, 3, &mut buffer);
    assert_eq!(indexes, vec![3, 6, 9, 12]);
    indexes = diagonalrule.updates(sudokuSmall.size, 4, &mut buffer);
    assert_eq!(indexes, vec![]);
    indexes = diagonalrule.updates(sudokuSmall.size, 5, &mut buffer);
    assert_eq!(indexes, vec![0, 5, 10, 15]);
    indexes = diagonalrule.updates(sudokuSmall.size, 6, &mut buffer);
    assert_eq!(indexes, vec![3, 6, 9, 12]);
    indexes = diagonalrule.updates(sudokuSmall.size, 7, &mut buffer);
    assert_eq!(indexes, vec![]);
    indexes = diagonalrule.updates(sudokuSmall.size, 8, &mut buffer);
    assert_eq!(indexes, vec![]);
    indexes = diagonalrule.updates(sudokuSmall.size, 9, &mut buffer);
    assert_eq!(indexes, vec![3, 6, 9, 12]);    
    indexes = diagonalrule.updates(sudokuSmall.size, 10, &mut buffer);
    assert_eq!(indexes, vec![0, 5, 10, 15]);
    indexes = diagonalrule.updates(sudokuSmall.size, 11, &mut buffer);
    assert_eq!(indexes, vec![]);
    indexes = diagonalrule.updates(sudokuSmall.size, 12, &mut buffer);
    assert_eq!(indexes, vec![3, 6, 9, 12]);
    indexes = diagonalrule.updates(sudokuSmall.size, 13, &mut buffer);
    assert_eq!(indexes, vec![]);
    indexes = diagonalrule.updates(sudokuSmall.size, 14, &mut buffer);
    assert_eq!(indexes, vec![]);
    indexes = diagonalrule.updates(sudokuSmall.size, 15, &mut buffer);
    assert_eq!(indexes, vec![0, 5, 10, 15]);

}

#[test]
fn row_test() {
    let sudoku = Sudoku::new(9, vec![]);

    let rowrule = RowRule;
    let mut buffer = vec![];
    let indexes = rowrule.updates(sudoku.size, 11, &mut buffer);
    println!("{indexes:?}");

    assert_eq!(indexes, vec![9, 10, 11, 12, 13, 14, 15, 16, 17])
}

#[test]
fn column_test() {
    let sudoku = Sudoku::new(9, vec![]);

    let columnrule = ColumnRule;
    let mut buffer = vec![];
    let indexes = columnrule.updates(sudoku.size, 11, &mut buffer);
    println!("{indexes:?}");

    assert_eq!(indexes, vec![2, 11, 20, 29, 38, 47, 56, 65, 74])
}

#[test]
fn square_test() {
    let sudoku = Sudoku::new(9, vec![]);

    let squarerule = SquareRule;
    let mut buffer = vec![];
    let indexes = squarerule.updates(sudoku.size, 11, &mut buffer);
    println!("{indexes:?}");

    assert_eq!(indexes, vec![0, 1, 2, 9, 10, 11, 18, 19, 20])
}

#[test]
fn knight_test() {
    let sudoku = Sudoku::new(9, vec![]);

    let knightrule = KnightRule;
    let mut buffer = vec![];
    let indexes = knightrule.updates(sudoku.size, 11, &mut buffer);
    println!("{indexes:?}");

    assert_eq!(indexes, vec![0, 4, 18, 22, 28, 30]);

    let indexes = knightrule.updates(sudoku.size, 40, &mut buffer);
    println!("{indexes:?}");

    assert_eq!(indexes, vec![21, 23, 29, 33, 47, 51, 57, 59])
}

#[test]
fn diagonal_hidden_math_test() {
    let mut sudoku = Sudoku::new(9, vec![Box::new(SquareRule), Box::new(DiagonalRule)]);

    sudoku.set_cell(1, 27).unwrap();
    sudoku.set_cell(1, 39).unwrap();
    sudoku.set_cell(1, 78).unwrap();
    sudoku.set_cell(1, 55).unwrap();

    println!("{sudoku}");

    let diagonalrule = DiagonalRule;
    let res = diagonalrule.hidden_singles(&sudoku);
    assert_eq!(res, Some((1, 20)))
}

#[test]
fn x_hidden_math_test() {
    let x_rule = XRule {
        x_clue: vec![(1 as usize, 2 as usize)],
    };
    let mut sudoku = Sudoku::new(4, vec![Box::new(SquareRule), Box::new(x_rule.clone())]);

    sudoku.set_cell(1, 1).unwrap();
    println!("{sudoku}");

    let res = x_rule.hidden_singles(&sudoku);
    assert_eq!(res, Some((4, 2)))
}

#[test]
fn row_hidden_math_test() {
    let mut sudoku = Sudoku::new(9, vec![Box::new(SquareRule)]);

    sudoku.set_cell(2, 1).unwrap();
    sudoku.set_cell(1, 56).unwrap();
    sudoku.set_cell(1, 12).unwrap();
    sudoku.set_cell(1, 24).unwrap();

    println!("{sudoku}");

    let rowrule = RowRule;
    let res = rowrule.hidden_singles(&sudoku);
    assert_eq!(res, Some((1, 0)))
}

#[test]
fn column_hidden_math_test() {
    let mut sudoku = Sudoku::new(9, vec![Box::new(SquareRule)]);

    sudoku.set_cell(2, 9).unwrap();
    sudoku.set_cell(1, 24).unwrap();
    sudoku.set_cell(1, 28).unwrap();
    sudoku.set_cell(1, 56).unwrap();

    println!("\n\n{sudoku}");

    let columnrule = ColumnRule;
    let res = columnrule.hidden_singles(&sudoku);
    assert_eq!(res, Some((1, 0)))
}

#[test]
fn square_hidden_math_test() {
    let mut sudoku = Sudoku::new(9, vec![Box::new(SquareRule)]);

    sudoku.set_cell(1, 27).unwrap();
    sudoku.set_cell(1, 55).unwrap();
    sudoku.set_cell(1, 8).unwrap();
    sudoku.set_cell(1, 12).unwrap();

    println!("{sudoku}");

    let squarerule = SquareRule;
    let res = squarerule.hidden_singles(&sudoku);
    println!("{res:?}");
    assert_eq!(res, Some((1, 20)))
}