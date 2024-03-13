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

