use std::ops::Deref;
use std::sync::mpsc::channel;
use std::sync::{mpsc, Mutex};
use std::time::Duration;

use lazy_static::lazy_static;
use solver::sudoku::{AllSolutionsContext, DynRule, Sudoku};

use crate::appstate::get_state;

pub fn generate_with_size(size: usize, rules_src: Vec<String>, difficulty: String) -> Option<String> {
    let mut state = get_state();
    state.x_positions = vec![];

    let rules = rules_src
        .iter()
        //.map(|s| insert_x_locations(size, s, &mut state)) //Savner f# currying lol
        .map(Deref::deref)
        .map(str::parse::<DynRule>)
        .collect::<Result<Vec<_>, _>>()
        .ok()?;

    let sender = PROGRESS.lock().unwrap().0.clone();
    let difficulty = difficulty.parse().ok()?;
    let mut sudoku = Sudoku::generate_with_size(size, rules, Some(sender), difficulty).ok()?;

    if let Some(x_rule) = sudoku.rules.iter_mut().find_map(|r| r.to_x_rule()) {
        state.x_positions = x_rule.x_clue.clone();
    }

    let mut solved = sudoku.clone();
    solved.solve(None, None).ok()?;

    let mut str_buffer = String::new();

    for cell in sudoku.cells.iter() {
        match cell.available.as_slice() {
            [value] => str_buffer.push_str(&value.to_string()),
            _ => str_buffer.push('0'),
        }
        str_buffer.push(',');
    }

    println!("Sending: {str_buffer}");

    state.current_sudoku = Some((sudoku, solved));

    Some(str_buffer)
}

pub fn get_x_positions() -> Vec<(usize, usize)> {
    get_state().x_positions.clone()
}

lazy_static! {
    static ref PROGRESS: Mutex<(mpsc::Sender<usize>, Option<mpsc::Receiver<usize>>)> =
        Mutex::new({
            let (sx, rx) = channel();

            (sx, Some(rx))
        });
}

pub fn wait_for_progess() -> Option<usize> {
    let rx = PROGRESS.lock().unwrap().1.take()?;

    let mut res = None;
    while let Ok(c_res) = rx.try_recv() {
        res = Some(c_res);
    }
    if res.is_none() {
        res = rx.recv_timeout(Duration::from_secs(5)).ok();
    }

    PROGRESS.lock().unwrap().1 = Some(rx);

    res
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

pub fn close_threads() {
    let pool = AllSolutionsContext::get_pool();
    drop(pool);
}
