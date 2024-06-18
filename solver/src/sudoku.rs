use std::{
    cmp::Ordering,
    collections::HashSet,
    fmt::{Display, Write},
    hash::{DefaultHasher, Hash, Hasher},
    num::ParseIntError,
    ops::{Deref, Range},
    ptr::copy_nonoverlapping,
    str::FromStr,
    sync::{atomic::AtomicUsize, Arc, Mutex},
    time::Instant,
};

use bumpalo::Bump;
use integer_sqrt::IntegerSquareRoot;
use lazy_static::lazy_static;
use priority_queue::PriorityQueue;
use rand::random;
use regex_macro::regex;
use smallvec::{smallvec, SmallVec};
use threadpool::ThreadPool;

use crate::rules::{
    column_rule::ColumnRule, row_rule::RowRule, square_rule::SquareRule, DynRule, Rule,
};
// Author Thor s224817
#[derive(Debug, Clone, Copy)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
    Expert,
}
// Author Thor s224817
impl FromStr for Difficulty {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Easy" => Ok(Difficulty::Easy),
            "Medium" => Ok(Difficulty::Medium),
            "Hard" => Ok(Difficulty::Hard),
            "Expert" => Ok(Difficulty::Expert),
            _ => Err(()),
        }
    }
}
// Author Thor s224817
impl Difficulty {
    pub fn get_removes(&self, size: usize, clues: usize) -> usize {
        match self {
            Difficulty::Easy => (size * size + clues) / 2,
            Difficulty::Medium => (size * size * 2 + clues) / 3,
            Difficulty::Hard => (size * size * 3 + clues) / 4,
            Difficulty::Expert => size * size + clues,
        }
    }
}
// Author Thor s224817
#[derive(Debug)]
pub struct Sudoku {
    pub size: usize,
    pub cells: Vec<Cell>,
    pub rules: SmallVec<[DynRule; 8]>,
    pub has_square: bool,
}

// Author Thor s224817
// Since the priority queue is a maxqueue, we use an entropy struct and reverse the implementation of Ord(ering)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Entropy(usize);

// Author Thor s224817
impl PartialOrd for Entropy {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.0.partial_cmp(&self.0)
        //self.0.partial_cmp(&other.0)
    }
}
// Author Thor s224817
impl Ord for Entropy {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other).unwrap()
    }
}
// Author Thor s224817
#[derive(Debug, Clone, Copy)]
pub enum SudokuSolveError {
    AlreadyManySolutions,
    UnsolveableError,
    RemovedLockedValue,
}

// Author Thor s224817
//Global Theadpool pool. 
lazy_static! {
    static ref GLOBAL_POOL: Mutex<Option<ThreadPool>> = Mutex::new(None);
}

// Author Thor s224817
#[derive(Debug, Clone)]
pub struct AllSolutionsContext {
    solutions: Arc<AtomicUsize>,
    pool: ThreadPool,
    cache: Option<Arc<(HashSet<u64>, HashSet<u64>)>>,
    write_cache: Arc<Mutex<(HashSet<u64>, HashSet<u64>)>>,
}
// Author Thor s224817
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

    fn add_branch(
        &self,
        old: &Sudoku,
        cells: Vec<Cell>,
        new_queue: PriorityQueue<usize, Entropy>,
        states: HashSet<u64>,
    ) {
        let mut new_sudoku = Sudoku {
            cells,
            ..old.clone()
        };
        let self_clone = self.clone();

        self.pool.execute(move || {
            let _res = new_sudoku.solve(Some(&self_clone), Some(new_queue), Some(states));
        });
    }

    fn wait_for_solutions(self) -> (usize, HashSet<u64>, HashSet<u64>) {
        self.pool.join();
        let solutions = self.solutions.load(std::sync::atomic::Ordering::SeqCst);
        let cache_lock = self.write_cache.lock().unwrap();
        let (good, bad) = cache_lock.clone();

        drop(cache_lock);
        self.return_pool();
        (solutions, good, bad)
    }

    fn new_with_cache(cache: Arc<(HashSet<u64>, HashSet<u64>)>) -> Self {
        Self {
            solutions: Arc::new(0.into()),
            pool: Self::get_pool(),
            cache: Some(cache),
            write_cache: Arc::new(Mutex::new((HashSet::new(), HashSet::new()))),
        }
    }

    #[allow(unused)]
    fn new() -> Self {
        Self {
            solutions: Arc::new(0.into()),
            pool: Self::get_pool(),
            cache: None,
            write_cache: Arc::new(Mutex::new((HashSet::new(), HashSet::new()))),
        }
    }
}
// Author Thor s224817
impl Display for SudokuSolveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SudokuSolveError::UnsolveableError => write!(f, "No Solution. Failed to pop branch queue when entropy was 0 in cell"),
            SudokuSolveError::RemovedLockedValue => write!(f, "Something went seriously wrong. Removed the only value in a locked cell\nThis indicates a bug in the rules."),
            SudokuSolveError::AlreadyManySolutions => write!(f, "Has already found more than 1 solution when searching for all solutions. Short circuting"),
        }
    }
}
// Author Thor s224817
// Global bump allocator pool
lazy_static! {
    static ref ARENA_POOL: Mutex<Vec<Bump>> = Mutex::new(vec![]);
}

