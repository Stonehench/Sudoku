use solver::sudoku::{Cell, DynRule, Sudoku};

use solver::rules::*;

use crate::appstate::get_state;

#[flutter_rust_bridge::frb(sync)]
pub fn generate_with_size(size: usize, rules_src: Vec<String>) -> bool {
    let mut rules: Vec<DynRule> = vec![
        Box::new(RowRule),
        Box::new(SquareRule),
        Box::new(ColumnRule),
    ];
    for rule in rules_src {
        let rule = rule.parse::<DynRule>();
        if let Ok(rule) = rule {
            rules.push(rule);
        } else {
            return false;
        }
    }

    let mut sudoku = Sudoku::new(size, rules);
    sudoku.solve().unwrap();

    for _ in 0..(sudoku.size*sudoku.size) / 2 {
        let index = rand::random::<usize>() % sudoku.cells.len();
        sudoku.cells[index] = Cell::new_with_range(1..sudoku.size as u16 + 1);
    }

    let mut state = get_state();
    state.current_sudoku = Some(sudoku);

    true
}

#[flutter_rust_bridge::frb(sync)]
pub fn check_legality(position: usize, value: u16) -> bool {
    let state = get_state();
    let sudoku = state.current_sudoku.as_ref().unwrap();
    let mut buffer = vec![];
    sudoku
        .rules
        .iter()
        .all(|r| r.is_legal(sudoku, position, value, &mut buffer))
}

#[flutter_rust_bridge::frb(sync)]
pub fn get_sudoku_str() -> Option<String> {
    let state = get_state();

    let mut str_buffer = String::new();

    for cell in &state.current_sudoku.as_ref()?.cells {
        match cell.available.as_slice() {
            [value] => str_buffer.push_str(&value.to_string()),
            _ => str_buffer.push_str(&"0"),
        }
        str_buffer.push(',');
    }

    println!("Sending: {str_buffer}");

    Some(str_buffer)
}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}
