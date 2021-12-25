use grid::*;
use itertools::Itertools;
use std::fmt::Debug;
use std::fs;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Direction {
    East,
    South,
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Cucumber {
    heading: Direction,
    heading_clear: bool,
}

impl Debug for Cucumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self.heading {
                Direction::East => ">",
                Direction::South => "v",
            }
        )
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("File not readable");

    let mut grid: Grid<Option<Cucumber>> = grid![];

    for line in input.lines() {
        let r = line
            .trim()
            .chars()
            .map(|c| match c {
                'v' => Some(Cucumber {
                    heading: Direction::South,
                    heading_clear: false,
                }),
                '>' => Some(Cucumber {
                    heading: Direction::East,
                    heading_clear: false,
                }),
                _ => None,
            })
            .collect::<Vec<_>>();

        grid.push_row(r);
    }

    pretty_print(&grid);

    println!();

    let mut steps = 0;
    loop {
        let moves = step_grid(&mut grid);
        steps += 1;

        if moves == 0 {
            break;
        }
    }

    pretty_print(&grid);
    println!("Answer 1: {}", steps);
}

#[test]
fn test_with_wrap() {
    let g = grid![[1,2]
                            [3,4]];

    assert_eq!(get_with_wrap(&g, 0, 0, Direction::East), (0, 1));
    assert_eq!(get_with_wrap(&g, 1, 0, Direction::South), (0, 0));
    assert_eq!(get_with_wrap(&g, 0, 1, Direction::East), (0, 0));
}

fn get_with_wrap<T>(grid: &Grid<T>, row: usize, col: usize, heading: Direction) -> (usize, usize) {
    match heading {
        Direction::East => {
            if col + 1 >= grid.cols() {
                return (row, 0);
            } else {
                return (row, col + 1);
            }
        }
        Direction::South => {
            if row + 1 >= grid.rows() {
                return (0, col);
            } else {
                return (row + 1, col);
            }
        }
    }
}

fn step_grid(grid: &mut Grid<Option<Cucumber>>) -> usize {
    let mut moves = 0;

    // Check east
    for row in 0..grid.rows() {
        for col in 0..grid.cols() {
            let pos = get_with_wrap(grid, row, col, Direction::East);

            let east_clear = match grid.get(pos.0, pos.1).unwrap() {
                Some(_) => false,
                None => true,
            };

            if let Some(c) = grid.get_mut(row, col).unwrap() {
                if c.heading == Direction::East {
                    c.heading_clear = east_clear;
                }
            }
        }
    }

    // Move east
    for row in 0..grid.rows() {
        for col in 0..grid.cols() {
            let new_pos = get_with_wrap(grid, row, col, Direction::East);

            if let Some(c) = grid.get_mut(row, col).unwrap() {
                if c.heading == Direction::East && c.heading_clear {
                    moves += 1;

                    let mut new_c = c.clone();
                    new_c.heading_clear = false;

                    grid[new_pos.0][new_pos.1] = Some(new_c);
                    grid[row][col] = None;
                }
            }
        }
    }

    // Check south
    for row in 0..grid.rows() {
        for col in 0..grid.cols() {
            let pos = get_with_wrap(grid, row, col, Direction::South);

            let south_clear = match grid.get(pos.0, pos.1).unwrap() {
                Some(_) => false,
                None => true,
            };

            if let Some(c) = grid.get_mut(row, col).unwrap() {
                if c.heading == Direction::South {
                    c.heading_clear = south_clear;
                }
            }
        }
    }

    // Move south
    for row in 0..grid.rows() {
        for col in 0..grid.cols() {
            let new_pos = get_with_wrap(grid, row, col, Direction::South);

            if let Some(c) = grid.get_mut(row, col).unwrap() {
                if c.heading == Direction::South && c.heading_clear {
                    moves += 1;

                    let mut new_c = c.clone();
                    new_c.heading_clear = false;

                    grid[new_pos.0][new_pos.1] = Some(new_c);
                    grid[row][col] = None;
                }
            }
        }
    }

    moves
}

fn pretty_print(grid: &Grid<Option<Cucumber>>) {
    for r in 0..grid.rows() {
        for el in grid.iter_row(r) {
            match el {
                None => print!("."),
                Some(c) => print!("{:?}", c),
            }
            print!(" ");
        }
        println!();
    }
}
