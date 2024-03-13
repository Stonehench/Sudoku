use allocator_api2::vec::Vec as AlloVec;
use bumpalo::Bump;
use integer_sqrt::IntegerSquareRoot;
use std::cell::RefCell;
use std::{fmt::Debug, str::FromStr};
use crate::rules::x_rule::XRule;
use crate::rules::diagonal_rule::DiagonalRule;
use crate::rules::row_rule::RowRule;
use crate::rules::column_rule::ColumnRule;
use crate::rules::square_rule::SquareRule;
use crate::rules::knight_rule::KnightRule;



use crate::sudoku::{DynRule, Sudoku};

pub mod square_rule;
pub mod column_rule;
pub mod row_rule;
pub mod knight_rule;
pub mod x_rule;
pub mod diagonal_rule;

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

    // not all rules may have a possibblility to avail hidden singles
    fn hidden_singles(&self, _sudoku: &Sudoku) -> Option<(u16, usize)> {
        None
    }

    // TODO altså jeg er ikke helt sikker på at det her er 100% lovligt
    // return (Value to be removed, [list of indexes where the removel should happen])
    fn locked_candidate<'buf>(
        &self,
        _sudoku: &Sudoku,
        _buffer: &'buf mut Vec<usize>,
        _arena: &mut Bump,
    ) -> Option<(u16, &'buf [usize])> {
        None
    }

    fn boxed_clone(&self) -> DynRule;
    fn get_name(&self) -> &'static str;

    fn to_x_rule(&mut self) -> Option<&mut XRule> {
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

#[test]
fn locked_column_candidate() {
    let mut sudoku = Sudoku::new(9, vec![Box::new(SquareRule)]);
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
    
    sudoku = Sudoku::new(9, vec![Box::new(SquareRule)]);
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
fn locked_row_candidate() {
    let mut sudoku = Sudoku::new(9, vec![Box::new(SquareRule)]);
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


    sudoku = Sudoku::new(9, vec![Box::new(SquareRule)]);

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
fn locked_x_candidate() {
    let mut sudoku = Sudoku::new(4, vec![]);
    let x_rule = XRule {
        x_clue: vec![(1 as usize, 2 as usize)],
    };

    sudoku.set_cell(1, 5).unwrap();
    println!("{sudoku}");
    let mut buffer = vec![];
    let mut arena = Bump::new();
    let res = x_rule.locked_candidate(&sudoku, &mut buffer, &mut arena);
    assert_eq!(res, Some((4, vec![2].as_slice())))
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

    let rowrule = RowRule::new();
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

    let columnrule = ColumnRule::new();
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
#[test]
fn locked_square_x_candidate() {
    let mut sudoku = Sudoku::new(9, vec![Box::new(SquareRule)]);

    let removes = vec![
        0, 2, 4, 5, 7, 8, 9, 11, 12, 14, 15, 16, 18, 19, 20, 21, 22, 24, 25, 26,
    ];

    for index in removes {
        sudoku.cells[index].available.retain(|n| *n != 1);
    }
    let mut buffer = vec![];
    let mut arena = Bump::new();
    let res = SquareRule.locked_candidate(&sudoku, &mut buffer, &mut arena);

    assert_eq!(res, Some((1, vec![3, 13].as_slice())));
}

#[test]
fn locked_square_y_candidate() {
    let mut sudoku = Sudoku::new(9, vec![Box::new(SquareRule)]);

    let removes = vec![
        0, 2, 9, 10, 18, 19, 20, 27, 28, 36, 37, 38, 45, 47, 54, 56, 64, 65, 72, 73,
    ];

    for index in removes {
        sudoku.cells[index].available.retain(|n| *n != 1);
    }

    let mut buffer = vec![];
    let mut arena = Bump::new();
    let res = SquareRule.locked_candidate(&sudoku, &mut buffer, &mut arena);

    assert_eq!(res, Some((1, vec![55, 74].as_slice())));
}
