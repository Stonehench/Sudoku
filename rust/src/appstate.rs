use std::sync::{Mutex, MutexGuard};

use lazy_static::lazy_static;
use solver::sudoku::Sudoku;

pub struct AppState {
    pub current_sudoku: Option<(Sudoku, Sudoku)>,
}

lazy_static! {
    static ref STATE: Mutex<AppState> = Mutex::new(AppState {
        current_sudoku: None
    });
}

pub fn get_state() -> MutexGuard<'static, AppState> {
    STATE.lock().unwrap()
}
