// Author Thor s224817

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
    } else if input_filename == "--generate" {
        let difficulty = if let Some(diff_name) = args().nth(2) {
            diff_name
                .parse()
                .expect(&format!("Failed to parse difficulty {diff_name}"))
        } else {
            Difficulty::Medium
        };

        let (sudoku, _) =
            Sudoku::generate_with_size(9, vec![SquareRule::new()], None, difficulty).unwrap();

        println!("SquareRule\n");

        for (index, cell) in sudoku.cells.iter().enumerate() {
            match cell.available.as_slice() {
                [value] => print!("{value}"),
                _ => print!("0"),
            }
            if index + 1 < sudoku.cells.len() {
                print!(",");
            }
        }
        println!("");
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

    const N: u32 = 0;

    let post_parse = pre_parse.elapsed();

    let pre_solve = Instant::now();

    for _ in 0..N {
        let mut sud = sudoku.clone();
        sud.solve(None, None,None).unwrap();
    }

    sudoku.solve(None, None,None).unwrap();

    let solve_time = pre_solve.elapsed() / (N + 1);

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
        sudoku.solve(None, None,None).unwrap();
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
