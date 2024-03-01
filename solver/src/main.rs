use std::{
    env::{self, args},
    fs,
    time::Instant,
};

use sudoku::Sudoku;

mod rules;
mod sudoku;

fn main() {
    let pre_read = Instant::now();
    let Some(input_filename) = args().nth(1) else {
        println!("Needs filename as arg");
        return;
    };
    if input_filename == "--benchmark" {
        benchmark();
        return;
    }

    let Ok(file_source) = fs::read_to_string(&input_filename) else {
        println!("Failed to read file");
        return;
    };

    let post_read = pre_read.elapsed();

    let pre_parse = Instant::now();

    let mut sudoku: Sudoku = match file_source.parse() {
        Ok(sudoku) => sudoku,
        Err(err) => {
            println!("Failed to parse file {err}");
            return;
        }
    };

    let post_parse = pre_parse.elapsed();

    let pre_solve = Instant::now();

    sudoku.solve();

    let solve_time = pre_solve.elapsed();

    println!("Solution:{sudoku}\n");
    println!("Read file in {post_read:?}");
    println!("parsed file in {post_parse:?}");
    println!("Solved in {solve_time:?}");
}

fn benchmark() {
    let mut path = env::current_dir().unwrap();
    if !path.ends_with("solver") {
        path.push("solver");
    }
    path.push("./sudokuBenchmark");
    let source = fs::read_to_string(path).unwrap();

    let sudoku: Sudoku = source.parse().unwrap();

    const COUNT: u32 = 1000;

    let clones: Vec<_> = (0..COUNT).map(|_| sudoku.clone()).collect();

    let timer = Instant::now();
    for mut sudoku in clones {
        sudoku.solve();
    }
    let avg_time = timer.elapsed() / COUNT;
    println!("Avg solve time for {COUNT} solves: {avg_time:?}");
}
