use solver::sudoku::{DynRule, Sudoku};

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

    let mut state = get_state();
    state.current_sudoku = Some(sudoku);

    true
}

#[flutter_rust_bridge::frb(sync)]
pub fn get_sudoku_str() -> Option<String> {
    let state = get_state();
    let str = state.current_sudoku.as_ref()?.to_string();
    let str = str.replace('[', "");
    let str = str.replace(']', "");
    let str = str.replace('\n',"");

    println!("Sending: {str}");

    Some(str)
}

#[flutter_rust_bridge::frb(sync)] // Synchronous mode for simplicity of the demo
pub fn greet(name: String) -> String {
    format!("Hello, {name}!")
}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}
