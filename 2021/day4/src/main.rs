use std::fs;
use std::fmt;
// use itertools::{Itertools};
use grid::*;
use regex::Regex;

const BOARD_SIZE: usize = 5;

struct BoardEntry {
    board_value: usize,
    marked: bool,
}

type BingoBoard = Grid<BoardEntry>;

impl fmt::Debug for BoardEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.marked {
            write!(f, "{}", self.board_value)
        } else {
            write!(f, "*{}*", self.board_value)
        }
    }
}

impl Default for BoardEntry {
    fn default() -> Self {
        BoardEntry {
            board_value: 0,
            marked: false,
        }
    }
}

#[test]
fn test_board_input_to_grid () {
    let test_board = "22 13 17 11  0\n 8  2 23  4 24\n21  9 14 16  7\n 6 10  3 18  5\n 1 12 20 15 19";

    println!("Original board: \n{}", test_board);

    let b: Grid<BoardEntry> = board_input_to_grid(test_board);

    assert_eq!(b.get(0, 0).unwrap().board_value, 22);
    assert_eq!(b.get(4, 4).unwrap().board_value, 19)
}


fn board_input_to_grid(input: &str) -> BingoBoard {
    let re = Regex::new(r"\s{1,2}").unwrap();

    println!("Input to function: {}", input);

    let numbers: Vec<BoardEntry> = re.split(input.trim()) // .map(|d| { println!("{}", d); return d} )
        .map(|n| n.parse::<usize>().unwrap())
        .map(|n| BoardEntry {board_value: n, marked: false})
        .collect();

    Grid::from_vec(numbers, BOARD_SIZE)
}

#[test]
fn test_mark_number_on_board() {
    let mut board: BingoBoard = Grid::from_vec(vec![BoardEntry {board_value: 1, marked: false}, BoardEntry {board_value: 2, marked: true}, BoardEntry {board_value: 3, marked: false}, BoardEntry {board_value: 4, marked: false}], 2);

    assert_eq!(board.get(0, 1).unwrap().marked, true);

    board.get_mut(1, 1).unwrap().marked = true;

    println!("Test mark: {:?}", board);
    assert_eq!(board.get(1, 1).unwrap().marked, true);
}

fn mark_number_on_board(b: &mut BingoBoard, n: usize) {
    for e in b.iter_mut() {
        if e.board_value == n {
            e.marked = true;
        }
    }
}

#[test]
fn test_board_score() {
    let mut board: BingoBoard = Grid::from_vec(vec![BoardEntry {board_value: 1, marked: true}, BoardEntry {board_value: 2, marked: true}, BoardEntry {board_value: 3, marked: true}, BoardEntry {board_value: 4, marked: false}], 2);

    let score = board_score(board);

    assert_eq!(score, 4);
}

fn board_score(b: &BingoBoard) -> usize {
    // Check if row won
    let mut won = false;

    for i in 0..b.rows() {
        if b.iter_row(i).filter(|be| !be.marked).count() == 0 {
            won = true;
        }
    }

    // Check if col won
    for i in 0..b.cols() {
        if b.iter_col(i).filter(|be| !be.marked).count() == 0 {
            won = true;
        }
    }

    if won {
        let els: Vec<usize> = b.iter().filter(|be| !be.marked).map(|be| be.board_value).collect();

        return els.iter().sum();
    } else {
        return 0;
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("File not readable");
    let mut input_parts = input.split("\n\n");

    let numbers: Vec<usize> = input_parts.next().unwrap().split(",").map(|p| p.parse::<usize>().unwrap()).collect();

    println!("Numbers: {:?}", numbers);

    let mut boards: Vec<BingoBoard> = input_parts.map(|p| board_input_to_grid(p)).collect();

    'outer: for n in numbers {
        for b in &mut boards {
            mark_number_on_board(b, n);

            let score = board_score(b);
            if score > 0 {
                println!("Answer: {}", score * n);
                break 'outer;
            }
        }
    }


    println!("V: {:?}", boards);
}
