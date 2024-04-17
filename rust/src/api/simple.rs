use std::ops::Deref;

use solver::rules::DynRule;
use solver::sudoku::{AllSolutionsContext, Difficulty, Sudoku};

use crate::appstate::get_state;
use crate::frb_generated::StreamSink;

pub fn progress(sink: StreamSink<(usize, usize)>) {
    let mut state = get_state();
    state.progress_sink = Some(sink);
}

pub fn generate_with_size(
    size: usize,
    rules_src: Vec<String>,
    difficulty: String,
) -> Option<String> {
    let rules = rules_src
        .iter()
        .map(Deref::deref)
        .map(str::parse::<DynRule>)
        .collect::<Result<Vec<_>, _>>()
        .ok()?;

    let Ok(difficulty): Result<Difficulty, _> = difficulty.parse() else {
        eprintln!("Failed to parse difficulty: \"{difficulty}\"");
        return None;
    };

    let progress = Box::new(move |progress| {
        let state = get_state();
        if let Some(sink) = &state.progress_sink {
            if let Err(err) = sink.add((progress, difficulty.get_removes(size))) {
                eprintln!("Failed writing to progress sink: {err}");
            }
        } else {
            eprintln!("Progress sink missing at {progress}");
        }
    });

    let Ok(mut sudoku) = Sudoku::generate_with_size(size, rules, Some(progress), difficulty) else {
        println!("Sudoku generation failed!");
        return None;
    };

    for (index, cell) in sudoku.cells.iter().enumerate() {
        if cell.available.len() == 1 {
            print!("{}", cell.available[0]);
        } else {
            print!("0");
        }
        print!(",");
        if index % sudoku.size == 0 {
            println!("");
        }
    }

    let mut state = get_state();
    state.x_positions = vec![];
    state.parity_positions = vec![];
    state.zipper_positions = vec![];
    state.consecutive_positions = vec![];
    state.thermometer_positions = vec![];

    if let Some(x_rule) = sudoku.rules.iter_mut().find_map(|r| r.to_x_rule()) {
        state.x_positions = x_rule.x_clue.clone();
    }

    if let Some(parity_rule) = sudoku.rules.iter_mut().find_map(|r| r.to_parity_rule()) {
        state.parity_positions = parity_rule.parity_clue.clone();
    }

    if let Some(consecutive_rule) = sudoku
        .rules
        .iter_mut()
        .find_map(|r| r.to_consecutive_rule())
    {
        state.consecutive_positions = consecutive_rule.consecutive_clue.clone();
    }

    if let Some(zipper_rule) = sudoku.rules.iter_mut().find_map(|r| r.to_zipper_rule()) {
        state.zipper_positions = zipper_rule.zipper_clue.clone();
    }

    if let Some(thermometer_rule) = sudoku
        .rules
        .iter_mut()
        .find_map(|r| r.to_thermometer_rule())
    {
        state.thermometer_positions = thermometer_rule
            .themometer_clue
            .iter()
            .map(|ls| ls.iter().map(|u| *u as u16).collect())
            .collect();
    }

    let mut solved = sudoku.clone();
    if let Err(err) = solved.solve(None, None) {
        println!("Failed to solve generated sudoku: {err}");
        return None;
    };

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

pub fn get_consecutive_positions() -> Vec<(usize, usize)> {
    get_state().consecutive_positions.clone()
}

pub fn get_parity_positions() -> Vec<(usize, usize)> {
    get_state().parity_positions.clone()
}

pub fn get_zipper_positions() -> Vec<(usize, Vec<(usize, usize)>)> {
    get_state().zipper_positions.clone()
}

pub fn get_thermometer_positions() -> Vec<Vec<u16>> {
    get_state().thermometer_positions.clone()
}

pub fn check_legality(position: usize, value: u16) -> bool {
    let state = get_state();
    let (_, sudoku) = state.current_sudoku.as_ref().unwrap();
    sudoku.cells[position].available.deref() == [value]
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

pub fn difficulty_values(size: usize, difficulty: String) -> Option<usize> {
    let difficulty: Difficulty = difficulty.parse().ok()?;

    Some(difficulty.get_removes(size))
}

pub fn set_from_str(sudoku: String) {
    let sudoku: Sudoku = sudoku.parse().unwrap();
    let mut solved = sudoku.clone();
    solved.solve(None, None).unwrap();

    let mut parity = vec![];
    let mut zippers = vec![];
    let mut x = vec![];
    let mut consecutive = vec![];
    let mut thermometers = vec![];

    if let Some(parity_rule) = solved.rules.iter_mut().find_map(|r| r.to_parity_rule()) {
        parity = parity_rule.parity_clue.clone();
    }

    if let Some(zipper_rule) = solved.rules.iter_mut().find_map(|r| r.to_zipper_rule()) {
        zippers = zipper_rule.zipper_clue.clone();
    }

    if let Some(thermometer_rule) = solved
        .rules
        .iter_mut()
        .find_map(|r| r.to_thermometer_rule())
    {
        thermometers = thermometer_rule
            .themometer_clue
            .iter()
            .map(|thermo| thermo.iter().map(|i| *i as u16).collect())
            .collect();
    }

    if let Some(x_rule) = solved.rules.iter_mut().find_map(|r| r.to_x_rule()) {
        x = x_rule.x_clue.clone();
    }

    if let Some(consecutive_rule) = solved
        .rules
        .iter_mut()
        .find_map(|r| r.to_consecutive_rule())
    {
        consecutive = consecutive_rule.consecutive_clue.clone();
    }

    let mut state_lock = get_state();
    state_lock.current_sudoku = Some((sudoku, solved));
    state_lock.zipper_positions = zippers;
    state_lock.parity_positions = parity;
    state_lock.x_positions = x;
    state_lock.consecutive_positions = consecutive;
    state_lock.thermometer_positions = thermometers;
}
