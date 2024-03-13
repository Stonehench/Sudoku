use super::Rule;
use allocator_api2::vec::Vec as AlloVec;
use bumpalo::Bump;
use integer_sqrt::IntegerSquareRoot;
use std::fmt::Debug;

use crate::sudoku::{DynRule, Sudoku};

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

    fn locked_candidate<'buf>(
        &self,
        sudoku: &Sudoku,
        buffer: &'buf mut Vec<usize>,
        arena: &mut Bump,
    ) -> Option<(u16, &'buf [usize])> {
        arena.reset();
        let sub_size = sudoku.size.integer_sqrt();
        enum SqType {
            Row,
            Column,
        }

        // Hjælper funktion
        fn locked_in_sq<'arena>(
            sq_y: usize,
            sq_x: usize,
            sub_size: usize,
            value: u16,
            sq_type: SqType,
            arena: &'arena Bump,
            sudoku: &Sudoku,
        ) -> AlloVec<usize, &'arena Bump> {
            let mut data = AlloVec::with_capacity_in(sub_size, arena);

            for l_x in 0..sub_size {
                for l_y in 0..sub_size {
                    let x = l_x + sq_x * sub_size;
                    let y = l_y + sq_y * sub_size;
                    let i = x + y * sudoku.size;
                    let cell = &sudoku.cells[i];
                    if !cell.locked_in && cell.available.contains(&value) {
                        data.push(match sq_type {
                            SqType::Row => l_x,
                            SqType::Column => l_y,
                        });
                    }
                }
            }
            data.dedup();
            data.sort();
            data
        }

        for value in 1..sudoku.size as u16 + 1 {
            //Tjek for vandret
            let mut masks_y = AlloVec::<_, &Bump>::new_in(arena);
            for sq_y in 0..sub_size {
                masks_y.clear();
                for sq_x in 0..sub_size {
                    masks_y.push(locked_in_sq(
                        sq_y,
                        sq_x,
                        sub_size,
                        value,
                        SqType::Column,
                        arena,
                        sudoku,
                    ))
                }

                //Tjek om der er nogle af dem som er 100% ens
                for l in 0..sub_size {
                    for r in l + 1..sub_size {
                        if !masks_y[l].is_empty()
                            && masks_y[l].len() < sub_size
                            && masks_y[l] == masks_y[r]
                        {
                            //println!("HORIZONTAL {value}: {:?} = {:?} at {l} {r}", masks_y[l], masks_y[r]);
                            buffer.clear();

                            for n_sq_x in (0..sub_size).filter(|sq_x| *sq_x != l && *sq_x != r) {
                                for l_x in 0..sub_size {
                                    for l_y in (0..sub_size).filter(|y| masks_y[l].contains(y)) {
                                        let x = l_x + n_sq_x * sub_size;
                                        let y = l_y + sq_y * sub_size;
                                        let i = x + y * sudoku.size;
                                        let cell = &sudoku.cells[i];
                                        if cell.available.contains(&value) {
                                            buffer.push(i);
                                        }
                                    }
                                }
                            }

                            if !buffer.is_empty() {
                                return Some((value, buffer));
                            }
                        }
                    }
                }
            }
            //Tjek for lodret
            let mut masks_x = AlloVec::<_, &Bump>::new_in(&arena);
            for sq_x in 0..sub_size {
                masks_x.clear();
                for sq_y in 0..sub_size {
                    masks_x.push(locked_in_sq(
                        sq_y,
                        sq_x,
                        sub_size,
                        value,
                        SqType::Row,
                        arena,
                        sudoku,
                    ))
                }

                //Tjek om der er nogle af dem som er 100% identisk
                for l in 0..sub_size {
                    for r in l + 1..sub_size {
                        if !masks_x[l].is_empty()
                            && masks_x[l].len() < sub_size
                            && masks_x[l] == masks_x[r]
                        {
                            //println!("VERTICAL {value}: {:?} = {:?} at {l} {r}", masks_x[l], masks_x[r]);
                            buffer.clear();

                            for n_sq_y in (0..sub_size).filter(|sq_y| *sq_y != l && *sq_y != r) {
                                for l_x in (0..sub_size).filter(|x| masks_x[l].contains(x)) {
                                    for l_y in 0..sub_size {
                                        let x = l_x + sq_x * sub_size;
                                        let y = l_y + n_sq_y * sub_size;
                                        let i = x + y * sudoku.size;
                                        let cell = &sudoku.cells[i];
                                        if cell.available.contains(&value) {
                                            buffer.push(i);
                                        }
                                    }
                                }
                            }

                            if !buffer.is_empty() {
                                return Some((value, buffer));
                            }
                        }
                    }
                }
            }
        }
        None
    }
}
