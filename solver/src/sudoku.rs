use std::{
    cmp::Ordering,
    fmt::{Display, Write},
    num::ParseIntError,
    ops::Range,
    str::FromStr,
};

use integer_sqrt::IntegerSquareRoot;
use priority_queue::PriorityQueue;
use rand::random;
use regex_macro::regex;

use crate::rules::{ColumnRule, KnightRule, RowRule, Rule, SquareRule};

#[derive(Debug)]
pub struct Sudoku {
    pub size: usize,
    pub cells: Vec<Cell>,
    pub rules: Vec<Box<dyn Rule>>,
}

//Det her er ret fucked, men siden vi skal have den laveste entropy ud af vores priority queue skal den sammenligne omvendt
// siden priority_queue tager den med størst priority lol
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Entropy(usize);

//Sammenligning ift større / mindre men reversed
impl PartialOrd for Entropy {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.0.partial_cmp(&self.0)
        //self.0.partial_cmp(&other.0)
    }
}

impl Ord for Entropy {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other).unwrap()
    }
}
#[derive(Debug, Clone, Copy)]
pub enum SudokuSolveError {
    UnsolveableError,
    RemovedLockedValue,
}

impl Display for SudokuSolveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SudokuSolveError::UnsolveableError => write!(f, "No Solution. Failed to pop branch queue when entropy was 0 in cell"),
            SudokuSolveError::RemovedLockedValue => write!(f, "Something went seriously wrong. Removed the only value in a locked cell\nThis indicates either an unsolveable sudoku or a bug in the rules."),
        }
    }
}

impl Sudoku {
    pub fn new(size: usize, rules: Vec<Box<dyn Rule>>) -> Self {
        Self {
            size,
            cells: (0..size * size)
                .map(|_| Cell::new_with_range(1..(size as u16 + 1)))
                .collect(),
            rules,
        }
    }

    pub fn set_cell(&mut self, n: u16, index: usize) -> Result<(), SudokuSolveError> {
        let mut ret_buffer = vec![];
        self.cells[index] = Cell::single(n);
        for rule in &self.rules {
            for inner_index in rule
                .updates(self.size, index, &mut ret_buffer)
                .into_iter()
                .filter(|i| **i != index)
            {
                self.cells[*inner_index].remove(n)?;
            }
        }
        Ok(())
    }

    fn update_cell(
        &mut self,
        n: u16,
        index: usize,
        queue: &mut PriorityQueue<usize, Entropy>,
        ret_buffer: &mut Vec<usize>,
    ) -> Result<(), SudokuSolveError> {
        self.cells[index] = Cell::single(n);
        for rule in &self.rules {
            for inner_index in rule
                .updates(self.size, index, ret_buffer)
                .into_iter()
                .filter(|i| **i != index)
            {
                let cell = &mut self.cells[*inner_index];
                cell.remove(n)?;
                queue.change_priority(&inner_index, Entropy(cell.available.len()));
            }
        }
        Ok(())
    }

