use crate::rules::diagonal_rule::DiagonalRule;
use crate::rules::knight_rule::KnightRule;
use crate::rules::square_rule::SquareRule;
use crate::rules::x_rule::XRule;
use bumpalo::Bump;
use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
    str::FromStr,
};

use crate::sudoku::Sudoku;

use self::zipper_rule::ZipperRule;

pub mod column_rule;
pub mod diagonal_rule;
pub mod knight_rule;
pub mod row_rule;
pub mod square_rule;
pub mod x_rule;
pub mod zipper_rule;
pub mod consecutive_rule;
pub mod parity_rule;


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

    fn to_zipper_rule(&mut self) -> Option<&mut ZipperRule> {
        None
    }

    fn needs_square_for_locked(&self) -> bool {
        false
    }

    fn priority(&self) -> ExecutionPriority {
        ExecutionPriority::Medium
    }

    fn multi_remove<'buf>(
        &self,
        _sudoku: &Sudoku,
        _big_buffer: &'buf mut Vec<(usize, usize)>,
    ) -> Option<&[(usize, usize)]> {
        return None;
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ExecutionPriority {
    High = 0,
    Medium = 1,
    Low = 2,
}

#[derive(Debug)]
pub struct DynRule(Box<dyn Rule + Send>);

impl Clone for DynRule {
    fn clone(&self) -> Self {
        self.0.boxed_clone()
    }
}

impl Deref for DynRule {
    type Target = Box<dyn Rule + Send>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for DynRule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl FromStr for DynRule {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SquareRule" => Ok(SquareRule::new()),
            "KnightsMove" => Ok(KnightRule::new()),
            "DiagonalRule" => Ok(DiagonalRule::new()),
            _ => {
                let mut rule_params = s.split(';').map(str::trim);
                match rule_params.next() {
                    Some("XRule") => Ok(DynRule(Box::new(XRule {
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
                    }))),
                    Some("ZipperRule") =>{ 
                        Ok(DynRule(Box::new(ZipperRule {
                        zipper_clue: rule_params
                            .map(|s| {
                                let Some((center, rest)) = s.split_once(',') else {
                                    return Err(format!("Failed to split {s} on ,"));
                                };

                                let center = center.parse().map_err(|e| format!("{e:?}"))?;
                                let indecies = rest.split(',').map(str::trim); 

                                let rest_resolved = indecies.map(|s| {
                                    let Some((l, r)) = s.split_once('+') else {
                                        return Err(format!("Failed to split {s} on +"));
                                    };
                                    let l = l.parse().map_err(|e| format!("{e:?}"))?;
                                    let r = r.parse().map_err(|e| format!("{e:?}"))?;

                                    Ok((l, r))
                                })
                                .collect::<Result<_, _>>()?;

                                Ok((center, rest_resolved))
                            })
                            .collect::<Result<_, _>>()?,
                    })))},
                    //Some("ParityRule") => return Err(s.to_owned()), // TODO: finish this for parity should be almost the same as xrule
                    //Some("ConsecutiveRule") => return Err(s.to_owned()), // TODO: finish this for consecutive should be almost the same as xrule
                    _ => return Err(s.to_owned()),
                }
            }
        }
    }
}
