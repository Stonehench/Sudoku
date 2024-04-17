use rand::random;
use solver::sudoku::Sudoku;

use crate::appstate::get_state;

pub fn hint(free_indexes: Vec<usize>, sudoku: &mut Sudoku) -> Option<(u16, usize)> {
    let hint_index = free_indexes[random::<usize>() % free_indexes.len()];
    let hint_value = get_state().current_sudoku.as_ref().unwrap().0.cells[hint_index].available[0];
    sudoku.set_cell(hint_value, hint_index).unwrap();
    return Some((hint_value, hint_index));
}