impl Sudoku {
    // Author Thor s224817 and Katinka s224805
    //Create a new Sudoku with a size and list of rules
    pub fn new(size: usize, mut rules: Vec<DynRule>) -> Self {
        if !rules.iter().any(|r| r.get_name() == "ColumnRule") {
            rules.push(ColumnRule::new());
        }
        if !rules.iter().any(|r| r.get_name() == "RowRule") {
            rules.push(RowRule::new());
        }

        rules.sort_by_key(|a| a.priority());
        let has_square: bool = rules
            .iter()
            .any(|rule| rule.get_name() == SquareRule.get_name());

        Self {
            size,
            cells: (0..size * size)
                .map(|_| Cell::new_with_range(1..(size as u16 + 1)))
                .collect(),
            rules: rules.into(),
            has_square,
        }
    }
    // Author Thor s224817
    pub fn reset_locked(&mut self) {
        for cell in &mut self.cells {
            cell.locked_in = false;
        }
    }
    // Author Thor s224817
    // Setting a cell outside solving context.
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
    // Author Thor s224817
    // Setting a cell inside solving context. Updates the entropy of all affected cells
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
    // Author Thor s224817
    // Take an arena from the pool
    fn get_arena() -> Bump {
        if let Ok(mut lock) = ARENA_POOL.try_lock() {
            if let Some(bump) = lock.pop() {
                return bump;
            }
        }
        Bump::new()
    }
    // Author Thor s224817
    // Free arena into the pool
    fn free_arena(bump: Bump) {
        let mut lock = ARENA_POOL.lock().unwrap();
        lock.push(bump);
    }

