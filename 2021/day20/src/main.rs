use grid::*;
use itertools::Itertools;
use std::fs;

const PADDING: usize = 2;

fn main() {
    let input = fs::read_to_string("test_input.txt").expect("File not readable");

    let mut lines = input.lines();

    let enhancement_algorithm = lines.next().unwrap().trim();

    let _blank_line = lines.next();

    let mut grid: Grid<char> = grid![];

    for line in lines {
        grid.push_row(line.trim().chars().collect::<Vec<char>>());
    }

    let cols = grid.cols();

    // Add rows above and below
    for _ in 0..PADDING {
        grid.insert_row(0, vec!['.'; grid.cols()]);
        grid.push_row(vec!['.'; grid.cols()]);
    }

    // Add cols ahead and in front
    for _ in 0..PADDING {
        grid.insert_col(0, vec!['.'; grid.rows()]);
        grid.push_col(vec!['.'; grid.rows()]);
    }

    println!("Input: {:?}", enhancement_algorithm);

    pretty_print(&grid);
}

fn pretty_print(grid: &Grid<char>) {
    for r in 0..grid.rows() {
        for el in grid.iter_row(r) {
            print!("{}", el);
        }
        println!();
    }
}
