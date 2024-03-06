use solver::sudoku::{DynRule, Sudoku};

use solver::rules::*;

use crate::appstate::get_state;

pub fn generate_with_size(size: usize, rules_src: Vec<String>) -> Option<String> {
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
            return None;
        }
    }

    let sudoku = Sudoku::generate_with_size(size, rules);

    let mut solved = sudoku.clone();
    for cell in &mut solved.cells {
        cell.locked_in = false;
    }
    solved.solve(None, None).unwrap();

    let mut str_buffer = String::new();

    for cell in &sudoku.cells {
        match cell.available.as_slice() {
            [value] => str_buffer.push_str(&value.to_string()),
            _ => str_buffer.push('0'),
        }
        str_buffer.push(',');
    }

    println!("Sending: {str_buffer}");

    let mut state = get_state();
    state.current_sudoku = Some((sudoku, solved));

    Some(str_buffer)
}

pub fn check_legality(position: usize, value: u16) -> bool {
    let state = get_state();
    let (_, sudoku) = state.current_sudoku.as_ref().unwrap();
    sudoku.cells[position].available == [value]
}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}
