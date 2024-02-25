use std::{
    cmp::Ordering, fmt::{Display, Write}, num::ParseIntError, ops::Range, str::FromStr
};

use priority_queue::PriorityQueue;

use crate::rules::{ColumnRule, RowRule, Rule, SquareRule};

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
        self.cells[index] = Cell::single(n);
        for rule in &self.rules {
            for inner_index in rule.updates(&self, index) {
                self.cells[inner_index].remove(n);
            }
        }
    }

    fn update_cell(&mut self, n: u16, index: usize, queue: &mut PriorityQueue<usize, Entropy>) {
        self.cells[index] = Cell::single(n);
        for rule in &self.rules {
            for inner_index in rule.updates(&self, index) {
                let cell = &mut self.cells[inner_index];
                cell.remove(n);
                queue.change_priority(&inner_index, Entropy(cell.available.len()));
            }
        }
    }

    pub fn solve(&mut self) {
        let mut pri_queue = PriorityQueue::new();
        for (index, cell) in self.cells.iter().enumerate() {
            if !cell.locked_in {
                pri_queue.push(index, Entropy(cell.available.len()));
            }
        }

        let mut branch_stack: Vec<(Vec<Cell>, PriorityQueue<usize, Entropy>)> = vec![];

        while let Some((index, entropy)) = pri_queue.pop() {
            match entropy.0 {
                0 => {
                    //Der er ingen løsning på den nuværende branch. Derfor popper vi en branch og løser den i stedet
                    let Some((cells, new_pri_queue)) = branch_stack.pop() else {
                        panic!("No Solution. Failed to pop branch queue when entropy was 0 in cell {index}");
                    };

                    self.cells = cells;
                    pri_queue = new_pri_queue;
                }
                1 => self.update_cell(self.cells[index].available[0], index, &mut pri_queue),
                _ => {
                    //Der er flere muligheder for hvad der kan vælges. Derfor pushes state på branch stacken og der vælges en mulighed
                    // Den vælger altid den forreste i listen
                    // Det kan køre en lille bitte smule hurtigere hvis den vælger den sidste i stedet men whatever
                    let n = self.cells[index].available[0];

                    let mut cloned_cells = self.cells.clone();

                    //Fjern n fra cloned_cells så den ikke kan blive valgt igen!
                    cloned_cells[index].available.remove(0);

                    let mut cloned_queue = pri_queue.clone();
                    //Siden den allerede er poppet i den nuværende queue skal den indsættes igen
                    // i den cloned queue. Ellers vil clonen aldrig løse index cellen.
                    cloned_queue.push(index, Entropy(entropy.0 - 1));
                    branch_stack.push((cloned_cells, cloned_queue));

                    self.update_cell(n, index, &mut pri_queue);
                }
            }
        }
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

        for (index, part) in s.split(',').map(str::trim).enumerate() {
            let n = part.parse()?;
            if n != 0 {
                sudoku.set_cell(n, index);
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
    available: Vec<u16>,
    locked_in: bool,
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
    fn remove(&mut self, n: u16) {
        self.available.retain(|i| *i != n);
        if self.locked_in && self.available.len() == 0 {
            panic!("Something went seriously wrong. Removed the only value in a locked cell");
        }
    }
    pub fn contains(&self, n: u16) -> bool {
        self.available.contains(&n)
    }
}

#[test]
fn read_file_test() {
    let file_str = std::fs::read_to_string("./sudokuUløst").unwrap();
    let sudoku: Sudoku = file_str.parse().unwrap();

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
            sudoku.solve();
            sudoku_name = filename;
        }

        let solution = sudoku.to_string();
        if let Some(other_solution) = solutions.get(&sudoku_name) {
            assert_eq!(solution, *other_solution);
        } else {
            solutions.insert(sudoku_name, solution);
        }
    }

    for (key, value) in solutions {
        println!("{key}: {value}");
    }
}