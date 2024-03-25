use super::{DynRule, Rule};
use bumpalo::Bump;
use integer_sqrt::IntegerSquareRoot;
use std::fmt::Debug;

use crate::sudoku::{self, Sudoku};

#[derive(Debug, Clone)]
pub struct ZipperRule {
    pub zipper_clue: Vec<(usize, Vec<(usize, usize)>)>, 
    // zippers are touples of (centerIndex, Vec<Index, Index>), 
    // such that the left part of the touple is the central index, and the value which the zipper should add to,
    // and the right apart of the touple us a vektor of sets on the zipper that ar equal lengths from the center
}

impl ZipperRule {
    pub fn new(zipper_clue: Vec<(usize, Vec<(usize, usize)>)>) -> DynRule {
        DynRule(Box::new(ZipperRule { zipper_clue }))
    }
}


impl Rule for ZipperRule {
    fn updates<'buf>(
        &self,
        _size: usize,
        index: usize,
        buffer: &'buf mut Vec<usize>,
    ) -> &'buf [usize] {
        buffer.clear();

        // if the center of a zipper is set to a number, that number can no longer appear on the zipper,
        // since you would have to add it with 0 to get the center digit which does not make any sense as 0 is not a valid digit
        for (center, rest) in &self.zipper_clue{
            if *center == index {
                for (left, right) in rest {
                    buffer.push(*left);
                    buffer.push(*right);
                }
            // if a place on the zipper that is not the center is updated, the center can no longer be this number
            } else if rest.into_iter().any(|(left, right)| *left == index || *right == index){
                buffer.push(*center);
            }
        }
        buffer
    }

    fn hidden_singles(&self, sudoku: &Sudoku) -> Option<(u16, usize)> {
        // Type 1:
        // center is unknown, two digits in a zipper pair are known, 
        // then the center is a hidden single
        
        // Type 2:
        // center is known, one digit in a zipper pair is known
        // then the other digit in the pair is a hidden single
        
        for (center, rest) in &self.zipper_clue {
            // Type 1:
            if !sudoku.cells[*center].locked_in {
                for (left, right) in rest {
                    if sudoku.cells[*left].locked_in && sudoku.cells[*right].locked_in {
                        let value = sudoku.cells[*left].available[0] + sudoku.cells[*right].available[0];
                        if sudoku.cells[*center].available.contains(&value){
                            return Some((value, *center));
                        }
                        //println!("a mistake has happend"); 
                    }
                } 
            }
            // Type 2:
            if sudoku.cells[*center].locked_in {
                for (left, right) in rest {
                    // the left side is known, calculate the right
                    if sudoku.cells[*left].locked_in && !sudoku.cells[*right].locked_in {
                        if sudoku.cells[*center].available[0] > sudoku.cells[*left].available[0]{
                            let value = sudoku.cells[*center].available[0] - sudoku.cells[*left].available[0];
                            if sudoku.cells[*right].available.contains(&value){
                                return Some((value, *right));
                            }
                        }
                        //println!("a mistake has happend"); 
                    }
                    // the right side is known, calculate the left
                    if sudoku.cells[*right].locked_in && !sudoku.cells[*left].locked_in {
                        if sudoku.cells[*center].available[0] > sudoku.cells[*right].available[0]{
                            let value = sudoku.cells[*center].available[0] - sudoku.cells[*right].available[0];
                            if sudoku.cells[*left].available.contains(&value){
                                return Some((value, *left));
                            }   
                        }
                        //println!("a mistake has happend"); 
                    }
                }                    
            }
        }
        
        None
    }

    fn needs_square_for_locked(&self) -> bool {
        true
    }


    fn multi_remove<'buf>(
            &self,
            sudoku: &Sudoku,
            big_buffer: &'buf mut Vec<(u16, usize)>,
        ) ->   &'buf [(u16, usize)] {
        big_buffer.clear();

        for value in 1..(sudoku.size + 1) as u16{
            for (center, rest) in &self.zipper_clue {
                // for all the pairs in the zipper
                for (left, right) in rest {
                    //if sudoku.cells[*center].available.len() > 0 && sudoku.cells[*left].available.len() > 0 && sudoku.cells[*right].available.len() > 0{
                        let center_greatest = sudoku.cells[*center].available[sudoku.cells[*center].available.len() - 1];

                        // left index contains the value
                        if sudoku.cells[*left].available.contains(&value) && !sudoku.cells[*left].locked_in && (
                            sudoku.cells[*right].available[0] + value > center_greatest ||  // right smallest avalible + value overflows the avalible in center
                            center_greatest <= value || // the value overflows the center avalible
                            sudoku.cells[*center].locked_in && !sudoku.cells[*right].available.contains(&(&sudoku.cells[*center].available[0] - value)) 
                            )
                        {
                            big_buffer.push((value,*left));
                        }
                        // right index contains the value
                        if sudoku.cells[*right].available.contains(&value) && !sudoku.cells[*right].locked_in && (
                            sudoku.cells[*left].available[0] + value > center_greatest || // left smallest avalible + value overflows the avalible in center
                            center_greatest <= value || // the value overflows the center avalible
                            sudoku.cells[*center].locked_in && !sudoku.cells[*left].available.contains(&(&sudoku.cells[*center].available[0] - value)) 
                            )
                        {
                            big_buffer.push((value,*right));
                        }
                    //}
                }
                 
                // calculate how many digit the center "sees", in row, cloumn, and square
                // because the center digit is larger than all the values on the zipper
                // and because only uniqe digits can appear in row, column, and square
                // the center must be larger than the amount of digits it sees
                let sub_s = sudoku.size.integer_sqrt();        
                let same_row: u16 = rest.into_iter().map(|(l,r)| {
                    let mut val = 0;
                    if l % sudoku.size == center % sudoku.size  {
                        val = val + 1;
                    }
                    if r % sudoku.size == center % sudoku.size  {
                        val = val + 1;
                    }
                    val
                }).sum();
                let same_column: u16 = rest.into_iter().map(|(l,r)| {
                    let mut val = 0;
                    if l / sudoku.size == center / sudoku.size  {
                        val = val + 1;
                    }
                    if r / sudoku.size == center / sudoku.size  {
                        val = val + 1;
                    }
                    val
                }).sum();
                // THIS NEEDS SQUARE RULE, BUT NO PRIOR CALCULATIONS NEEDED SQUARE RULE
                let same_square: u16 = rest.into_iter().map(|(l,r)| {
                    let mut val = 0;
                    if l / sudoku.size / sub_s == center / sudoku.size / sub_s && l % sudoku.size / sub_s == center % sudoku.size / sub_s  {
                        val = val + 1;
                    }
                    if r / sudoku.size / sub_s == center / sudoku.size / sub_s && r % sudoku.size / sub_s == center % sudoku.size / sub_s  {
                        val = val + 1;
                    }
                    val
                }).sum();

            
                if  !sudoku.cells[*center].locked_in && sudoku.cells[*center].available.contains(&value) && 
                    ( value == 1 ||
                    rest.into_iter().any(|(left,right)| 
                        sudoku.cells[*left].available[0] + sudoku.cells[*right].available[0] > value )
                        || (value <= same_row || value <= same_column || value <= same_square)) 
                    
                {
                    big_buffer.push((value,*center));
                }
            
            }
        }

        for (center, rest) in &self.zipper_clue {
            if sudoku.cells[*center].locked_in && sudoku.cells[*center].available[0] % 2 == 0 {
                let value = sudoku.cells[*center].available[0] / 2;
                for (left, right) in rest{
                    if left % sudoku.size == right % sudoku.size || left / sudoku.size == right / sudoku.size{ //in same comlumn || row
                        if sudoku.cells[*left].available.contains(&value) && !sudoku.cells[*left].locked_in{
                            big_buffer.push((value,*left));
                        }
                        if sudoku.cells[*right].available.contains(&value) && !sudoku.cells[*right].locked_in{
                            big_buffer.push((value,*right));
                        }
                        
                    }
                    // THIS NEEDS SQUARE RULE
                    let sub_s = sudoku.size.integer_sqrt();
                    if left % sudoku.size / sub_s == right  % sudoku.size / sub_s && left / sudoku.size / sub_s == right / sudoku.size / sub_s { //in same square
                        if sudoku.cells[*left].available.contains(&value) && !sudoku.cells[*left].locked_in{
                            big_buffer.push((value,*left));
                        }
                        if sudoku.cells[*right].available.contains(&value) && !sudoku.cells[*left].locked_in {
                            big_buffer.push((value,*right));
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
        "ZipperRule"
    }

    fn to_zipper_rule(&mut self) -> Option<&mut ZipperRule> {
        Some(self)
    }
}

//########################### TEST ###############################

#[test]
fn zipper_test() {
    let sudoku = Sudoku::new(9, vec![]);

    let zipper_rule = ZipperRule {
        zipper_clue: vec![(1 as usize, vec![(0 as usize, 2 as usize)])],
    };

    let mut buffer = vec![];
    let indexes = zipper_rule.updates(sudoku.size, 1, &mut buffer);
    //println!("{indexes:?}");

    assert_eq!(indexes, vec![0, 2])
}

#[test]
fn zipper_hidden_single_test() {
    let zipper_rule = ZipperRule {
        zipper_clue: vec![(1 as usize, vec![(0 as usize, 2 as usize)])],
    };

    let mut sudoku = Sudoku::new(
        4,
        vec![super::square_rule::SquareRule::new(), zipper_rule.boxed_clone()],
    );

    sudoku.set_cell(4, 1).unwrap();
    sudoku.set_cell(1, 0).unwrap();
    //println!("{sudoku}");

    let res = zipper_rule.hidden_singles(&sudoku);
    // value , index
    assert_eq!(res, Some((3, 2)));

    sudoku = Sudoku::new(
        9,
        vec![super::square_rule::SquareRule::new(), zipper_rule.boxed_clone()],
    );

    let zipper_rule = ZipperRule {
        zipper_clue: vec![(1 as usize, vec![(0 as usize, 2 as usize),(9 as usize, 3 as usize)])],
    };
    sudoku.set_cell(4, 1).unwrap();
    sudoku.set_cell(1, 0).unwrap();
    sudoku.set_cell(3, 2).unwrap();
    sudoku.set_cell(2, 9).unwrap();
    //println!("{sudoku}");

    let res = zipper_rule.hidden_singles(&sudoku);
    // value , index
    assert_eq!(res, Some((2, 3)));
}


#[test]

fn avalible_test() {
    let mut sudoku = Sudoku::new(9, vec![super::square_rule::SquareRule::new()]);
    let mut zipper_rule = ZipperRule {
        zipper_clue: vec![(40,vec![(39,41) , (48,32), (47, 33), (46, 34), (45, 35)])],
    };
    let mut buffer = vec![];
    let mut big_buffer = vec![];
    let mut arena = Bump::new();


    sudoku.set_cell(6, 3).unwrap();
    sudoku.set_cell(9, 4).unwrap();
    sudoku.set_cell(2, 10).unwrap();
    sudoku.set_cell(2, 33).unwrap();
    sudoku.set_cell(1, 50).unwrap();
    sudoku.set_cell(4, 66).unwrap();

    let mut multi_remove_indecies = zipper_rule.multi_remove(&sudoku,  &mut big_buffer);

    for (value, index) in multi_remove_indecies {
        sudoku.cells[*index].available.retain(|i| *i != *value);
    }
    println!("{sudoku}");

    sudoku = Sudoku::new(9, vec![super::square_rule::SquareRule::new()]);
    sudoku.set_cell(6, 3).unwrap();
    sudoku.set_cell(9, 4).unwrap();
    sudoku.set_cell(2, 10).unwrap();
    sudoku.set_cell(7, 40).unwrap();
    sudoku.set_cell(2, 33).unwrap();
    sudoku.set_cell(5, 47).unwrap();
    sudoku.set_cell(1, 50).unwrap();
    sudoku.set_cell(4, 66).unwrap();

    multi_remove_indecies = zipper_rule.multi_remove(&sudoku,  &mut big_buffer);
    println!("{multi_remove_indecies:?}");
    
    for _ in 0..10{
    if let Some((value,indecies)) = zipper_rule.locked_candidate(&sudoku, &mut buffer, &mut arena){
        println!("{value} {indecies:?}");
        for index in indecies {
            sudoku.cells[*index].available.retain(|i| *i != value);
        }
    }}

    println!("{sudoku}");


    zipper_rule = ZipperRule {
        zipper_clue: vec![(40,vec![(39,41) ,(48,32), (49,31), (50,30)])],
    };

    sudoku = Sudoku::new(9, vec![super::square_rule::SquareRule::new()]);
    sudoku.set_cell(6, 3).unwrap();
    sudoku.set_cell(2, 10).unwrap();
    sudoku.set_cell(2, 33).unwrap();
    sudoku.set_cell(1, 50).unwrap();
    sudoku.set_cell(4, 66).unwrap();


    multi_remove_indecies = zipper_rule.multi_remove(&sudoku,  &mut big_buffer);
    println!("{multi_remove_indecies:?}");
    for _ in 0..10{
        if let Some((value,indecies)) = zipper_rule.locked_candidate(&sudoku, &mut buffer, &mut arena){
            println!("{value} {indecies:?}");
            for index in indecies {
                sudoku.cells[*index].available.retain(|i| *i != value);
            }
        }
    }
    
    println!("{sudoku}");
}