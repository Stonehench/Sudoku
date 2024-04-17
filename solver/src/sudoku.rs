use std::{
    cmp::Ordering,
    fmt::{Display, Write},
    num::ParseIntError,
    ops::{Deref, Range},
    str::FromStr,
    sync::{atomic::AtomicUsize, mpsc, Arc, Mutex},
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

use crate::rules::{column_rule::ColumnRule, row_rule::RowRule, DynRule};

pub enum Difficulty {
    Easy,
    Medium,
    Hard,
    Expert,
}

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

impl Difficulty {
    pub fn get_removes(&self, size: usize) -> usize {
        match self {
            Difficulty::Easy => size * size / 2,
            Difficulty::Medium => (size * size * 2) / 3,
            Difficulty::Hard => (size * size * 3) / 4,
            Difficulty::Expert => size * size,
        }
    }
}

#[derive(Debug)]
pub struct Sudoku {
    pub size: usize,
    pub cells: Vec<Cell>,
    pub rules: SmallVec<[DynRule; 8]>,
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
            SudokuSolveError::RemovedLockedValue => write!(f, "Something went seriously wrong. Removed the only value in a locked cell\nThis indicates a bug in the rules."),
            SudokuSolveError::AlreadyManySolutions => write!(f, "Has already found more than 1 solution when searching for all solutions. Short circuting"),
        }
    }
}

lazy_static! {
    static ref ARENA_POOL: Mutex<Vec<Bump>> = Mutex::new(vec![]);
}

impl Sudoku {
    pub fn new(size: usize, mut rules: Vec<DynRule>) -> Self {
        if !rules.iter().any(|r| r.get_name() == "ColumnRule") {
            rules.push(ColumnRule::new());
        }
        if !rules.iter().any(|r| r.get_name() == "RowRule") {
            rules.push(RowRule::new());
        }

        rules.sort_by_key(|a| a.priority());

        Self {
            size,
            cells: (0..size * size)
                .map(|_| Cell::new_with_range(1..(size as u16 + 1)))
                .collect(),
            rules: rules.into(),
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

    fn get_arena() -> Bump {
        let mut lock = ARENA_POOL.lock().unwrap();

        if let Some(bump) = lock.pop() {
            return bump;
        }
        drop(lock);

        Bump::new()
    }

    pub fn solve(
        &mut self,
        ctx: Option<&AllSolutionsContext>,
        pri_queue: Option<PriorityQueue<usize, Entropy>>,
    ) -> Result<(), SudokuSolveError> {
        let has_square = self.rules.iter().any(|r| r.get_name() == "SquareRule");

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

        'main: while let Some((index, entropy)) = pri_queue.pop() {
            assert_eq!(entropy.0, self.cells[index].available.len(), "which happend at {index}");
            match entropy.0 {
                0 => {
                    //Der er ingen løsning på den nuværende branch. Derfor popper vi en branch og løser den i stedet
                    let Some((cells, new_pri_queue)) = branch_stack.pop() else {
                        return Err(SudokuSolveError::UnsolveableError);
                    };

                    self.cells = cells;
                    pri_queue = new_pri_queue;
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

                    // TODO: Should only work on Hard and Expert in the future
                    //Locked candidates
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
                                self.cells[*remove_index].remove(n)?;
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
                                self.cells[*index].remove(*value)?;
                                pri_queue.change_priority(
                                    index,
                                    Entropy(self.cells[*index].available.len()),
                                );
                            }

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
                        //#[cfg(not(debug_assertions))]
                        if ctx.solutions.load(std::sync::atomic::Ordering::Relaxed) >= 2 {
                            return Err(SudokuSolveError::AlreadyManySolutions);
                        }

                        ctx.add_branch(self, cloned_cells, cloned_queue);
                    } else {
                        branch_stack.push((cloned_cells, cloned_queue));
                    }

                    self.update_cell(n, index, &mut pri_queue, &mut ret_buffer)?;
                }
            }

            if pri_queue.is_empty() {
                //Check om alle regler bliver overholdt
                if !self.rules.iter().all(|r| r.finished_legal(&self)) {
                    //Der er ingen løsning på den nuværende branch. Derfor popper vi en branch og løser den i stedet
                    let Some((cells, new_pri_queue)) = branch_stack.pop() else {
                        return Err(SudokuSolveError::UnsolveableError);
                    };

                    self.cells = cells;
                    pri_queue = new_pri_queue;
                }
            }
        }

