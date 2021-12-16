use grid::*;
use std::cmp::min;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result;
use std::fs;

#[derive(Clone, Copy, Eq, PartialEq)]
struct Node {
    visited: bool,
    cost: u32,
    cumulative_cost: u32,
}

impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.cost)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cumulative_cost
            .cmp(&self.cumulative_cost)
            .then_with(|| self.cost.cmp(&other.cost))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("File not readable");

    let mut grid: Grid<Node> = grid![];

    for line in input.lines() {
        let l: Vec<Node> = line
            .trim()
            .chars()
            .map(|c| Node {
                cost: c.to_digit(10).unwrap(),
                visited: false,
                cumulative_cost: 999999999,
            })
            .collect();

        grid.push_row(l);
    }

    grid.get_mut(0, 0).unwrap().visited = true;
    grid.get_mut(0, 0).unwrap().cumulative_cost = 0;

    let east_node = grid.get_mut(0, 1).unwrap();
    east_node.cumulative_cost = east_node.cost;

    let south_node = grid.get_mut(1, 0).unwrap();
    south_node.cumulative_cost = south_node.cost;

    while let Some(n) = find_node(&grid) {
        process_node(n, &mut grid);

        // println!("-------------------");
        // println!("{:?}", grid);
    }

    println!(
        "Answer 1: {}",
        grid.get(grid.rows() - 1, grid.cols() - 1)
            .unwrap()
            .cumulative_cost
    );
}

fn find_node(grid: &Grid<Node>) -> Option<(usize, usize)> {
    let mut min_cum_cost = 999999999;
    let mut min_coord = None;

    for i in 0..grid.rows() {
        for j in 0..grid.cols() {
            let n = grid.get(i, j).unwrap();

            if !n.visited && n.cumulative_cost < min_cum_cost {
                min_cum_cost = n.cumulative_cost;
                min_coord = Some((i, j));
            }
        }
    }

    // println!("Node found: {:?}", min_coord);

    min_coord
}

fn process_node(coord: (usize, usize), grid: &mut Grid<Node>) {
    let base_cost = grid.get_mut(coord.0, coord.1).unwrap().cumulative_cost;

    // N
    if coord.0 >= 1 {
        let n = grid.get_mut(coord.0 - 1, coord.1).unwrap();

        if !n.visited {
            n.cumulative_cost = min(n.cost + base_cost, n.cumulative_cost);
        }
    }

    // E

    if coord.1 < grid.cols() - 1 {
        let n = grid.get_mut(coord.0, coord.1 + 1).unwrap();

        if !n.visited {
            n.cumulative_cost = min(n.cost + base_cost, n.cumulative_cost);
        }
    }

    // S
    if coord.0 < grid.rows() - 1 {
        let n = grid.get_mut(coord.0 + 1, coord.1).unwrap();

        if !n.visited {
            n.cumulative_cost = min(n.cost + base_cost, n.cumulative_cost);
        }
    }

    // W
    if coord.1 > 0 {
        let n = grid.get_mut(coord.0, coord.1 - 1).unwrap();

        if !n.visited {
            n.cumulative_cost = min(n.cost + base_cost, n.cumulative_cost);
        }
    }

    grid.get_mut(coord.0, coord.1).unwrap().visited = true;
}
