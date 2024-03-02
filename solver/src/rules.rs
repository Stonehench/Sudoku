use integer_sqrt::IntegerSquareRoot;
use std::fmt::Debug;

use crate::sudoku::Sudoku;

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
    fn boxed_clone(&self) -> Box<dyn Rule>;
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
    fn boxed_clone(&self) -> Box<dyn Rule> {
        Box::new(self.clone())
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
    fn boxed_clone(&self) -> Box<dyn Rule> {
        Box::new(self.clone())
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
    fn boxed_clone(&self) -> Box<dyn Rule> {
        Box::new(self.clone())
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

        let size = size;

        let max_index = size as i32 * size as i32 - 1;

        let mut temp_index = index as i32 - 2 * size as i32 - 1;

        // no matter the size of the sudoku you always get 8 cells
        // therefore the length of the returned buffer is always the same
        // if there is a way to do it in a loop or a range or something
        // fell free to change it
        // do remember that it migth give negative values out..

        if temp_index >= 0 && max_index > temp_index 
            && temp_index / size as i32 + 3 != index as i32 / size as i32 {
            // is it faster to cast to usize or just do the math? I have no idea...
            buffer.push(temp_index as usize);
        } // 1

        temp_index = index as i32 - 2 * size as i32 + 1;
        if temp_index >= 0 && max_index > temp_index 
            && temp_index / size as i32 + 1 != index as i32 / size as i32 {
            buffer.push(temp_index as usize);
        } // 2

        temp_index =  index as i32 - 1 * size as i32 - 2 ;
        if temp_index >= 0 && max_index > temp_index
            && temp_index / size as i32 != index as i32 / size as i32 - 2{
            buffer.push(temp_index as usize);
        } // 3

        temp_index = index as i32 - 1 * size as i32 + 2;
        if temp_index >= 0 && max_index > temp_index 
            && temp_index / size as i32 != index as i32 / size as i32{
            buffer.push(temp_index as usize);
        } // 4

        temp_index = index as i32 + 1 * size as i32 - 2;
        if temp_index >= 0 && max_index > temp_index  
            && temp_index / size as i32 != index as i32 / size as i32{
            buffer.push(temp_index as usize);
        } // 5

        temp_index = index as i32 + 1 * size as i32 + 2;
        if temp_index >= 0 && max_index > temp_index 
            && temp_index / size as i32 != index as i32 / size as i32 + 2 {
            buffer.push(temp_index as usize);
        } // 6

        temp_index = index as i32 + 2 * size as i32 - 1;
        if temp_index >= 0 && max_index > temp_index 
            && temp_index / size as i32 != index as i32 / size as i32 + 1 {
            buffer.push(temp_index as usize);
        } // 7

        temp_index = index as i32 + 2 * size as i32 + 1 ;
        if temp_index >= 0 && max_index > temp_index 
            && temp_index / size as i32 != index as i32 / size as i32 + 3 {
            buffer.push(temp_index as usize);
        } // 8  

        buffer
    }

    fn hidden_singles(&self, sudoku: &Sudoku) -> Option<(u16, usize)> {
        // Hidden singles are not a thing for the knights rule
        None
    }
    fn boxed_clone(&self) -> Box<dyn Rule> {
        Box::new(self.clone())
    }
   
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
fn row_hidden_math_test() {
    let mut sudoku = Sudoku::new(
        9,
        vec![
            Box::new(RowRule),
            Box::new(ColumnRule),
            Box::new(SquareRule),
        ],
    );

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
    let mut sudoku = Sudoku::new(
        9,
        vec![
            Box::new(RowRule),
            Box::new(ColumnRule),
            Box::new(SquareRule),
        ],
    );

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
    let mut sudoku = Sudoku::new(
        9,
        vec![
            Box::new(RowRule),
            Box::new(ColumnRule),
            Box::new(SquareRule),
        ],
    );

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
