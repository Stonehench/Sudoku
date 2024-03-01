use integer_sqrt::IntegerSquareRoot;
use std::fmt::Debug;

use crate::sudoku::Sudoku;

pub trait Rule: Debug {
    fn updates<'buf>(
        &self,
        sudoku: &Sudoku,
        index: usize,
        buffer: &'buf mut Vec<usize>,
    ) -> &'buf [usize];
    fn is_legal(&self, sudoku: &Sudoku, index: usize, value: u16, buffer: &mut Vec<usize>) -> bool {
        !self
            .updates(sudoku, index, buffer)
            .iter()
            .map(|i| &sudoku.cells[*i])
            .any(|c| c.is_single_eq(value))
    }

    fn hidden_singles(&self, sudoku: &Sudoku) -> Option<(u16, usize)>;
    fn boxed_clone(&self) -> Box<dyn Rule>;
}

#[derive(Debug, Clone)]
pub struct SquareRule;

impl SquareRule {
    fn updates_iter(&self, sudoku: &Sudoku, index: usize) -> impl Iterator<Item = usize> {
        //Burde gerne være ok med arbitær størrelse?
        let row = index / sudoku.size;
        let size = sudoku.size;
        let sub_size = sudoku.size.integer_sqrt();

        (0..sudoku.size).map(move |i| {
            (index - (index % sub_size)) - (size * (row % sub_size))
                + (i % sub_size)
                + (size * (i / sub_size))
        })
    }
}

impl Rule for SquareRule {
    fn updates<'buf>(
        &self,
        sudoku: &Sudoku,
        index: usize,
        buffer: &'buf mut Vec<usize>,
    ) -> &'buf [usize] {
        buffer.clear();

        self.updates_iter(sudoku, index)
            .for_each(|i| buffer.push(i));
        buffer
    }

    fn hidden_singles(&self, sudoku: &Sudoku) -> Option<(u16, usize)> {
        let sub_size = sudoku.size.integer_sqrt();
        let squares = (0..sudoku.size)
            .map(|index| index * sub_size + (index / sub_size) * sudoku.size * (sub_size - 1));

        for square_entry_index in squares {
            'value: for value in 1..=sudoku.size as u16 {
                let mut found_position = None;
                for position in self.updates_iter(sudoku, square_entry_index) {
                    if sudoku.cells[position].available.contains(&value) {
                        if found_position.is_some() {
                            // Der er allerede fundet en anden i denne square som har value.
                            continue 'value;
                        }
                        found_position = Some(position);
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
pub struct RowRule;

impl RowRule {
    fn updates_iter(&self, sudoku: &Sudoku, index: usize) -> impl Iterator<Item = usize> {
        let row = index / sudoku.size;
        let size = sudoku.size;
        (0..sudoku.size).map(move |i| i + row * size)
    }
}

impl Rule for RowRule {
    fn updates<'buf>(
        &self,
        sudoku: &Sudoku,
        index: usize,
        buffer: &'buf mut Vec<usize>,
    ) -> &'buf [usize] {
        buffer.clear();

        self.updates_iter(sudoku, index)
            .for_each(|i| buffer.push(i));
        buffer
    }

    fn hidden_singles(&self, sudoku: &Sudoku) -> Option<(u16, usize)> {
        for row_number in 0..sudoku.size {
            'value: for value in 1..=sudoku.size as u16 {
                let mut found_position = None;
                for position in self.updates_iter(sudoku, row_number) {
                    if sudoku.cells[position].available.contains(&value) {
                        if found_position.is_some() {
                            continue 'value;
                        }
                        found_position = Some(position);
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
        sudoku: &Sudoku,
        index: usize,
        buffer: &'buf mut Vec<usize>,
    ) -> &'buf [usize] {
        buffer.clear();
        let column = index % sudoku.size;
        (0..sudoku.size)
            .map(|i| i * sudoku.size + column)
            .for_each(|i| buffer.push(i));
        buffer
    }

    fn hidden_singles(&self, sudoku: &Sudoku) -> Option<(u16, usize)> {
        let mut buffer = vec![];
        let buffer = &mut buffer;
        for column_number in 0..sudoku.size {
            let column = self.updates(sudoku, column_number, buffer);
            for value in 1..=sudoku.size as u16 {
                let cells = column.iter().map(|i| &sudoku.cells[*i]);
                let count = cells.filter(|cell| cell.available.contains(&value)).count();
                if count == 1 {
                    let mut cells = column.iter().map(|i| &sudoku.cells[*i]);
                    let position = cells
                        .position(|cell| cell.available.contains(&value))
                        .unwrap();

                    let real_position = column_number + position * sudoku.size;
                    if !sudoku.cells[real_position].locked_in {
                        //println!("Found Hidden {value} in column {column_number}");
                        return Some((value, real_position));
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

#[test]
fn row_test() {
    let sudoku = Sudoku::new(9, vec![]);

    let rowrule = RowRule;
    let mut buffer = vec![];
    let indexes = rowrule.updates(&sudoku, 11, &mut buffer);
    println!("{indexes:?}");

    assert_eq!(indexes, vec![9, 10, 11, 12, 13, 14, 15, 16, 17])
}

#[test]
fn column_test() {
    let sudoku = Sudoku::new(9, vec![]);

    let columnrule = ColumnRule;
    let mut buffer = vec![];
    let indexes = columnrule.updates(&sudoku, 11, &mut buffer);
    println!("{indexes:?}");

    assert_eq!(indexes, vec![2, 11, 20, 29, 38, 47, 56, 65, 74])
}

#[test]
fn square_test() {
    let sudoku = Sudoku::new(9, vec![]);

    let squarerule = SquareRule;
    let mut buffer = vec![];
    let indexes = squarerule.updates(&sudoku, 11, &mut buffer);
    println!("{indexes:?}");

    assert_eq!(indexes, vec![0, 1, 2, 9, 10, 11, 18, 19, 20])
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

    sudoku.set_cell(2, 1);
    sudoku.set_cell(1, 56);
    sudoku.set_cell(1, 12);
    sudoku.set_cell(1, 24);

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

    sudoku.set_cell(2, 9);
    sudoku.set_cell(1, 24);
    sudoku.set_cell(1, 28);
    sudoku.set_cell(1, 56);

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

    sudoku.set_cell(1, 27);
    sudoku.set_cell(1, 55);
    sudoku.set_cell(1, 8);
    sudoku.set_cell(1, 12);

    println!("{sudoku}");

    let squarerule = SquareRule;
    let res = squarerule.hidden_singles(&sudoku);
    println!("{res:?}");
    assert_eq!(res, Some((1, 20)))
}
