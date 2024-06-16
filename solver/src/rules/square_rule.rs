use super::{DynRule, Rule};
use bumpalo::Bump;
use integer_sqrt::IntegerSquareRoot;

use crate::sudoku::Sudoku;

#[derive(Debug, Clone)]
pub struct SquareRule;

impl SquareRule {
    pub fn new() -> DynRule {
        DynRule(Box::new(Self))
    }
}

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
        DynRule(Box::new(self.clone()))
    }

    fn priority(&self) -> super::ExecutionPriority {
        super::ExecutionPriority::Low
    }

    fn get_name(&self) -> &'static str {
        "SquareRule"
    }

    fn locked_candidate<'buf>(
        &self,
        sudoku: &Sudoku,
        buffer: &'buf mut Vec<usize>,
        arena: &mut Bump,
    ) -> Option<(u16, &'buf [usize])> {
        //println!("Entering locked square");
        arena.reset();
        let sub_s = sudoku.size.integer_sqrt();
        for value in 1..sudoku.size as u16 {
            'row: for row in 0..sudoku.size {
                let mut found_square = None;
                for x in 0..sudoku.size {
                    let index = row * sudoku.size + x;
                    let cell = &sudoku.cells[index];

                    if !cell.locked_in && cell.available.contains(&value) {
                        if let Some(found_square) = found_square {
                            let current_square = x / sub_s;
                            if found_square != current_square {
                                continue 'row;
                            }
                        } else {
                            found_square = Some(x / sub_s);
                        }
                    }
                }

                if let Some(sq_x) = found_square {
                    let sq_y = row / sub_s;
                    buffer.clear();

                    for l_y in 0..sub_s {
                        for l_x in 0..sub_s {
                            let x = l_x + sq_x * sub_s;
                            let y = l_y + sq_y * sub_s;
                            if y == row {
                                break;
                            }
                            if sudoku.cells[x + y * sudoku.size].available.contains(&value) {
                                buffer.push(x + y * sudoku.size);
                            }
                        }
                    }
                    //println!("square locked found in ROW");
                    if !buffer.is_empty() {
                        return Some((value, buffer));
                    }
                }
            }

            'col: for col in 0..sudoku.size {
                let mut found_square = None;

                for y in 0..sudoku.size {
                    let index = y * sudoku.size + col;
                    let cell = &sudoku.cells[index];

                    if !cell.locked_in && cell.available.contains(&value) {
                        if let Some(found_square) = found_square {
                            let current_square = y / sub_s;
                            if found_square != current_square {
                                continue 'col;
                            }
                        } else {
                            found_square = Some(y / sub_s);
                        }
                    }
                }

                if let Some(sq_y) = found_square {
                    let sq_x = col / sub_s;
                    buffer.clear();

                    for l_x in 0..sub_s {
                        for l_y in 0..sub_s {
                            let x = l_x + sq_x * sub_s;
                            let y = l_y + sq_y * sub_s;
                            if x == col {
                                break;
                            }
                            if sudoku.cells[x + y * sudoku.size].available.contains(&value) {
                                buffer.push(x + y * sudoku.size);
                            }
                        }
                    }
                    //println!("square locked found in COLUMN");
                    if !buffer.is_empty() {
                        return Some((value, buffer));
                    }
                }
            }
        }
        //println!("LOCKED SQUARE!! FOUND NONE");
        None
    }

    fn print_self(&self) -> bool {
        print!("SquareRule");
        true
    }
}

//########################### TEST ###############################

