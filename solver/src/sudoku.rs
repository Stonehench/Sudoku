use std::{
    cmp::Ordering, fmt::{Display, Write}, num::ParseIntError, ops::Range, str::FromStr, sync::{atomic::AtomicUsize, mpsc, Arc, Mutex}, time::Instant
};

use integer_sqrt::IntegerSquareRoot;
use lazy_static::lazy_static;
use priority_queue::PriorityQueue;
use rand::random;
use regex_macro::regex;
use threadpool::ThreadPool;

use crate::rules::{ColumnRule, RowRule, Rule, SquareRule};

pub type DynRule = Box<dyn Rule + Send>;

#[derive(Debug)]
pub struct Sudoku {
    pub size: usize,
    pub cells: Vec<Cell>,
    pub rules: Vec<DynRule>,
}

//Det her er ret fucked, men siden vi skal have den laveste entropy ud af vores priority queue skal den sammenligne omvendt
// siden priority_queue tager den med størst priority lol
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Entropy(usize);

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
    AlreadyManySolutions,
    UnsolveableError,
    RemovedLockedValue,
}

lazy_static! {
    static ref GLOBAL_POOL: Mutex<Option<ThreadPool>> = Mutex::new(None);
}

#[derive(Debug, Clone)]
pub struct AllSolutionsContext {
    solutions: Arc<AtomicUsize>,
    pool: ThreadPool,
}

impl AllSolutionsContext {
    pub fn get_pool() -> ThreadPool {
        if let Some(pool) = GLOBAL_POOL.lock().unwrap().take() {
            pool
        } else {
            ThreadPool::new(num_cpus::get())
        }
    }
    fn return_pool(self) {
        *GLOBAL_POOL.lock().unwrap() = Some(self.pool);
    }

    fn add_branch(&self, old: &Sudoku, cells: Vec<Cell>, new_queue: PriorityQueue<usize, Entropy>) {
        let mut new_sudoku = Sudoku {
            cells,
            ..old.clone()
        };
        let self_clone = self.clone();

        self.pool.execute(move || {
            let _res = new_sudoku.solve(Some(&self_clone), Some(new_queue));
        });
    }

    fn wait_for_solutions(self) -> usize {
        self.pool.join();
        let solutions = self.solutions.load(std::sync::atomic::Ordering::Relaxed);
        self.return_pool();
        solutions
    }

    fn new() -> Self {
        Self {
            solutions: Arc::new(0.into()),
            pool: Self::get_pool(),
        }
    }
}

impl Display for SudokuSolveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SudokuSolveError::UnsolveableError => write!(f, "No Solution. Failed to pop branch queue when entropy was 0 in cell"),
            SudokuSolveError::RemovedLockedValue => write!(f, "Something went seriously wrong. Removed the only value in a locked cell\nThis indicates either an unsolveable sudoku or a bug in the rules."),
            SudokuSolveError::AlreadyManySolutions => write!(f, "Has already found more than 1 solution when searching for all solutions. Short circuting"),
        }
    }
}