        let mut lock = ARENA_POOL.lock().unwrap();
        lock.push(arena);

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
        difficulty: Difficulty,
    ) -> Result<Self, SudokuSolveError> {
        let mut sudoku = Sudoku::new(size, rules);

        sudoku.solve(None, None)?;

        for cell in sudoku.cells.iter_mut() {
            cell.locked_in = false;
        }

        // if x-rule is part of the rule set insert the X's
        if let Some(x_rule) = sudoku.rules.iter_mut().find_map(|r| r.to_x_rule()) {
            for index in 0..sudoku.cells.len() {
                if let Some(current) = sudoku.cells[index].available.get(0) {
                    if index + 1 >= sudoku.cells.len() {
                        continue;
                    }
                    if let Some(left) = sudoku.cells[index + 1].available.get(0) {
                        if current + left == sudoku.size as u16 + 1
                            && (index + 1) % sudoku.size != 0
                        {
                            // x rule should have (index , left)
                            x_rule.x_clue.push((index, index + 1));
                        }
                    }
                    if index + sudoku.size >= sudoku.cells.len() {
                        continue;
                    }
                    if let Some(below) = sudoku.cells[index + sudoku.size].available.get(0) {
                        if current + below == sudoku.size as u16 + 1
                            && index + sudoku.size < sudoku.cells.len()
                        {
                            // x rule should have (index , below)
                            x_rule.x_clue.push((index, index + sudoku.size));
                        }
                    }
                }
            }
            let count = x_rule.x_clue.len();
            if count > sudoku.size * 2 {
                for i in 0..count - (sudoku.size * 3)/2 {
                    x_rule
                        .x_clue
                        .remove(random::<usize>() % (count - i));
                }
        }}

        // if parity-rule is part of the rule-set insert some Paritys
        if let Some(parity_rule) = sudoku.rules.iter_mut().find_map(|r| r.to_parity_rule()) {
            for index in 0..sudoku.cells.len() {
                if let Some(current) = sudoku.cells[index].available.get(0) {
                    if index + 1 >= sudoku.cells.len() {
                        continue;
                    }
                    if let Some(right) = sudoku.cells[index + 1].available.get(0) {
                        if ((current & 1) == 0 && (right & 1) != 0)
                            || ((current & 1) != 0 && (right & 1) == 0)
                        {
                            // parity rule should have (current , right)
                            parity_rule.parity_clue.push((index, index + 1));
                        }
                    }
                    if index + sudoku.size >= sudoku.cells.len() {
                        continue;
                    }
                    if let Some(below) = sudoku.cells[index + sudoku.size].available.get(0) {
                        if (current & 1 == 0 && below & 1 != 0)
                            || (current & 1 != 0 && below & 1 == 0)
                        {
                            // parity rule should have (index , below)
                            parity_rule.parity_clue.push((index, index + sudoku.size));
                        }
                    }
                }
            }
            let count = parity_rule.parity_clue.len();
            if count > sudoku.size * 2 {
                for i in 0..count - sudoku.size * 2 {
                    parity_rule
                        .parity_clue
                        .remove(random::<usize>() % (count - i));
                }
            }
        }

        // if consecutive-rule is part of the rule set insert the X's
        if let Some(consecutive_rule) = sudoku
            .rules
            .iter_mut()
            .find_map(|r| r.to_consecutive_rule())
        {
            for index in 0..sudoku.cells.len() {
                if let Some(current) = sudoku.cells[index].available.get(0) {
                    if index + 1 >= sudoku.cells.len() {
                        continue;
                    }
                    if let Some(left) = sudoku.cells[index + 1].available.get(0) {
                        if current + 1 == *left
                            || *current == left + 1 && (index + 1) % sudoku.size != 0
                        {
                            consecutive_rule.consecutive_clue.push((index, index + 1));
                        }
                    }
                    if index + sudoku.size >= sudoku.cells.len() {
                        continue;
                    }
                    if let Some(below) = sudoku.cells[index + sudoku.size].available.get(0) {
                        if current + 1 == *below
                            || *current == below + 1 && index + sudoku.size < sudoku.cells.len()
                        {
                            // x rule should have (index , below)
                            consecutive_rule
                                .consecutive_clue
                                .push((index, index + sudoku.size));
                        }
                    }
                }
            }
            // remove some of the generated consecutive pairs
            let count = consecutive_rule.consecutive_clue.len();
            if count > sudoku.size * 2 {
                for i in 0..count - sudoku.size * 2 {
                    consecutive_rule
                        .consecutive_clue
                        .remove(random::<usize>() % (count - i));
                }
            }
        }

        // ThermometerRule
        if let Some(thermometer_rule) = sudoku.rules.iter_mut().find_map(|r| r.to_thermometer_rule()) {
            let tries = sudoku.size * 3;
            let mut seen = vec![];

            'themometers: for i in 0..tries {
                let mut random_index = random::<usize>() % (sudoku.size * sudoku.size);
                while seen.contains(&random_index) && seen.len() < (sudoku.size * sudoku.size) {
                    random_index = random::<usize>() % (sudoku.size * sudoku.size);
                }

                let mut current_themometer: Vec<usize> = vec![];
                let mut searching = true;
                let mut surrounding: Vec<usize> = vec![];
                let mut current_index: usize = random_index;
                let mut current_value = sudoku.cells[random_index].available[0];
                current_themometer.push(current_index);

                if current_value == sudoku.size as u16 {
                    // The value at the bottom of a themometer can not be the highest value
                    continue 'themometers;
                }

                'searching: while searching {
                    surrounding.clear();

                    if current_index >= sudoku.size {
                        //above
                        surrounding.push(current_index - sudoku.size);
                    }
                    if !(current_index % sudoku.size == 0) {
                        //left
                        surrounding.push(current_index - 1);
                    }
                    if current_index % sudoku.size != (sudoku.size - 1) {
                        //right
                        surrounding.push(current_index + 1);
                    }
                    if current_index < sudoku.size * sudoku.size - sudoku.size {
                        //below
                        surrounding.push(current_index + sudoku.size);
                    }
                    if current_index >= sudoku.size && current_index % sudoku.size != (sudoku.size - 1) {
                        //above right
                        surrounding.push(current_index - sudoku.size + 1);
                    }
                    if current_index < sudoku.size * sudoku.size - sudoku.size && !(current_index % sudoku.size == 0) {
                        //below left
                        surrounding.push(current_index + sudoku.size - 1);
                    }
                    if current_index >= sudoku.size && !(current_index % sudoku.size == 0) {
                        //above left
                        surrounding.push(current_index - sudoku.size - 1);
                    }
                    if current_index < sudoku.size * sudoku.size - sudoku.size
                        && current_index % sudoku.size != (sudoku.size - 1)
                    {
                        //below right
                        surrounding.push(current_index + sudoku.size + 1);
                    }

                    surrounding.retain(|e| sudoku.cells[*e].available[0] > current_value);
                    surrounding.sort_by(|a , b| sudoku.cells[*a].available[0].cmp(&sudoku.cells[*b].available[0]));
                    
                    if !surrounding.is_empty() && !seen.contains(&surrounding[0])
                        && sudoku.cells[surrounding[0]].available[0] > current_value
                    {
                        
                        seen.push(surrounding[0]);
                        current_themometer.push(surrounding[0]);
                        current_index = surrounding[0];
                        current_value = sudoku.cells[surrounding[0]].available[0];

                        continue 'searching; 
                    } 

                    searching = false;

                }
                if current_themometer.len() > 1 {
                    seen.push(random_index);
                    thermometer_rule.themometer_clue.push(current_themometer);
                }
            }
        }

        // if zipper-rule is part of the rule-set insert some Zippers
        // do some depth first kinda thing
        if let Some(zipper_rule) = sudoku.rules.iter_mut().find_map(|r| r.to_zipper_rule()) {
            //println!("Creating Zippers");
            let tries = sudoku.size * 3;
            let mut seen = vec![];

            'zippers: for i in 0..tries {
                let mut random_index = random::<usize>() % (sudoku.size * sudoku.size);
                while seen.contains(&random_index) && seen.len() < (sudoku.size * sudoku.size) {
                    //println!("Atemting to create zipper at an occupiued index while at zipper {i}");
                    random_index = random::<usize>() % (sudoku.size * sudoku.size);
                }

                // get the value at the random selected cell
                let center_cell_value = &sudoku.cells[random_index].available[0];
                if *center_cell_value == 1 {
                    // the value at the center of a zipper can never be 1
                    continue 'zippers;
                }
                let mut zipper_arms: Vec<(usize, usize)> = vec![];

                let mut searching = true;
                let mut left_index: usize = random_index;
                let mut right_index: usize = random_index;
                let mut left_surrounding: Vec<usize> = vec![];
                let mut right_surrounding: Vec<usize> = vec![];

                'searching: while searching {
                    left_surrounding.clear();
                    right_surrounding.clear();

                    // get the surronding digits to the left arm
                    for k in vec![left_index, right_index] {
                        if k >= sudoku.size {
                            //above
                            if k == left_index {
                                left_surrounding.push(k - sudoku.size);
                            }
                            if k == right_index {
                                right_surrounding.push(k - sudoku.size);
                            }
                        }
                        if !(k % sudoku.size == 0) {
                            //left
                            if k == left_index {
                                left_surrounding.push(k - 1);
                            }
                            if k == right_index {
                                right_surrounding.push(k - 1);
                            }
                        }
                        if k % sudoku.size != (sudoku.size - 1) {
                            //right
                            if k == left_index {
                                left_surrounding.push(k + 1);
                            }
                            if k == right_index {
                                right_surrounding.push(k + 1);
                            }
                        }
                        if k < sudoku.size * sudoku.size - sudoku.size {
                            //below
                            if k == left_index {
                                left_surrounding.push(k + sudoku.size);
                            }
                            if k == right_index {
                                right_surrounding.push(k + sudoku.size);
                            }
                        }
                        if k >= sudoku.size && k % sudoku.size != (sudoku.size - 1) {
                            //above right
                            if k == left_index {
                                left_surrounding.push(k - sudoku.size + 1);
                            }
                            if k == right_index {
                                right_surrounding.push(k - sudoku.size + 1);
                            }
                        }
                        if k < sudoku.size * sudoku.size - sudoku.size && !(k % sudoku.size == 0) {
                            //below left
                            if k == left_index {
                                left_surrounding.push(k + sudoku.size - 1);
                            }
                            if k == right_index {
                                right_surrounding.push(k + sudoku.size - 1);
                            }
                        }
                        if k >= sudoku.size && !(k % sudoku.size == 0) {
                            //above left
                            if k == left_index {
                                left_surrounding.push(k - sudoku.size - 1);
                            }
                            if k == right_index {
                                right_surrounding.push(k - sudoku.size - 1);
                            }
                        }
                        if k < sudoku.size * sudoku.size - sudoku.size
                            && k % sudoku.size != (sudoku.size - 1)
                        {
                            //below right
                            if k == left_index {
                                left_surrounding.push(k + sudoku.size + 1);
                            }
                            if k == right_index {
                                right_surrounding.push(k + sudoku.size + 1);
                            }
                        }
                        //println!("{left_surrounding:?} {right_surrounding:?} {k}");
                        if left_index == right_index {
                            break;
                        }
                    }

                    // for all indecies surronding left
                    for i_in_l in &left_surrounding {
                        // and all indecies surronding right
                        for i_in_r in &right_surrounding {
                            // if the inxed is not in any other zipper (including this one)
                            // and the values of the incecies add to the center value
                            // these should be added as a pair

                            if i_in_l != i_in_r
                                && !seen.contains(i_in_l)
                                && !seen.contains(i_in_r)
                                && sudoku.cells[*i_in_l].available[0]
                                    + sudoku.cells[*i_in_r].available[0]
                                    == *center_cell_value
                            {
                                seen.push(*i_in_l);
                                seen.push(*i_in_r);
                                zipper_arms.push((*i_in_l, *i_in_r));
                                right_index = *i_in_r;
                                left_index = *i_in_l;

                                continue 'searching;
                            }
                        }
                    }
                    searching = false;
                }
                if !zipper_arms.is_empty() {
                    seen.push(random_index);
                    zipper_rule.zipper_clue.push((random_index, zipper_arms));
                }
            }
        }

        #[cfg(debug_assertions)]
        println!("Solved rules: {:#?}", sudoku.rules);

        let remove_limit = difficulty.get_removes(size);

        const ATTEMPT_COUNT: usize = 25;

        //#[cfg(debug_assertions)]
        let timer = Instant::now();

        let mut count = 0;

        let mut currents_left = ATTEMPT_COUNT;

        let mut available_to_remove: Vec<_> = (0..sudoku.cells.len()).collect();

        loop {
            if timer.elapsed().as_secs() > 20 {
                println!("OUT OF GEN TIME!!");
                break;
            }

            if count >= remove_limit {
                break;
            }
            if let Some(progess) = &progess {
                progess.send(count).unwrap();
            }

            if available_to_remove.len() == 0 {
                println!("nothing more to try");
                break;
            }
            let removed_index =
                available_to_remove.remove(random::<usize>() % available_to_remove.len());

            let mut solved_clone = sudoku.clone();

            solved_clone.cells[removed_index] = Cell::new_with_range(1..sudoku.size as u16 + 1);

            let ctx = AllSolutionsContext::new();
            let _ = solved_clone.solve(Some(&ctx), None);

            let solutions = ctx.wait_for_solutions();

            if solutions != 1 {
                if currents_left == 0 {
                    break;
                } else {
                    currents_left -= 1;
                    #[cfg(debug_assertions)]
                    println!("Attempts left: {currents_left}");
                    continue;
                }
            }

            currents_left = ATTEMPT_COUNT;

            sudoku.cells[removed_index] = Cell::new_with_range(1..sudoku.size as u16 + 1);
            count += 1;
        }

        #[cfg(debug_assertions)]
        println!("Removed {count} in {:?}", timer.elapsed());

        for cell in &mut sudoku.cells {
            cell.locked_in = false;
        }

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
        let mut rules: Vec<DynRule> = vec![];

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
    pub available: smallvec::SmallVec<[u16; 16]>,
    pub locked_in: bool,
}

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

