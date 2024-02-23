use std::{num::ParseIntError, ops::Range, str::FromStr};

use priority_queue::PriorityQueue;

use crate::rules::{ColumnRule, RowRule, Rule, SquareRule};

#[derive(Debug)]
pub struct Sudoku {
    pub size: usize,
    pub cells: Vec<Cell>,
    pub rules: Vec<Box<dyn Rule>>,
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

    fn set_cell(&mut self, n: u16, index: usize) {
        self.cells[index] = Cell::single(n, true);
        for rule in &self.rules {
            for inner_index in rule.updates(&self, index) {
                self.cells[inner_index].remove(n);
            }
        }
    }

    fn update_cell(&mut self, n: u16, index: usize, queue: &mut PriorityQueue<usize, usize>) {
        self.cells[index] = Cell::single(n, false);
        for rule in &self.rules {
            for inner_index in rule.updates(&self, index) {
                let cell = &mut self.cells[inner_index];
                cell.remove(n);
                queue.change_priority(&inner_index, cell.available.len());
            }
        }
    }

    // Dette skal gøres om, siden det er blevet til et clusterfuck
    // Eventuel lav en "solver" struct som har metoder til ting som 
    // Man har brug for (solver) self.pop_q_and_update() og pop_branch / push_branch.


    pub fn solve(&mut self) {
        let mut priority_queue = PriorityQueue::new();

        type Branch = (Vec<Cell>, PriorityQueue<usize, usize>, usize);
        let mut branch_stack: Vec<Branch> = vec![];
        let mut just_popped_index = None;

        for (index, cell) in self.cells.iter().enumerate() {
            if !cell.from_file {
                priority_queue.push(index, cell.available.len());
            }
        }

        while let Some((index, entropy)) = priority_queue.pop() {
            if entropy == 0 {
                //Vi er nået til at der er ingen løsning
                if let Some((popped_cells, popped_queue, prev_branch_index)) = branch_stack.pop() {
                    self.cells = popped_cells;
                    priority_queue = popped_queue;
                    just_popped_index = Some(prev_branch_index + 1);
                    continue;
                }
                panic!("Der er ingen løsning til denne sudoku. Der var ingen løsning med den har ikke branched");
            } else if entropy == 1 {
                let n = self.cells[index].available[0];
                self.update_cell(n, index, &mut priority_queue);
            } else {
                let branch_choice_index = just_popped_index.take().unwrap_or(0);

                println!(
                    "Brancher på dybde {}, og vælger {branch_choice_index}",
                    branch_stack.len()
                );

                //Hvis alle branches er blevet prøvet og den ikke har kunne
                //finde en løsning i nogen af dem
                if branch_choice_index >= entropy {
                    //Vi bliver nødt til at poppe dybere :(
                    let Some((popped_cells, popped_queue, prev_branch_index)) = branch_stack.pop()
                    else {
                        panic!("Der er ingen løsning til denne sudoku. Der var ingen løsning med den har ikke branched");
                    };
                    self.cells = popped_cells;
                    priority_queue = popped_queue;
                    just_popped_index = Some(prev_branch_index + 1);
                    continue;
                }

                branch_stack.push((
                    self.cells.clone(),
                    priority_queue.clone(),
                    branch_choice_index,
                ));

                let n = self.cells[index].available[branch_choice_index];

                self.update_cell(n, index, &mut priority_queue);

                //todo!("Bliver nødt til at gætte. Kan ikke komme videre indtil videre")
            }
        }
        //Enten er den solved ellers skal den poppe branch stack.
        //Ved ikke helt endnu
    }
}

impl FromStr for Sudoku {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sudoku = Sudoku::new(
            9,
            vec![
                Box::new(RowRule),
                Box::new(ColumnRule),
                Box::new(SquareRule),
            ],
        );

        //let (uløst, _løsning) = s.split_once("\n\n").unwrap();

        for (index, part) in s.split(',').map(str::trim).enumerate() {
            let n = part.parse()?;
            if n != 0 {
                sudoku.set_cell(n, index);
            }
        }

        Ok(sudoku)
    }
}
#[derive(Debug, Clone)]
pub struct Cell {
    available: Vec<u16>,

    //Hvis den ikke er udledt. Men det er i starten af banen.
    from_file: bool,
}

impl Cell {
    fn single(n: u16, from_file: bool) -> Self {
        Self {
            available: vec![n],
            from_file,
        }
    }
    fn new_with_range(range: Range<u16>) -> Self {
        Self {
            available: range.collect(),
            from_file: false,
        }
    }
    fn remove(&mut self, n: u16) {
        self.available.retain(|i| *i != n);
    }
}

#[test]
fn read_file_test() {
    let file_str = std::fs::read_to_string("./sudokuUløst").unwrap();
    let sudoku: Sudoku = file_str.parse().unwrap();

    println!("{sudoku:#?}");
}

#[test]
fn solve_test() {
    let file_str = std::fs::read_to_string("./sudokuUløst").unwrap();
    let mut sudoku: Sudoku = file_str.parse().unwrap();

    sudoku.solve();

    println!("{sudoku:#?}");
}