    // Author Thor s224817, Katinka s224805 and Peter
    // Solve a Sudoku. AllsolutionsContext, priqueue and new_states are all only needed in generation.
    pub fn solve(
        &mut self,
        ctx: Option<&AllSolutionsContext>,
        pri_queue: Option<PriorityQueue<usize, Entropy>>,
        new_states: Option<HashSet<u64>>,
    ) -> Result<(), SudokuSolveError> {
        let has_square = self
            .rules
            .iter()
            .any(|r| r.get_name() == SquareRule.get_name());

        
        //If not pri_queue is given, create a new and fill it.
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
        let mut big_buffer = vec![];
        let mut arena = Self::get_arena();
        let mut state_buffer = Vec::with_capacity(self.cells.len());

        //If no states are given, create new hashset.
        let mut new_states = if let Some(new_states) = new_states {
            new_states
        } else {
            HashSet::new()
        };


        //This is the main solver loop. Stats by getting the cell with lowest entropy.
        'main: while let Some((index, entropy)) = pri_queue.pop() {
            // Assert entropy is always = available.len(). Otherwise crash as this is a bug.
            assert_eq!(
                entropy.0,
                self.cells[index].available.len(),
                "which happend at {index}"
            );
            match entropy.0 {
                0 => {
                    //No solution on current branch. Pop and solve other branch.
                    let Some((cells, new_pri_queue)) = branch_stack.pop() else {
                        if let Some(ctx) = ctx {
                            let mut lock = ctx.write_cache.lock().unwrap();

                            for state in new_states {
                                lock.1.insert(state);
                            }
                        }
                        Self::free_arena(arena);
                        return Err(SudokuSolveError::UnsolveableError);
                    };

                    self.cells = cells;
                    pri_queue = new_pri_queue;
                }
                1 => self.update_cell(
                    //naked singles
                    self.cells[index].available[0],
                    index,
                    &mut pri_queue,
                    &mut ret_buffer,
                )?,
                _ => {
                    // No more naked singles. Use more advances analysis functions.

                    for rule in &self.rules {
                        if let Some((n, hidden_index)) = rule.hidden_singles(self) {
                            //Put nuværende cell tilbage i priority queue
                            pri_queue.push(index, entropy);
                            pri_queue.remove(&hidden_index);
                            if let Err(_) =
                                self.update_cell(n, hidden_index, &mut pri_queue, &mut ret_buffer)
                            {
                                let Some((cells, new_pri_queue)) = branch_stack.pop() else {
                                    return Err(SudokuSolveError::UnsolveableError);
                                };

                                self.cells = cells;
                                pri_queue = new_pri_queue;
                            }

                            continue 'main;
                        }
                    }

                    for rule in self.rules.iter().filter(|r| {
                        if r.needs_square_for_locked() {
                            has_square
                        } else {
                            true
                        }
                    }) {
                        if let Some((n, removable_indexes)) =
                            rule.locked_candidate(self, &mut ret_buffer, &mut arena)
                        {
                            #[cfg(debug_assertions)] // I debug mode tjekker vi om locked_candidates ikke er tomme
                            assert!(!removable_indexes.is_empty());

                            //Put nuværende cell tilbage i priority queue
                            pri_queue.push(index, entropy);

                            for remove_index in removable_indexes {
                                if let Err(_) = self.cells[*remove_index].remove(n) {
                                    let Some((cells, new_pri_queue)) = branch_stack.pop() else {
                                        return Err(SudokuSolveError::UnsolveableError);
                                    };

                                    self.cells = cells;
                                    pri_queue = new_pri_queue;
                                }
                                pri_queue.change_priority(
                                    remove_index,
                                    Entropy(self.cells[*remove_index].available.len()),
                                );
                            }

                            continue 'main;
                        }
                    }

                    for rule in self.rules.iter().filter(|r| {
                        if r.needs_square_for_locked() {
                            return has_square;
                        }
                        true
                    }) {
                        let multi_remove_indecies = rule.multi_remove(self, &mut big_buffer);
                        if !multi_remove_indecies.is_empty() {
                            //Put nuværende cell tilbage i priority queue
                            pri_queue.push(index, entropy);

                            for (value, index) in multi_remove_indecies {
                                if let Err(_) = self.cells[*index].remove(*value) {
                                    let Some((cells, new_pri_queue)) = branch_stack.pop() else {
                                        return Err(SudokuSolveError::UnsolveableError);
                                    };

                                    self.cells = cells;
                                    pri_queue = new_pri_queue;
                                }
                                pri_queue.change_priority(
                                    index,
                                    Entropy(self.cells[*index].available.len()),
                                );
                            }

                            continue 'main;
                        }
                    }

                    //Check if current state is in cache, and exit if it is
                    if let Some(ctx) = ctx {
                        if let Some((good, bad)) = ctx.cache.as_deref() {
                            let hash = self.state_hash(&mut state_buffer);

                            if good.contains(&hash) {
                                ctx.solutions
                                    .fetch_add(1, std::sync::atomic::Ordering::SeqCst);

                                let mut lock = ctx.write_cache.lock().unwrap();
                                lock.0 = new_states;

                                //println!("Hit good solve cache");
                                Self::free_arena(arena);
                                return Ok(());
                            } else if bad.contains(&hash) {
                                let mut lock = ctx.write_cache.lock().unwrap();

                                for state in new_states {
                                    lock.1.insert(state);
                                }

                                //println!("Hit bad solve cache");
                                Self::free_arena(arena);
                                return Err(SudokuSolveError::UnsolveableError);
                            } else {
                                new_states.insert(hash);
                            }
                        }
                    }

                    // Analysis failed. Branch instead by chossing random number from popped cell.

                    let choice = random::<usize>() % entropy.0;

                    let n = self.cells[index].available[choice];

                    let mut cloned_cells = self.cell_fastclone();

                    cloned_cells[index].available.remove(choice);

                    let mut cloned_queue = pri_queue.clone();
                    cloned_queue.push(index, Entropy(entropy.0 - 1));

                    if let Some(ctx) = ctx {
                        if ctx.solutions.load(std::sync::atomic::Ordering::Relaxed) >= 2 {
                            return Err(SudokuSolveError::AlreadyManySolutions);
                        }

                        ctx.add_branch(self, cloned_cells, cloned_queue, new_states.clone());
                    } else {
                        branch_stack.push((cloned_cells, cloned_queue));
                    }

                    if let Err(_) = self.update_cell(n, index, &mut pri_queue, &mut ret_buffer) {
                        let Some((cells, new_pri_queue)) = branch_stack.pop() else {
                            if let Some(ctx) = ctx {
                                let mut lock = ctx.write_cache.lock().unwrap();

                                for state in new_states {
                                    lock.1.insert(state);
                                }
                            }

                            Self::free_arena(arena);
                            return Err(SudokuSolveError::UnsolveableError);
                        };

                        self.cells = cells;
                        pri_queue = new_pri_queue;
                    }
                }
            }

            if pri_queue.is_empty() {
                //Check if all rules are kept
                if !self.rules.iter().all(|r| r.finished_legal(&self)) {
                    //No solution on current branch. Pop and solve.
                    let Some((cells, new_pri_queue)) = branch_stack.pop() else {
                        if let Some(ctx) = ctx {
                            let mut lock = ctx.write_cache.lock().unwrap();

                            for state in new_states {
                                lock.1.insert(state);
                            }
                        }

                        Self::free_arena(arena);
                        return Err(SudokuSolveError::UnsolveableError);
                    };

                    self.cells = cells;
                    pri_queue = new_pri_queue;
                }
            }
        }

