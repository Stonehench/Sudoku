use super::{DynRule, Rule};
use bumpalo::Bump;
use rand::random;
use std::fmt::Debug;

use crate::sudoku::{Cell, Sudoku};

#[derive(Debug, Clone)]
pub struct ConsecutiveRule {
    pub consecutive_clue: Vec<(usize, usize)>,
}

impl ConsecutiveRule {
    pub fn new(consecutive_clue: Vec<(usize, usize)>) -> DynRule {
        DynRule(Box::new(ConsecutiveRule { consecutive_clue }))
    }
}

impl Rule for ConsecutiveRule {
    fn updates<'buf>(
        &self,
        _size: usize,
        _index: usize,
        buffer: &'buf mut Vec<usize>,
    ) -> &'buf [usize] {
        buffer.clear();

        // Might not need to affect stuff

        buffer
    }

    fn hidden_singles(&self, sudoku: &Sudoku) -> Option<(u16, usize)> {

        for (left_index, right_index) in &self.consecutive_clue {
            if sudoku.cells[*left_index].locked_in && !sudoku.cells[*right_index].locked_in {
                // find the value of the locked candidate
                if let Some(value) = sudoku.cells[*left_index].available.get(0) {
                    if sudoku.cells[*right_index].available.contains( &(value + 1) ) && !(sudoku.cells[*right_index].available.contains(&(value - 1))){
                        return Some(((value + 1), *right_index));
                    }
                    if !(sudoku.cells[*right_index].available.contains( &(value + 1) )) && sudoku.cells[*right_index].available.contains(&(value - 1)){
                        return Some(((value - 1), *right_index));
                    }
                   
                }
            }
            if sudoku.cells[*right_index].locked_in && !sudoku.cells[*left_index].locked_in {
                // find the value of the locked candidate
                if let Some(value) = sudoku.cells[*right_index].available.get(0) {
                    if sudoku.cells[*left_index].available.contains(&(value + 1) ) && !(sudoku.cells[*left_index].available.contains(&(value - 1))){
                        return Some(((value + 1), *left_index));
                    }
                    if !(sudoku.cells[*left_index].available.contains(&(value + 1) )) && sudoku.cells[*left_index].available.contains(&(value - 1)){
                        return Some(((value - 1), *left_index));
                    }
                   
                }
            }


        }

        None
    }

    fn locked_candidate<'buf>(
            &self,
            sudoku: &Sudoku,
            buffer: &'buf mut Vec<usize>,
            _arena: &mut Bump,
        ) -> Option<(u16, &'buf [usize])> {
        
        buffer.clear();
        
        for value in 1..(sudoku.size +1) as u16{        
            for (left, right) in &self.consecutive_clue {
                if !sudoku.cells[*left].locked_in && sudoku.cells[*left].available.contains(&value) 
                    && !(sudoku.cells[*right].available.contains(&(value + 1)) || sudoku.cells[*right].available.contains(&(value - 1))){
                        buffer.push(*left);
                }

                if !sudoku.cells[*right].locked_in && sudoku.cells[*right].available.contains(&value) 
                    && !(sudoku.cells[*left].available.contains(&(value + 1)) || sudoku.cells[*left].available.contains(&(value - 1))){
                        buffer.push(*right);
                }
            }

            if !buffer.is_empty() {
                return Some((value, buffer));
            }
        }

        None

    }
    fn multi_remove<'buf>(
            &self,
            sudoku: &Sudoku,
            big_buffer: &'buf mut Vec<(u16, usize)>,
        ) ->  &'buf [(u16, usize)] {
            big_buffer.clear();
        
        for value in 1..(sudoku.size +1) as u16{        
            for (left, right) in &self.consecutive_clue {
                if !sudoku.cells[*left].locked_in && sudoku.cells[*left].available.contains(&value) 
                    && !(sudoku.cells[*right].available.contains(&(value + 1)) || sudoku.cells[*right].available.contains(&(value - 1))){
                        big_buffer.push((value, *left));
                }

                if !sudoku.cells[*right].locked_in && sudoku.cells[*right].available.contains(&value) 
                    && !(sudoku.cells[*left].available.contains(&(value + 1)) || sudoku.cells[*left].available.contains(&(value - 1))){
                        big_buffer.push((value, *right));
                }
            }
        }

        big_buffer
    }

    fn create_clue(&mut self, cells: &Vec<Cell>, size: usize) {

        for index in 0..cells.len() {
            if let Some(current) = cells[index].available.get(0) {
                if index + 1 >= cells.len() {
                    continue;
                }
                if let Some(left) = cells[index + 1].available.get(0) {
                    if current + 1 == *left
                        || *current == left + 1 && (index + 1) % size != 0
                    {
                        self.consecutive_clue.push((index, index + 1));
                    }
                }
                if index + size >= cells.len() {
                    continue;
                }
                if let Some(below) = cells[index + size].available.get(0) {
                    if current + 1 == *below
                        || *current == below + 1 && index + size < cells.len()
                    {
                        // x rule should have (index , below)
                        self
                            .consecutive_clue
                            .push((index, index + size));
                    }
                }
            }
        }
        // remove some of the generated consecutive pairs
        let count = self.consecutive_clue.len();
        if count > size * 2 {
            for i in 0..count - size * 2 {
                self
                    .consecutive_clue
                    .remove(random::<usize>() % (count - i));
            }
        }
    }

    fn boxed_clone(&self) -> DynRule {
        DynRule(Box::new(self.clone()))
    }

    fn get_name(&self) -> &'static str {
        "ConsecutiveRule"
    }

    fn to_consecutive_rule (&mut self) -> Option<&mut ConsecutiveRule> {
        Some(self)
    }
}