    pub fn solve(&mut self) -> Result<(), SudokuSolveError> {
        #[cfg(debug_assertions)]
        let mut branch_count = 0;
        #[cfg(debug_assertions)]
        let mut backtracks = 0;

        let mut pri_queue = PriorityQueue::new();
        for (index, cell) in self.cells.iter().enumerate() {
            if !cell.locked_in {
                pri_queue.push(index, Entropy(cell.available.len()));
            }
        }

        let mut branch_stack: Vec<(Vec<Cell>, PriorityQueue<usize, Entropy>)> = vec![];
        let mut ret_buffer = vec![];

        'main: while let Some((index, entropy)) = pri_queue.pop() {
            match entropy.0 {
                0 => {
                    //Der er ingen løsning på den nuværende branch. Derfor popper vi en branch og løser den i stedet
                    let Some((cells, new_pri_queue)) = branch_stack.pop() else {
                        return Err(SudokuSolveError::UnsolveableError);
                    };

                    self.cells = cells;
                    pri_queue = new_pri_queue;

                    #[cfg(debug_assertions)]
                    {
                        backtracks += 1;
                    }
                }
                1 => self.update_cell(
                    self.cells[index].available[0],
                    index,
                    &mut pri_queue,
                    &mut ret_buffer,
                )?,
                _ => {
                    // Der er ikke flere naked singles, så der tjekkes for hidden singles

                    for rule in &self.rules {
                        if let Some((n, hidden_index)) = rule.hidden_singles(self) {
                            //Put nuværende cell tilbage i priority queue

                            pri_queue.push(index, entropy);
                            pri_queue.remove(&hidden_index);
                            self.update_cell(n, hidden_index, &mut pri_queue, &mut ret_buffer)?;

                            continue 'main;
                        }
                    }

                    //Der er flere muligheder for hvad der kan vælges. Derfor pushes state på branch stacken og der vælges en mulighed
                    //Vælg random
                    let choice = random::<usize>() % entropy.0;

                    let n = self.cells[index].available[choice];

                    let mut cloned_cells = self.cells.clone();

                    //Fjern n fra cloned_cells så den ikke kan blive valgt igen!
                    cloned_cells[index].available.remove(choice);

                    let mut cloned_queue = pri_queue.clone();
                    //Siden den allerede er poppet i den nuværende queue skal den indsættes igen
                    // i den cloned queue. Ellers vil clonen aldrig løse index cellen.
                    cloned_queue.push(index, Entropy(entropy.0 - 1));
                    branch_stack.push((cloned_cells, cloned_queue));

                    self.update_cell(n, index, &mut pri_queue, &mut ret_buffer)?;

                    #[cfg(debug_assertions)]
                    {
                        branch_count += 1;
                    }
                }
            }
        }