        Self::free_arena(arena);

        // If in multisolve context, write relevant data.
        if let Some(ctx) = ctx {
            let mut lock = ctx.write_cache.lock().unwrap();
            lock.0 = new_states;

            ctx.solutions
                .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        }

        Ok(())
    }
    // Author Thor s224817 and Peter s224759
    pub fn generate_with_size(
        size: usize,
        rules: Vec<DynRule>,
        progess: Option<Box<dyn Fn(usize)>>,
        difficulty: Difficulty,
    ) -> Result<(Self, Self), SudokuSolveError> {
        let mut sudoku = Sudoku::new(size, rules);

        //Initial solve.
        sudoku.solve(None, None, None)?;
        sudoku.reset_locked();
        let solved = sudoku.clone();
        let mut extra_clues = 0;

        for rule in &mut sudoku.rules {
            rule.create_clue(&sudoku.cells, size);
            extra_clues += rule.no_of_clues();
        }

        let remove_limit = difficulty.get_removes(size, extra_clues);

        const ATTEMPT_COUNT: usize = 25;

        let timer = Instant::now();

        let mut count = 0;
        let mut currents_left = ATTEMPT_COUNT;
        let mut available_to_remove: Vec<_> = (0..sudoku.cells.len()).collect();

        let mut good_cache = HashSet::<u64>::new();
        let mut bad_cache = HashSet::<u64>::new();

        //Remove clue loop.
        loop {
            let shared_caches = Arc::new((good_cache.clone(), bad_cache.clone()));
            
            if timer.elapsed().as_secs() > 30 {
                println!("OUT OF GEN TIME!!");
                break;
            }

            if count >= remove_limit {
                break;
            }
            if let Some(progess) = &progess {
                progess(count);
            }

            if available_to_remove.len() == 0 {
                //println!("Removed everything");
                break;
            }
            let removed_index =
                available_to_remove.remove(random::<usize>() % available_to_remove.len());

            let mut solving_clone = sudoku.clone();

            solving_clone.cells[removed_index] = Cell::new_with_range(1..sudoku.size as u16 + 1);

            let ctx = AllSolutionsContext::new_with_cache(shared_caches);
            let _ = solving_clone.solve(Some(&ctx), None, None);

            let (solutions, new_good_cache, new_bad_cache) = ctx.wait_for_solutions();

            //let badlen = new_bad_cache.len();
            for bad_state in new_bad_cache {
                bad_cache.insert(bad_state);
            }

            if solutions != 1 {
                if currents_left == 0 {
                    break;
                } else {
                    currents_left -= 1;
                    continue;
                }
            }

            for good_state in new_good_cache {
                good_cache.insert(good_state);
            }

            currents_left = ATTEMPT_COUNT;

            sudoku.cells[removed_index] = Cell::new_with_range(1..sudoku.size as u16 + 1);
            count += 1;
        }

        #[cfg(debug_assertions)]
        println!("Removed {count} in {:?}", timer.elapsed());

        Ok((sudoku, solved))
    }
    // Author Thor s224817
    fn cell_fastclone(&self) -> Vec<Cell> {
        //If size is <= 16 then copy cells via memcpy. Otherwise do normal clone.
        if self.size <= 16 {
            let data_slice = self.cells.as_slice();
            let mut buffer = Vec::with_capacity(data_slice.len());

            let src = data_slice.as_ptr();
            let dst = buffer.as_mut_ptr();

            unsafe {
                copy_nonoverlapping(src, dst, data_slice.len());
                buffer.set_len(data_slice.len());
            }

            return buffer;
        }
        return self.cells.clone();
    }
    // Author Thor s224817
    fn state_hash(&self, state: &mut Vec<u16>) -> u64 {
        state.clear();
        for cell in &self.cells {
            if cell.available.len() == 1 {
                state.push(cell.available[0]);
            } else {
                state.push(0);
            }
        }
        let mut hasher = DefaultHasher::new();
        state.hash(&mut hasher);
        hasher.finish()
    }
}