#[test]
fn square_hidden_math_test() {
    let mut sudoku = Sudoku::new(9, vec![SquareRule::new()]);

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

#[test]
fn square_test() {
    let sudoku = Sudoku::new(9, vec![]);

    let squarerule = SquareRule;
    let mut buffer = vec![];
    let indexes = squarerule.updates(sudoku.size, 11, &mut buffer);
    assert_eq!(indexes, vec![0, 1, 2, 9, 10, 11, 18, 19, 20]);

    let indexes = squarerule.updates(16, 255, &mut buffer);
    assert_eq!(
        indexes,
        vec![204, 205, 206, 207, 220, 221, 222, 223, 236, 237, 238, 239, 252, 253, 254, 255]
    )
}

#[test]
fn square_16x_locked() {
    let mut sudoku = Sudoku::new(16, vec![SquareRule::new()]);

    sudoku.set_cell(1, 0).unwrap();
    sudoku.set_cell(2, 1).unwrap();
    sudoku.set_cell(3, 2).unwrap();
    sudoku.set_cell(4, 3).unwrap();
    sudoku.set_cell(5, 4).unwrap();
    sudoku.set_cell(6, 5).unwrap();
    sudoku.set_cell(7, 6).unwrap();
    sudoku.set_cell(8, 7).unwrap();
    sudoku.set_cell(9, 8).unwrap();
    sudoku.set_cell(10, 9).unwrap();
    sudoku.set_cell(11, 10).unwrap();
    sudoku.set_cell(12, 11).unwrap();

    let mut buffer = vec![];
    let mut arena = Bump::new();

    let squarerule = SquareRule;
    let res = squarerule.locked_candidate(&sudoku, &mut buffer, &mut arena);
    println!("{res:?}");
    assert_eq!(
        res,
        Some((
            13,
            vec![28, 29, 30, 31, 44, 45, 46, 47, 60, 61, 62, 63].as_slice()
        ))
    );

    sudoku = Sudoku::new(16, vec![SquareRule::new()]);

    sudoku.set_cell(1, 0).unwrap();
    sudoku.set_cell(2, 16).unwrap();
    sudoku.set_cell(3, 32).unwrap();
    sudoku.set_cell(4, 48).unwrap();
    sudoku.set_cell(5, 64).unwrap();
    sudoku.set_cell(6, 80).unwrap();
    sudoku.set_cell(7, 96).unwrap();
    sudoku.set_cell(8, 112).unwrap();
    sudoku.set_cell(9, 128).unwrap();
    sudoku.set_cell(10, 144).unwrap();
    sudoku.set_cell(11, 160).unwrap();
    sudoku.set_cell(12, 176).unwrap();

    let squarerule = SquareRule;
    let res = squarerule.locked_candidate(&sudoku, &mut buffer, &mut arena);
    println!("{res:?}");
    assert_eq!(
        res,
        Some((
            13,
            vec![193, 209, 225, 241, 194, 210, 226, 242, 195, 211, 227, 243].as_slice()
        ))
    );

    sudoku = Sudoku::new(16, vec![SquareRule::new()]);

    let res = squarerule.locked_candidate(&sudoku, &mut buffer, &mut arena);
    println!("{res:?}");
    assert_eq!(res, None);

    sudoku = Sudoku::new(9, vec![SquareRule::new()]);
    sudoku.set_cell(1, 9).unwrap();
    sudoku.set_cell(2, 10).unwrap();
    sudoku.set_cell(3, 11).unwrap();
    sudoku.set_cell(4, 12).unwrap();
    sudoku.set_cell(5, 13).unwrap();
    sudoku.set_cell(6, 14).unwrap();

    let res = squarerule.locked_candidate(&sudoku, &mut buffer, &mut arena);
    println!("{res:?}");
    assert_eq!(res, Some((7, vec![6, 7, 8, 24, 25, 26].as_slice())));

    sudoku = Sudoku::new(9, vec![SquareRule::new()]);
    sudoku.set_cell(1, 8).unwrap();
    sudoku.set_cell(2, 17).unwrap();
    sudoku.set_cell(3, 26).unwrap();
    sudoku.set_cell(4, 35).unwrap();
    sudoku.set_cell(5, 44).unwrap();
    sudoku.set_cell(6, 53).unwrap();

    let res = squarerule.locked_candidate(&sudoku, &mut buffer, &mut arena);
    assert_eq!(res, Some((7, vec![60, 69, 78, 61, 70, 79].as_slice())));

    // remove the 7's from the avalible, such that it should become 8 that is the value in the next return
    if let Some((_value, remove_indecies)) = res {
        for index in remove_indecies {
            let cell = &mut sudoku.cells[*index];
            cell.available.remove(6);
        }
    }

    println!("{sudoku}");

    let res = squarerule.locked_candidate(&sudoku, &mut buffer, &mut arena);
    assert_eq!(res, Some((8, vec![60, 69, 78, 61, 70, 79].as_slice())));

    sudoku = Sudoku::new(4, vec![SquareRule::new()]);
    let res = squarerule.locked_candidate(&sudoku, &mut buffer, &mut arena);
    assert_eq!(res, None);
}