impl Clone for Sudoku {
    fn clone(&self) -> Self {
        Self {
            size: self.size.clone(),
            cells: self.cells.clone(),
            rules: self.rules.iter().map(|r| r.boxed_clone()).collect(),
        }
    }
}



//########################### TEST ###############################
#[test]
fn generate_sudoku_zipper() {
    let sudoku = Sudoku::generate_with_size(
        4,
        vec![
            super::rules::square_rule::SquareRule::new(),
            crate::rules::zipper_rule::ZipperRule::new(vec![]),
        ],
        None,
        Difficulty::Expert,
    )
    .unwrap();

    println!("{sudoku:?}");
    println!("{sudoku}");
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
fn solve_4x4_xdiagonal_sudoku() {
    // TODO This will calculate two different solutions at random!!!!!

    let file_str = std::fs::read_to_string("./sudokuXDiagonal4x4").unwrap();
    let mut sudoku: Sudoku = file_str.parse().unwrap();

    println!("{sudoku}");

    let cxt = AllSolutionsContext::new();
    sudoku.solve(Some(&cxt), None).unwrap();
    println!("{sudoku}, {:?}", cxt.wait_for_solutions());
}
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
    let mut sudoku = Sudoku::generate_with_size(
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
    println!("{sudoku}");

    let cxt = AllSolutionsContext::new();
    sudoku.solve(Some(&cxt), None).unwrap();

    println!("{sudoku} = {}", cxt.wait_for_solutions());
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
    let mut sudoku = Sudoku::new(9, vec![super::rules::square_rule::SquareRule::new()]);
    sudoku.solve(None, None).unwrap();
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
fn solve_zipper_test() {
    let file_str = std::fs::read_to_string("./sudokuZipper").unwrap();
    let mut sudoku: Sudoku = file_str.parse().unwrap();

    sudoku.solve(None, None).unwrap();

    println!("{sudoku}");
}

#[test]
fn solve_zipper9x9_test() {
    let file_str = std::fs::read_to_string("./sudokuZipper9x9").unwrap();
    let mut sudoku: Sudoku = file_str.parse().unwrap();

    sudoku.solve(None, None).unwrap();

    println!("{sudoku}");
    assert_eq!(
        sudoku.to_string().trim(),
        std::fs::read_to_string("./sudokuZipper9x9Solution")
            .unwrap()
            .replace("\r\n", "\n")
            .trim()
    );
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
    let sudoku = Sudoku::generate_with_size(
        9,
        vec![super::rules::square_rule::SquareRule::new()],
        None,
        Difficulty::Expert,
    )
    .unwrap();

    println!("{sudoku} at {:?}", timer.elapsed());
}

#[test]
fn generate_thermometer_sudoku() {
    let timer = std::time::Instant::now();
    let sudoku = Sudoku::generate_with_size(
        9,
        vec![super::rules::square_rule::SquareRule::new(),crate::rules::thermometer_rule::ThermometerRule::new(vec![]),],
        None,
        Difficulty::Expert,
    )
    .unwrap();

    println!("{sudoku} at {:?}", timer.elapsed());
}

#[test]
fn generate_sudoku_x() {
    let timer = std::time::Instant::now();
    let sudoku = Sudoku::generate_with_size(
        4,
        vec![
            super::rules::square_rule::SquareRule::new(),
            crate::rules::knight_rule::KnightRule::new(),
            crate::rules::x_rule::XRule::new(vec![(0, 1), (4, 5), (4, 8), (8, 9)]),
        ],
        None,
        Difficulty::Expert,
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
