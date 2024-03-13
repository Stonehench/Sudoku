use std::{env::args, fs, time::Instant};

use rand::random;
use rules::square_rule::SquareRule;
use sudoku::Sudoku;

use crate::{rules::knight_rule::KnightRule, sudoku::DynRule};

mod rules;
mod sudoku;

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
    const COUNT: u32 = 10;

    fn random_rules() -> Vec<DynRule> {
        let mut rules: Vec<DynRule> = vec![];
        if random() {
            //rules.push(DiagonalRule::new());
        }
        if random() {
            rules.push(SquareRule::new());
        }
        if random() {
            rules.push(KnightRule::new());
        }

        rules
    }

    println!("Generating...");
    let mut sudokus: Vec<Sudoku> = (0..COUNT)
        .filter_map(|_| Sudoku::generate_with_size(9, random_rules(), None).ok())
        .collect();
    println!("Generated {} sudokus", sudokus.len());

    let timer = Instant::now();
    for sudoku in &mut sudokus {
        println!("Solving with: {:#?}", sudoku.rules);
        sudoku.solve(None, None).unwrap();
    }
    let avg_time = timer.elapsed() / sudokus.len() as u32;
    println!("Avg solve time for {} solves: {avg_time:?}", sudokus.len());

    if also_generate {
        const GEN_COUNT: u32 = COUNT / 50;

        let timer = Instant::now();
        for _ in 0..GEN_COUNT {
            Sudoku::generate_with_size(4, vec![Box::new(SquareRule)], None).unwrap();
        }
        let avg_time_4x4 = timer.elapsed() / GEN_COUNT;
        println!("Avg generate time for {GEN_COUNT} 4x4: {avg_time_4x4:?}");

        let timer = Instant::now();
        for _ in 0..GEN_COUNT {
            Sudoku::generate_with_size(9, vec![Box::new(SquareRule)], None).unwrap();
        }
        let avg_time_9x9 = timer.elapsed() / GEN_COUNT;
        println!("Avg generate time for {GEN_COUNT} 9x9: {avg_time_9x9:?}");

        let timer = Instant::now();
        for _ in 0..GEN_COUNT {
            Sudoku::generate_with_size(16, vec![Box::new(SquareRule)], None).unwrap();
        }
        let avg_time_16x16 = timer.elapsed() / GEN_COUNT;
        println!("Avg generate time for {GEN_COUNT} 16x16: {avg_time_16x16:?}");
    }
}
