// This file is all the logic and tests pertaining to the Thermometer rule
// Author Kasper s224776

use rand::random;

use super::{DynRule, Rule};
use std::fmt::Debug;

use crate::sudoku::Sudoku;

#[derive(Debug, Clone)]
pub struct ThermometerRule {
    // Vector of themometers contaning indexes in rising order
    pub themometer_clue: Vec<Vec<usize>>,
}

impl ThermometerRule {
    pub fn new(themometer_clue: Vec<Vec<usize>>) -> DynRule {
        DynRule(Box::new(ThermometerRule { themometer_clue }))
    }
}

impl Rule for ThermometerRule {
    // Update function for thermometer rule
    // The input consists of the index of the digit that is being placed, a buffer, and the size of the Sudoku
    // Returns all indices of cells on the thermometer.
    fn updates<'buf>(
        &self,
        _size: usize,
        index: usize,
        buffer: &'buf mut Vec<usize>,
    ) -> &'buf [usize] {
        buffer.clear();

        // For each thermometer check if the index placed is on the thermometer
        // If true add all indices belonging to the thermometer to the buffer
        for themometer in &self.themometer_clue {
            if themometer.into_iter().any(|e| e == &index) {
                for element in themometer {
                    buffer.push(*element);
                }
            }
        }
        buffer
    }

    // Finds hidden singles for the thermometer rule
    // Takes the list of thermometers and the Sudoku
    // Returns an option contaning the value and index of a hidden single
    fn hidden_singles(&self, sudoku: &Sudoku) -> Option<(u16, usize)> {
        // Interrates through all cells in all sudoku
        for themometer in &self.themometer_clue {
            for (enumeration, index) in themometer.iter().enumerate() {
                // If the current index is the first cell in the thermometer and is not locked in
                // And 1 is available while the next cell has locked in 2
                // The current cell can only be 1
                if !sudoku.cells[*index].locked_in
                    && enumeration + 1 < themometer.len()
                    && index == &themometer[0]
                    && sudoku.cells[themometer[enumeration + 1]].locked_in
                    && sudoku.cells[themometer[enumeration + 1]].available[0] == 2
                    && sudoku.cells[*index].available.contains(&1)
                {
                    return Some(((1), *index));
                }

                // If the current index is the last cell in the thermometer and is not locked in
                // And the largest digit is avaliable while the previous cell has locked in the next largest digit
                // The current cell can only be the largest digit
                if !sudoku.cells[*index].locked_in
                    && enumeration > 0
                    && index == themometer.last().unwrap()
                    && sudoku.cells[themometer[enumeration - 1]].locked_in
                    && sudoku.cells[themometer[enumeration - 1]].available[0]
                        == sudoku.size as u16 - 1
                    && sudoku.cells[*index]
                        .available
                        .contains(&(sudoku.size as u16))
                {
                    return Some(((sudoku.size as u16), *index));
                }

                // If the current index is on the thermometer but neither at the start or the end
                // And the next and privous cell are locked in and have a difference of 2
                // The current cell can only the only number between those 2 numbers
                if !sudoku.cells[*index].locked_in
                    && enumeration < themometer.len() - 1
                    && enumeration > 0
                {
                    let prev_index = themometer[enumeration - 1];
                    let next_index = themometer[enumeration + 1];
                    if sudoku.cells[next_index].locked_in
                        && sudoku.cells[prev_index].locked_in
                        && sudoku.cells[next_index].available[0]
                            - sudoku.cells[prev_index].available[0]
                            == 2
                    {
                        return Some((
                            (sudoku.cells[themometer[enumeration - 1]].available[0] + 1),
                            *index,
                        ));
                    }
                }
            }
        }
        None
    }

    fn needs_square_for_locked(&self) -> bool {
        // There are no locked, so false
        false
    }

    // The multi remove function for the thermometer rule
    // Takes the thermometer list, the Sudoku and a buffer
    // Returns a list of all indencies with all the values that should be removed for all thermometers
    fn multi_remove<'buf>(
        &self,
        sudoku: &Sudoku,
        big_buffer: &'buf mut Vec<(u16, usize)>,
    ) -> &'buf [(u16, usize)] {
        big_buffer.clear();

        // Iterate through all cells in all thermometers
        for themometer in &self.themometer_clue {
            for (enumeration, index) in themometer.into_iter().enumerate() {
                // If the cell is not locked in
                if !sudoku.cells[*index].locked_in {
                    // Remove all values from 1 to the thermometer index(enumeration)
                    // This due to if we are on enumeration 3, it cannot have value 1 or 2
                    // because those will have to be before this enumeration.
                    for value in 1..(enumeration + 1) as u16 {
                        if sudoku.cells[*index].available.contains(&value) {
                            big_buffer.push((value, *index));
                        }
                    }
                    // Removes all values from the current enumeration to the size of the Sudoku
                    // Same logic as above just for the end of the thermometer
                    for value in (sudoku.size - (themometer.len() - enumeration) + 2) as u16
                        ..(sudoku.size + 1) as u16 
                    {
                        if sudoku.cells[*index].available.contains(&value) {
                            big_buffer.push((value, *index));
                        }
                    }
                }

                // If the current cell is locked in get the value
                if sudoku.cells[*index].locked_in {
                    if let Some(value) = sudoku.cells[*index].available.get(0) {
                        // Iterate through the same thermometer again
                        for (inner_enumeration, inner_index) in themometer.into_iter().enumerate() {
                            // If the second index is before the currnet and the second is not locked in
                            if inner_enumeration > enumeration
                                && !sudoku.cells[*inner_index].locked_in
                            {
                                // Remove all values, lower than or equal to the current index, form the second index
                                for i in 1..*value + 1 {
                                    if sudoku.cells[*inner_index].available.contains(&i) {
                                        big_buffer.push((i, *inner_index))
                                    }
                                }
                                // If the second index is after the currnet and the second is not locked in
                            } else if inner_enumeration < enumeration
                                && !sudoku.cells[*inner_index].locked_in
                            {
                                // Remove all values, higher than or equal to the current index, form the second index
                                for i in *value..=sudoku.size as u16 {
                                    if sudoku.cells[*inner_index].available.contains(&i) {
                                        big_buffer.push((i, *inner_index))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        big_buffer
    }

    // The create clue function
    fn create_clue(&mut self, cells: &Vec<crate::sudoku::Cell>, size: usize) {
        let tries = size * 3;
        let mut seen = vec![];

        // Choose a random index
        // If the index has been seen before try again
        'themometers: for _ in 0..tries {
            let mut random_index = random::<usize>() % (size * size);
            while seen.contains(&random_index) && seen.len() < (size * size) {
                random_index = random::<usize>() % (size * size);
            }
            // Initialization of variables
            let mut current_themometer: Vec<usize> = vec![];
            let mut searching = true;
            let mut surrounding: Vec<usize> = vec![];
            let mut current_index: usize = random_index;
            let mut current_value = cells[random_index].available[0];
            current_themometer.push(current_index);

            if current_value == size as u16 {
                // The value at the bottom of a themometer can not be the highest value
                continue 'themometers;
            }

            // Add all surrouding indencies
            'searching: while searching {
                surrounding.clear();

                if current_index >= size {
                    //above
                    surrounding.push(current_index - size);
                }
                if !(current_index % size == 0) {
                    //left
                    surrounding.push(current_index - 1);
                }
                if current_index % size != (size - 1) {
                    //right
                    surrounding.push(current_index + 1);
                }
                if current_index < size * size - size {
                    //below
                    surrounding.push(current_index + size);
                }
                if current_index >= size && current_index % size != (size - 1) {
                    //above right
                    surrounding.push(current_index - size + 1);
                }
                if current_index < size * size - size && !(current_index % size == 0) {
                    //below left
                    surrounding.push(current_index + size - 1);
                }
                if current_index >= size && !(current_index % size == 0) {
                    //above left
                    surrounding.push(current_index - size - 1);
                }
                if current_index < size * size - size && current_index % size != (size - 1) {
                    //below right
                    surrounding.push(current_index + size + 1);
                }

                // Remove all indencies from surrouding with value higher than the current value
                surrounding.retain(|e| cells[*e].available[0] > current_value);
                // Sort for lowest value
                surrounding.sort_by(|a, b| cells[*a].available[0].cmp(&cells[*b].available[0]));

                // If there is a value in surrinding that has not been seen and i larger than current value
                if !surrounding.is_empty()
                    && !seen.contains(&surrounding[0])
                    && cells[surrounding[0]].available[0] > current_value
                {
                    // Update seen and current thermometer to include the first value of surrounding
                    // Then use the new index as current index and continue
                    seen.push(surrounding[0]);
                    current_themometer.push(surrounding[0]);
                    current_index = surrounding[0];
                    current_value = cells[surrounding[0]].available[0];

                    continue 'searching;
                }

                searching = false;
            }
            // If the thermomter has length greater than 1, add it as a clue
            if current_themometer.len() > 1 {
                seen.push(random_index);
                self.themometer_clue.push(current_themometer);
            }
        }
    }

    fn boxed_clone(&self) -> DynRule {
        DynRule(Box::new(self.clone()))
    }

    fn get_name(&self) -> &'static str {
        "ThemometerRule"
    }

    fn to_thermometer_rule(&mut self) -> Option<&mut ThermometerRule> {
        Some(self)
    }

    fn no_of_clues(&self) -> usize {
        return self.themometer_clue.len();
    }

    // Prints the PSF part of the thermometer (PSF is explained in the report)
    fn print_self(&self) -> bool {
        print!("ThermometerRule");
        for ther in &self.themometer_clue {
            print!(" ;");
            for (l_index, index) in ther.iter().enumerate() {
                print!("{index}");
                if l_index != ther.len() - 1 {
                    print!(",");
                }
            }
        }
        true
    }
}

//########################### TEST ###############################

#[test]
fn themometer_test() {
    let sudoku = Sudoku::new(9, vec![]);

    let themometer_rule = ThermometerRule {
        themometer_clue: vec![vec![0 as usize, 1 as usize, 2 as usize]],
    };

    let mut buffer = vec![];
    let mut indexes = themometer_rule.updates(sudoku.size, 1, &mut buffer);

    assert_eq!(indexes, vec![0, 1, 2]);

    indexes = themometer_rule.updates(sudoku.size, 5, &mut buffer);

    assert_eq!(indexes, vec![]);
}

#[test]
fn themometer_multi_remove_test() {
    let mut sudoku = Sudoku::new(9, vec![]);

    let mut themometer_rule = ThermometerRule {
        themometer_clue: vec![vec![0 as usize, 1 as usize, 2 as usize, 3 as usize]],
    };

    let mut big_buffer = vec![];
    let mut indexes = themometer_rule.multi_remove(&sudoku, &mut big_buffer);

    // first run the multi-remove on an empty thermometer
    for (value, index) in indexes {
        sudoku.cells[*index].available.retain(|i| *i != *value);
    }
    assert_eq!(
        indexes,
        vec![
            (7, 0),
            (8, 0),
            (9, 0),
            (1, 1),
            (8, 1),
            (9, 1),
            (1, 2),
            (2, 2),
            (9, 2),
            (1, 3),
            (2, 3),
            (3, 3)
        ]
    );

    // set a cell on the thermometer
    sudoku.set_cell(3, 2).unwrap();
    indexes = themometer_rule.multi_remove(&sudoku, &mut big_buffer);

    assert_eq!(
        indexes,
        vec![(4, 0), (5, 0), (6, 0), (4, 1), (5, 1), (6, 1), (7, 1),]
    );

    sudoku = Sudoku::new(9, vec![]);
    sudoku.set_cell(3, 2).unwrap();

    println!("");
    themometer_rule = ThermometerRule {
        themometer_clue: vec![vec![3 as usize, 2 as usize, 1 as usize, 0 as usize]],
    };

    big_buffer.clear();
    let indexes = themometer_rule.multi_remove(&sudoku, &mut big_buffer);
    assert_eq!(
        indexes,
        vec![
            (7, 3),
            (8, 3),
            (9, 3),
            (4, 3),
            (5, 3),
            (6, 3),
            (7, 3),
            (8, 3),
            (9, 3),
            (1, 1),
            (2, 1),
            (1, 0),
            (2, 0),
            (1, 1),
            (2, 1),
            (9, 1),
            (1, 0),
            (2, 0)
        ]
    );
}

#[test]
fn themometer_hidden_single_test() {
    // test 1
    let mut sudoku = Sudoku::new(9, vec![]);

    let themometer_rule = ThermometerRule {
        themometer_clue: vec![vec![0 as usize, 1 as usize, 2 as usize]],
    };

    sudoku.set_cell(1, 0).unwrap();
    sudoku.set_cell(3, 2).unwrap();

    let result = themometer_rule.hidden_singles(&sudoku);

    assert_eq!(result, Some((2, 1)));

    // test 2
    let mut sudoku = Sudoku::new(9, vec![]);

    let themometer_rule = ThermometerRule {
        themometer_clue: vec![vec![
            0 as usize, 1 as usize, 2 as usize, 3 as usize, 4 as usize,
        ]],
    };

    sudoku.set_cell(1, 0).unwrap();
    sudoku.set_cell(3, 1).unwrap();
    sudoku.set_cell(7, 3).unwrap();
    sudoku.set_cell(9, 4).unwrap();

    let result = themometer_rule.hidden_singles(&sudoku);

    assert_eq!(result, None);

    // test 3
    let mut sudoku = Sudoku::new(9, vec![]);

    let themometer_rule = ThermometerRule {
        themometer_clue: vec![vec![0 as usize, 1 as usize, 2 as usize]],
    };

    sudoku.set_cell(2, 1).unwrap();

    let result = themometer_rule.hidden_singles(&sudoku);

    assert_eq!(result, Some((1, 0)));

    // test 4
    let mut sudoku = Sudoku::new(9, vec![]);

    let themometer_rule = ThermometerRule {
        themometer_clue: vec![vec![0 as usize, 1 as usize, 2 as usize]],
    };

    sudoku.set_cell(1, 0).unwrap();
    sudoku.set_cell(8, 1).unwrap();

    let result = themometer_rule.hidden_singles(&sudoku);

    assert_eq!(result, Some((9, 2)));
}
