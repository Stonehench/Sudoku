use std::{
    env::{self, args},
    fs,
    time::Instant,
};

use rules::square_rule::SquareRule;
use sudoku::Sudoku;

use crate::sudoku::Difficulty;

pub mod rules;
pub mod sudoku;

fn main() {
    let pre_read = Instant::now();
    let Some(input_filename) = args().nth(1) else {
        println!("Needs filename as arg");
        return;
    };
    if input_filename == "--benchmark" {
        benchmark(args().nth(2) == Some("generate".into()));
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

    sudoku.solve(None, None).unwrap();

    let solve_time = pre_solve.elapsed();

    println!("Solution:{sudoku}\n");
    println!("Read file in {post_read:?}");
    println!("parsed file in {post_parse:?}");
    println!("Solved in {solve_time:?}");
}

fn benchmark(also_generate: bool) {
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
        sudoku.solve(None, None).unwrap();
    }
    let avg_time = timer.elapsed() / COUNT;
    println!("Avg solve time for {COUNT} solves: {avg_time:?}");

    if also_generate {
        const GEN_COUNT: u32 = COUNT / 50;

        let timer = Instant::now();
        for _ in 0..GEN_COUNT {
            Sudoku::generate_with_size(4, vec![SquareRule::new()], None, Difficulty::Expert)
                .unwrap();
        }
        let avg_time_4x4 = timer.elapsed() / GEN_COUNT;
        println!("Avg generate time for {GEN_COUNT} 4x4: {avg_time_4x4:?}");

        let timer = Instant::now();
        for _ in 0..GEN_COUNT {
            Sudoku::generate_with_size(9, vec![SquareRule::new()], None, Difficulty::Expert)
                .unwrap();
        }
        let avg_time_9x9 = timer.elapsed() / GEN_COUNT;
        println!("Avg generate time for {GEN_COUNT} 9x9: {avg_time_9x9:?}");

        let timer = Instant::now();
        for _ in 0..GEN_COUNT {
            Sudoku::generate_with_size(16, vec![SquareRule::new()], None, Difficulty::Expert)
                .unwrap();
        }
        let avg_time_16x16 = timer.elapsed() / GEN_COUNT;
        println!("Avg generate time for {GEN_COUNT} 16x16: {avg_time_16x16:?}");
    }
}
