use rand::random;

use crate::appstate::get_state;

pub fn hint(free_indexes: Vec<u16>) -> (u16, usize) {
    let hint_index = free_indexes[random::<usize>() % free_indexes.len()] as usize;
    let hint_value = get_state().current_sudoku.as_ref().unwrap().1.cells[hint_index].available[0];
    return (hint_value, hint_index);
}