//########################### TEST ###############################


#[test]
fn consecutive_update_test() {}

#[test]
fn consecutive_hidden() {
        let consecutive_rule = ConsecutiveRule {
            consecutive_clue: vec![(1 as usize, 2 as usize)],
        };
        let mut sudoku = Sudoku::new(
            4,
            vec![super::square_rule::SquareRule::new(), consecutive_rule.boxed_clone()],
        );

        sudoku.set_cell(1, 1).unwrap();

        println!("{sudoku}");
        let res = consecutive_rule.hidden_singles(&sudoku);
        
        assert_eq!(res, Some((2, 2)));


        let mut sudoku = Sudoku::new(
            4,
            vec![super::square_rule::SquareRule::new(), consecutive_rule.boxed_clone()],
        );
        
        sudoku.set_cell(3, 1).unwrap();
        sudoku.set_cell(4, 3).unwrap();

        println!("{sudoku}");

        let res = consecutive_rule.hidden_singles(&sudoku);
        assert_eq!(res, Some((2, 2)))
}

#[test]
fn locked_consecutive_candidate() {
    let mut buffer = vec![];
    let mut arena = Bump::new();
/* The test sudoku a 4 x 4
=================
‖   | 1 O   |   ‖
-----------------
‖   |   ‖   |   ‖
==O==============
‖ 1 |   ‖   |   ‖
-----------------
‖   |   ‖   |   ‖
=================
*/

    let consecutive_rule = ConsecutiveRule {
        consecutive_clue: vec![(1 as usize, 2 as usize),(4 as usize, 8 as usize)],
    };
    let mut sudoku = Sudoku::new(
        4,
        vec![super::square_rule::SquareRule::new(), consecutive_rule.boxed_clone()],
    );

    sudoku.set_cell(1, 1).unwrap();
    sudoku.set_cell(1, 8).unwrap();
    let res = consecutive_rule.locked_candidate(&sudoku, &mut buffer, &mut arena);

    // locked candidates should return that there can not be 3 in either of the dominos (index 2 and 4)
    // because 3 is not consecutive with 1
    assert_eq!(res, Some((3, (vec![2 as usize, 4 as usize].as_slice()))));

/* The test sudoku a 4 x 4
=================
‖   | 1 O   |   ‖
-----------------
‖   |   ‖ 3 |   ‖
==O==============
‖ 1 |   ‖   |   ‖
-----------------
‖   |   ‖   |   ‖
=================
*/
    sudoku.set_cell(3, 6).unwrap();
    let res = consecutive_rule.locked_candidate(&sudoku, &mut buffer, &mut arena);

    // locked candidates should return that there can not be 4 in either of the dominos (index 2 and 4)
    // because 4 is not consecutive with 1
    assert_eq!(res, Some((4, (vec![2 as usize, 4 as usize].as_slice()))));
}


#[test]
fn consecutive_multi() {
        let consecutive_rule = ConsecutiveRule {
            consecutive_clue: vec![(1 as usize, 2 as usize)],
        };
        let mut sudoku = Sudoku::new(
            4,
            vec![super::square_rule::SquareRule::new(), consecutive_rule.boxed_clone()],
        );

        sudoku.set_cell(1, 1).unwrap();
        let mut big_buffer: Vec<(u16, usize)> = vec![];

        println!("{sudoku}");
        let res = consecutive_rule.multi_remove(&sudoku, &mut big_buffer);
        

        println!("{res:?}");
        //assert_eq!(res, Some((2, 2)));


        let mut sudoku = Sudoku::new(
            4,
            vec![super::square_rule::SquareRule::new(), consecutive_rule.boxed_clone()],
        );
        
        sudoku.set_cell(3, 1).unwrap();
        sudoku.set_cell(4, 3).unwrap();

        println!("{sudoku}");

        let res = consecutive_rule.multi_remove(&sudoku, &mut big_buffer);
        println!("{res:?}");
}
