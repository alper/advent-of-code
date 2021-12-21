use grid::*;
use itertools::Itertools;
use std::fs;

const PADDING: usize = 10;

fn main() {
    let input = fs::read_to_string("input.txt").expect("File not readable");

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

    println!("Gird ");
    pretty_print(&grid);

    println!("Input: {:?}", enhancement_algorithm);

    let mut new_grid1 = Grid::init(grid.rows(), grid.cols(), '.');

    for row in 1..grid.rows() - 1 {
        for col in 1..grid.cols() - 1 {
            let cv = convolute(&grid, row, col);
            let new_pix = enhance_convolution(&cv, &enhancement_algorithm);

            new_grid1[row][col] = new_pix;
        }
    }
    println!("Gird 2");
    pretty_print(&new_grid1);

    let mut new_grid2 = Grid::init(grid.rows(), grid.cols(), '.');
    for row in 1..grid.rows() - 1 {
        for col in 1..grid.cols() - 1 {
            let cv = convolute(&new_grid1, row, col);
            let new_pix = enhance_convolution(&cv, &enhancement_algorithm);

            new_grid2[row][col] = new_pix;
        }
    }

    println!("Gird 3");
    pretty_print(&new_grid2);

    println!(
        "Answer 1: {}",
        new_grid2.iter().filter(|c| **c == '#').count()
    );
}

#[test]
fn test_enhance_convolution() {
    let algo = "######......";

    let convo1 = vec!['.', '#', '.', '#'];
    assert_eq!(enhance_convolution(&convo1, &algo), '#');

    let convo2 = vec!['#', '.', '.', '.'];
    assert_eq!(enhance_convolution(&convo2, &algo), '.');
}

fn enhance_convolution(cv: &Vec<char>, algo: &str) -> char {
    let bitstring: String = cv
        .iter()
        .map(|c| match c {
            '.' => '0',
            _ => '1',
        })
        .collect();

    let index = usize::from_str_radix(&bitstring, 2).unwrap();

    algo.chars().nth(index).unwrap()
}

#[test]
fn test_convolute() {
    let grid = grid![[1, 2, 3][4, 5, 6][7, 8, 9]];

    let v = convolute(&grid, 1, 1);

    assert_eq!(v[0], 1);
}

fn convolute<T: Copy>(grid: &Grid<T>, row: usize, col: usize) -> Vec<T> {
    vec![
        *grid.get(row - 1, col - 1).unwrap(),
        *grid.get(row - 1, col).unwrap(),
        *grid.get(row - 1, col + 1).unwrap(),
        *grid.get(row, col - 1).unwrap(),
        *grid.get(row, col).unwrap(),
        *grid.get(row, col + 1).unwrap(),
        *grid.get(row + 1, col - 1).unwrap(),
        *grid.get(row + 1, col).unwrap(),
        *grid.get(row + 1, col + 1).unwrap(),
    ]
}

fn pretty_print(grid: &Grid<char>) {
    for r in 0..grid.rows() {
        for el in grid.iter_row(r) {
            print!("{}", el);
        }
        println!();
    }
}
