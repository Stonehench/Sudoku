use std::sync::{Mutex, MutexGuard};

use lazy_static::lazy_static;
use solver::sudoku::Sudoku;

use crate::frb_generated::StreamSink;

pub struct AppState {
    pub current_sudoku: Option<(Sudoku, Sudoku)>,
    pub x_positions: Vec<(usize, usize)>,
    pub parity_positions: Vec<(usize, usize)>,
    pub consecutive_positions: Vec<(usize, usize)>,
    pub zipper_positions: Vec<(usize, Vec<(usize, usize)>)>,
    pub progress_sink: Option<StreamSink<(usize, usize)>>,
    pub thermometer_positions: Vec<Vec<u16>>,
}

lazy_static! {
    static ref STATE: Mutex<AppState> = Mutex::new(AppState {
        current_sudoku: None,
        x_positions: vec![],
        parity_positions: vec![],
        consecutive_positions: vec![],
        zipper_positions: vec![],
        progress_sink: None,
        thermometer_positions: vec![],
    });
}

pub fn get_state() -> MutexGuard<'static, AppState> {
    STATE.lock().unwrap()
}