// Author Thor s224817
#[derive(Debug, Clone)]
pub enum ParseSudokuError {
    ParseIntError(ParseIntError),
    InvalidSizeError(usize),
    UnsolveableError,
    InvalidRuleName(String),
}
// Author Thor s224817
impl Display for ParseSudokuError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
// Author Thor s224817 and Katinka s224805
// PSF parser.
impl FromStr for Sudoku {
    type Err = ParseSudokuError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rules: Vec<DynRule> = vec![];

        //Windows sucks. Split on all double newlines. Use regex because windows is edgy.
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
// Author Katinka s224805
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
// Author Thor s224817 
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Cell {
    pub available: smallvec::SmallVec<[u16; 16]>,
    pub locked_in: bool,
}
// Author Thor s224817 
impl Cell {
    pub fn single(n: u16) -> Self {
        Self {
            available: smallvec![n],
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
            //panic!("Tried to remove locked value. This is a BUG!!");
            return Err(SudokuSolveError::RemovedLockedValue);
        }
        Ok(())
    }
    #[allow(unused)]
    pub fn is_single_eq(&self, n: u16) -> bool {
        self.available.deref() == &[n]
    }
}
// Author Thor s224817 
impl Clone for Sudoku {
    fn clone(&self) -> Self {
        Self {
            size: self.size,
            cells: self.cell_fastclone(),
            rules: self.rules.iter().map(|r| r.boxed_clone()).collect(),
            has_square: self.has_square,
        }
    }
}

//########################### TEST ###############################
// Author Thor s224817 
#[test]
fn read_file_test() {
    let file_str = std::fs::read_to_string("./sudokuBenchmark").unwrap();
    let sudoku: Sudoku = file_str.parse().unwrap();

    println!("{sudoku}");
}
// Author Katinka s224805
#[test]
fn solve_big_sudoku() {
    // tests a 16x16 sudoku solve

    let file_str = std::fs::read_to_string("./sudoku16").unwrap();
    let mut sudoku: Sudoku = file_str.parse().unwrap();

    println!("{sudoku}");
    sudoku.solve(None, None, None).unwrap();
    println!("{sudoku}");
}

// Author Katinka s224805
#[test]
fn solve_4x4_xdiagonal_sudoku() {
    // TODO This will calculate two different solutions at random!!!!!

    let file_str = std::fs::read_to_string("./sudokuXDiagonal4x4").unwrap();
    let mut sudoku: Sudoku = file_str.parse().unwrap();

    println!("{sudoku}");

    let cxt = AllSolutionsContext::new();
    sudoku.solve(Some(&cxt), None, None).unwrap();
    println!("{sudoku}, {:?}", cxt.wait_for_solutions());
}
// Author Katinka s224805
#[test]
fn generate_4x4_xdiagonal() {
    let x_clue = vec![/*
        (0, 4),
        (1, 5),
        (2, 6),
        (3, 7),
        (8, 12),
        (9, 13),
        (10, 14),
        (11, 15), */
    ];
    let (mut sudoku, _) = Sudoku::generate_with_size(
        4,
        vec![
            crate::rules::square_rule::SquareRule::new(),
            crate::rules::diagonal_rule::DiagonalRule::new(),
            crate::rules::x_rule::XRule::new(x_clue),
        ],
        None,
        Difficulty::Expert,
    )
    .unwrap();

    let cxt = AllSolutionsContext::new();
    sudoku.solve(Some(&cxt), None, None).unwrap();
}
// Author Thor s224817
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
            sudoku.solve(None, None, None).unwrap();
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
// Author Katinka s224805
#[test]
fn random_gen() {
    let mut sudoku = Sudoku::new(9, vec![super::rules::square_rule::SquareRule::new()]);
    sudoku.solve(None, None, None).unwrap();
    let pre = sudoku.to_string();
    println!("Pre:\n{}", pre);

    let difficulty = 10;

    for _ in 0..difficulty {
        let index = random::<usize>() % sudoku.cells.len();
        sudoku.cells[index] = Cell::new_with_range(1..(sudoku.size as u16 + 1))
    }
    for cell in sudoku.cells.iter_mut() {
        cell.locked_in = false;
    }

    let post = sudoku.to_string();
    println!("Post:\n{}", post);

    sudoku.solve(None, None, None).unwrap();

    assert_eq!(pre, sudoku.to_string());

    println!("PostPost:\n{sudoku}");
}

// Author Katinka s224805
#[test]
fn solve_16x_test() {
    let file_str = std::fs::read_to_string("./sudoku16x16").unwrap();
    let mut sudoku: Sudoku = file_str.parse().unwrap();

    sudoku.solve(None, None, None).unwrap();

    println!("{sudoku}");
}
// Author Katinka s224805
#[test]
fn solve_zipper_test() {
    let file_str = std::fs::read_to_string("./sudokuZipper").unwrap();
    let mut sudoku: Sudoku = file_str.parse().unwrap();

    sudoku.solve(None, None, None).unwrap();

    println!("{sudoku}");
}
// Author Katinka s224805
#[test]
fn solve_zipper9x9_test() {
    let file_str = std::fs::read_to_string("./sudokuZipper9x9").unwrap();
    let mut sudoku: Sudoku = file_str.parse().unwrap();

    sudoku.solve(None, None, None).unwrap();

    println!("{sudoku}");
    assert_eq!(
        sudoku.to_string().trim(),
        std::fs::read_to_string("./sudokuZipper9x9Solution")
            .unwrap()
            .replace("\r\n", "\n")
            .trim()
    );
}
// Author Katinka s224805
#[test]
fn solve_knights_move_sudoku() {
    let file_str = std::fs::read_to_string("./sudokuKnightsMove").unwrap();
    let mut sudoku: Sudoku = file_str.parse().unwrap();

    println!("{sudoku}");

    sudoku.solve(None, None, None).unwrap();

    println!("{sudoku}");
    assert_eq!(
        sudoku.to_string().trim(),
        std::fs::read_to_string("./sudokuKnightsMoveSolution")
            .unwrap()
            .replace("\r\n", "\n")
            .trim()
    );
}
// Author Thor s224817
#[test]
fn find_all_solutions() {
    let file_str = std::fs::read_to_string("./sudokuManySolutions").unwrap();
    let mut sudoku: Sudoku = file_str.parse().unwrap();

    let ctx = AllSolutionsContext::new();

    let _ = sudoku.solve(Some(&ctx), None, None);
    let solutions = ctx.wait_for_solutions();

    println!("Found {:?} solutions", solutions);
}
// Author Thor s224817
#[test]
fn generate_sudoku() {
    let timer = std::time::Instant::now();
    let (sudoku, _) = Sudoku::generate_with_size(
        9,
        vec![super::rules::square_rule::SquareRule::new()],
        None,
        Difficulty::Expert,
    )
    .expect("Failed to generate sudoku");

    println!("{sudoku} at {:?}", timer.elapsed());
}
// Author Katinka s224805
#[test]
fn generate_thermometer_sudoku() {
    let timer = std::time::Instant::now();
    let (sudoku, _) = Sudoku::generate_with_size(
        9,
        vec![
            super::rules::square_rule::SquareRule::new(),
            crate::rules::thermometer_rule::ThermometerRule::new(vec![]),
        ],
        None,
        Difficulty::Expert,
    )
    .expect("Failed to generate sudoku");

    println!("{sudoku} at {:?}", timer.elapsed());
}
// Author Katinka s224805
#[test]
fn generate_sudoku_x() {
    let timer = std::time::Instant::now();
    let (sudoku, _) = Sudoku::generate_with_size(
        4,
        vec![
            super::rules::square_rule::SquareRule::new(),
            crate::rules::knight_rule::KnightRule::new(),
            crate::rules::x_rule::XRule::new(vec![(0, 1), (4, 5), (4, 8), (8, 9)]),
        ],
        None,
        Difficulty::Expert,
    )
    .expect("Failed to generate sudoku");

    println!("{sudoku} at {:?}", timer.elapsed());
}
// Author Katinka s224805
#[test]
fn generate_sudoku_zipper() {
    let (sudoku, _) = Sudoku::generate_with_size(
        4,
        vec![
            super::rules::square_rule::SquareRule::new(),
            crate::rules::zipper_rule::ZipperRule::new(vec![]),
        ],
        None,
        Difficulty::Expert,
    )
    .expect("Failed to generate sudoku");
    println!("{sudoku}");
}
// Author Katinka s224805
#[test]
fn generate_sudoku_parity() {
    let timer = std::time::Instant::now();
    let (sudoku, _) = Sudoku::generate_with_size(
        4,
        vec![
            super::rules::square_rule::SquareRule::new(),
            crate::rules::knight_rule::KnightRule::new(),
            crate::rules::parity_rule::ParityRule::new(vec![(0, 1), (4, 5), (4, 8), (8, 9)]),
        ],
        None,
        Difficulty::Expert,
    )
    .expect("Failed to generate sudoku");

    println!("{sudoku} at {:?}", timer.elapsed());
}
// Author Katinka s224805
#[test]
fn generate_sudoku_consecutive() {
    let timer = std::time::Instant::now();
    let (sudoku, _) = Sudoku::generate_with_size(
        4,
        vec![
            super::rules::square_rule::SquareRule::new(),
            crate::rules::knight_rule::KnightRule::new(),
            crate::rules::consecutive_rule::ConsecutiveRule::new(vec![
                (0, 1),
                (4, 5),
                (4, 8),
                (8, 9),
            ]),
        ],
        None,
        Difficulty::Expert,
    )
    .expect("Failed to generate sudoku");

    println!("{sudoku} at {:?}", timer.elapsed());
}
// Author Katinka s224805
#[test]
fn knights_xsudoku() {
    let file_str = std::fs::read_to_string("./sudokuKnightsX").unwrap();
    let mut sudoku: Sudoku = file_str.parse().unwrap();

    sudoku.solve(None, None, None).unwrap();

    println!("{sudoku}");
}