impl Sudoku {
    pub fn new(size: usize, mut rules: Vec<DynRule>) -> Self {
        if !rules.iter().any(|r| r.get_name() == "ColumnRule") {
            rules.push(Box::new(ColumnRule));
        }
        if !rules.iter().any(|r| r.get_name() == "RowRule") {
            rules.push(Box::new(RowRule));
        }

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

    pub fn solve(
        &mut self,
        ctx: Option<&AllSolutionsContext>,
        pri_queue: Option<PriorityQueue<usize, Entropy>>,
    ) -> Result<(), SudokuSolveError> {
        #[cfg(debug_assertions)]
        let mut branch_count = 0;
        #[cfg(debug_assertions)]
        let mut backtracks = 0;

        let mut pri_queue = if let Some(pri_queue) = pri_queue {
            pri_queue
        } else {
            let mut pri_queue = PriorityQueue::with_capacity(self.size * self.size);
            for (index, cell) in self.cells.iter().enumerate() {
                if !cell.locked_in {
                    pri_queue.push(index, Entropy(cell.available.len()));
                }
            }
            pri_queue
        };

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

                    if let Some(ctx) = ctx {
                        if ctx.solutions.load(std::sync::atomic::Ordering::Relaxed) >= 2 {
                            return Err(SudokuSolveError::AlreadyManySolutions);
                        }

                        ctx.add_branch(self, cloned_cells, cloned_queue);
                    } else {
                        branch_stack.push((cloned_cells, cloned_queue));
                    }

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

        if let Some(ctx) = ctx {
            ctx.solutions
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }

        Ok(())
    }

    pub fn generate_with_size(
        size: usize,
        rules: Vec<DynRule>,
        progess: Option<mpsc::Sender<usize>>,
    ) -> Result<Self, SudokuSolveError> {
        let mut sudoku = Sudoku::new(size, rules);
        sudoku.solve(None, None)?;

        // if x-rule is part of the rule set insert the X's
        if let Some(x_rule) = sudoku.rules.iter_mut().find_map(|r| r.to_x_rule()) {
            for index in 0..sudoku.cells.len() {
                if let Some(current) = sudoku.cells[index].available.get(0) {
                    if let Some(left) = sudoku.cells[index + 1].available.get(0) {
                        if current + left == sudoku.size as u16 + 1 && (index + 1) % sudoku.size != 0 {
                            // x rule should have (index , left)
                            x_rule.x_clue.push((index, index + 1));
                            println!("Left X")
                        }
                    }
                    if let Some(below) = sudoku.cells[index + sudoku.size].available.get(0) {
                        if current + below == sudoku.size as u16 + 1 && index + sudoku.size < sudoku.cells.len() {
                            // x rule should have (index , below)
                            x_rule.x_clue.push((index, index + sudoku.size));
                            println!("Below X")
                        }
                    }
                }
                    
            }
        } 
        

        const ATTEMPT_COUNT: usize = 5;
        const RETRY_LIMIT: usize = 55;

        let mut count = 0;

        let mut currents_left = ATTEMPT_COUNT;
        let timer = Instant::now();
        loop {
            if let Some(progess) = &progess {
                progess.send(count).unwrap();
            }

            if count >= RETRY_LIMIT {
                break;
            }

            let removed_index = random::<usize>() % sudoku.cells.len();
            if sudoku.cells[removed_index].available.len() == 9 {
                //println!("Skipping already hit");
                continue;
            }

            let mut solved_clone = sudoku.clone();

            solved_clone.cells[removed_index] = Cell::new_with_range(1..sudoku.size as u16 + 1);

            let ctx = AllSolutionsContext::new();
            let _ = solved_clone.solve(Some(&ctx), None);

            let solutions = ctx.wait_for_solutions();

            //println!("Found {} with {count} removed", solutions);

            if solutions > 1 {
                if currents_left == 0 {
                    break;
                } else {
                    currents_left -= 1;
                    continue;
                }
            }

            currents_left = ATTEMPT_COUNT;

            sudoku.cells[removed_index] = Cell::new_with_range(1..sudoku.size as u16 + 1);

            count += 1;
        }
        println!("Removed {count} in {:?}", timer.elapsed());

        Ok(sudoku)
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
        let mut rules: Vec<DynRule> = vec![Box::new(SquareRule)];

        //WTFFF
        let sudoku_source = match regex!(r"(\r\n|\n)(\r\n|\n)")
            .split(s)
            .collect::<Vec<&str>>()
            .as_slice()
        {
            [rules_source, sudoku] => {
                for rule_name in rules_source.split('|').map(str::trim) {
                    rules.push(
                        rule_name
                            .parse()
                            .map_err(|e| ParseSudokuError::InvalidRuleName(e))?,
                    );
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
    pub fn new_with_range(range: Range<u16>) -> Self {
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
    sudoku.solve(None, None).unwrap();
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
            sudoku.solve(None, None).unwrap();
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
    let mut sudoku = Sudoku::new(9, vec![Box::new(SquareRule)]);
    sudoku.solve(None, None).unwrap();
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

    sudoku.solve(None, None).unwrap();

    assert_eq!(pre, sudoku.to_string());

    println!("PostPost:\n{sudoku}");
}

#[test]
fn solve_16x_test() {
    let file_str = std::fs::read_to_string("./sudoku16x16").unwrap();
    let mut sudoku: Sudoku = file_str.parse().unwrap();

    sudoku.solve(None, None).unwrap();

    println!("{sudoku}");
}

#[test]
fn solve_knights_move_sudoku() {
    let file_str = std::fs::read_to_string("./sudokuKnightsMove").unwrap();
    let mut sudoku: Sudoku = file_str.parse().unwrap();

    println!("{sudoku}");

    sudoku.solve(None, None).unwrap();

    println!("{sudoku}");
    assert_eq!(
        sudoku.to_string().trim(),
        std::fs::read_to_string("./sudokuKnightsMoveSolution")
            .unwrap()
            .replace("\r\n", "\n")
            .trim()
    );
}

#[test]
fn find_all_solutions() {
    let file_str = std::fs::read_to_string("./sudokuManySolutions").unwrap();
    let mut sudoku: Sudoku = file_str.parse().unwrap();

    let ctx = AllSolutionsContext::new();

    let _ = sudoku.solve(Some(&ctx), None);
    let solutions = ctx.wait_for_solutions();

    println!("Found {:?} solutions", solutions);
}

#[test]
fn generate_sudoku() {
    let timer = std::time::Instant::now();
    let sudoku = Sudoku::generate_with_size(9, vec![Box::new(SquareRule)], None).unwrap();

    println!("{sudoku} at {:?}", timer.elapsed());
}

#[test]
fn generate_sudoku_x() {
    use crate::rules::{KnightRule, SquareRule, XRule};
    let timer = std::time::Instant::now();
    let sudoku = Sudoku::generate_with_size(
        4,
        vec![
            Box::new(SquareRule),
            Box::new(KnightRule),
            Box::new(XRule {
                x_clue: vec![(0, 1), (4, 5), (4, 8), (8, 9)],
            }),
        ],
        None,
    )
    .unwrap();

    println!("{sudoku} at {:?}", timer.elapsed());
}

#[test]
fn knights_xsudoku() {
    let file_str = std::fs::read_to_string("./sudokuKnightsX").unwrap();
    let mut sudoku: Sudoku = file_str.parse().unwrap();

    sudoku.solve(None, None).unwrap();

    println!("{sudoku}");
}