        #[cfg(debug_assertions)]
        {
            println!("branch count: {branch_count}");
            println!("backtracks: {backtracks}");
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum ParseSudokuError {
    ParseIntError(ParseIntError),
    InvalidSizeError(usize),
    UnsolveableError,
    InvalidRuleName(String),
}

impl Display for ParseSudokuError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl FromStr for Sudoku {
    type Err = ParseSudokuError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rules: Vec<Box<dyn Rule>> = vec![
            Box::new(RowRule),
            Box::new(ColumnRule),
            Box::new(SquareRule),
        ];

        //WTFFF
        let sudoku_source = match regex!(r"(\r\n|\n)(\r\n|\n)")
            .split(s)
            .collect::<Vec<&str>>().as_slice()
        {
            [rules_source, sudoku] => {
                for rule_name in rules_source.split_whitespace() {
                    rules.push(match rule_name {
                        "KnightsMove" => Box::new(KnightRule),
                        invalid => {
                            return Err(ParseSudokuError::InvalidRuleName(invalid.to_owned()))
                        }
                    });
                }

                sudoku
            }
            _ => s,
        };

        let size = sudoku_source.split(',').count().integer_sqrt();
        let sub_size = size.integer_sqrt();
        if sub_size * sub_size != size {
            return Err(ParseSudokuError::InvalidSizeError(size));
        }

        #[cfg(debug_assertions)]
        println!("parsing size: {size}");

        let mut sudoku = Sudoku::new(size, rules);

        for (index, part) in sudoku_source.split(',').map(str::trim).enumerate() {
            let n = part
                .parse()
                .map_err(|e| ParseSudokuError::ParseIntError(e))?;
            if n != 0 {
                sudoku
                    .set_cell(n, index)
                    .map_err(|_| ParseSudokuError::UnsolveableError)?;
            }
        }

        Ok(sudoku)
    }
}

impl Display for Sudoku {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (index, cell) in self.cells.iter().enumerate() {
            if index % self.size == 0 {
                f.write_char('\n')?;
            }
            write!(f, "{:?},", cell.available)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Cell {
    pub available: Vec<u16>,
    pub locked_in: bool,
}

impl Cell {
    fn single(n: u16) -> Self {
        Self {
            available: vec![n],
            locked_in: true,
        }
    }
    fn new_with_range(range: Range<u16>) -> Self {
        Self {
            available: range.collect(),
            locked_in: false,
        }
    }
    fn remove(&mut self, n: u16) -> Result<(), SudokuSolveError> {
        self.available.retain(|i| *i != n);
        if self.locked_in && self.available.len() == 0 {
            return Err(SudokuSolveError::RemovedLockedValue);
        }
        Ok(())
    }
    #[allow(unused)]
    pub fn is_single_eq(&self, n: u16) -> bool {
        self.available == [n]
    }
}

impl Clone for Sudoku {
    fn clone(&self) -> Self {
        Self {
            size: self.size.clone(),
            cells: self.cells.clone(),
            rules: self.rules.iter().map(|r| r.boxed_clone()).collect(),
        }
    }
}

#[test]
fn read_file_test() {
    let file_str = std::fs::read_to_string("./sudokuBenchmark").unwrap();
    let sudoku: Sudoku = file_str.parse().unwrap();

    println!("{sudoku}");
}

#[test]
fn solve_big_sudoku() {
    // tests a 16x16 sudoku solve

    let file_str = std::fs::read_to_string("./sudoku16").unwrap();
    let mut sudoku: Sudoku = file_str.parse().unwrap();

    println!("{sudoku}");
    sudoku.solve().unwrap();
    println!("{sudoku}");
}

#[test]
fn solve_test() {
    use std::collections::HashMap;
    use std::fs;

    let sudokus = fs::read_dir("./tests").unwrap();

    let mut solutions = HashMap::<String, String>::new();

    for file in sudokus.map(Result::unwrap) {
        let filename = file.file_name().to_string_lossy().to_string();
        let sudoku_name;

        let mut sudoku: Sudoku = fs::read_to_string(file.path()).unwrap().parse().unwrap();

        if filename.contains("Løsning") {
            sudoku_name = filename.split_whitespace().next().unwrap().to_string();
        } else {
            sudoku.solve().unwrap();
            sudoku_name = filename;
        }

        let solution = sudoku.to_string();
        if let Some(other_solution) = solutions.get(&sudoku_name) {
            assert_eq!(solution, *other_solution);
            println!("{sudoku_name} solved correctly");
        } else {
            solutions.insert(sudoku_name, solution);
        }
    }

    for (key, value) in solutions {
        println!("{key}: {value}");
    }
}

#[test]
fn random_gen() {
    let mut sudoku = Sudoku::new(
        9,
        vec![
            Box::new(RowRule),
            Box::new(ColumnRule),
            Box::new(SquareRule),
        ],
    );
    sudoku.solve().unwrap();
    let pre = sudoku.to_string();
    println!("Pre:\n{}", pre);

    let difficulty = 10;

    for _ in 0..difficulty {
        let index = random::<usize>() % sudoku.cells.len();
        sudoku.cells[index] = Cell::new_with_range(1..(sudoku.size as u16 + 1))
    }
    for cell in &mut sudoku.cells {
        cell.locked_in = false;
    }

    let post = sudoku.to_string();
    println!("Post:\n{}", post);

    sudoku.solve().unwrap();

    assert_eq!(pre, sudoku.to_string());

    println!("PostPost:\n{sudoku}");
}

#[test]
fn solve_16x_test() {
    let file_str = std::fs::read_to_string("./sudoku16x16").unwrap();
    let mut sudoku: Sudoku = file_str.parse().unwrap();

    sudoku.solve().unwrap();

    println!("{sudoku}");
}

#[test]
fn solve_knights_move_sudoku() {
    let file_str = std::fs::read_to_string("./sudokuKnightsMove").unwrap();
    let mut sudoku: Sudoku = file_str.parse().unwrap();

    println!("{sudoku}");

    sudoku.solve().unwrap();

    println!("{sudoku}");
    assert_eq!(
        sudoku.to_string().trim(),
        std::fs::read_to_string("./sudokuKnightsMoveSolution")
            .unwrap()
            .replace("\r\n", "\n")
            .trim()
    );
}
