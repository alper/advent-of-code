use grid::*;
use itertools::Itertools;
use pathfinding::prelude::dijkstra;
use std::cmp::min;
use std::collections::BinaryHeap;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result;
use std::fs;

#[derive(Clone, Copy, Eq, PartialEq)]
struct Node {
    cost: usize,
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("File not readable");

    // let input = "123\n782\n456";

    // Part 2
    let mut grid: Grid<Node> = grid![];

    for line in input.lines() {
        let mut l: Vec<Node> = line
            .trim()
            .chars()
            .map(|c| Node {
                cost: c.to_digit(10).unwrap() as usize,
            })
            .collect();

        let clone_base = l.clone();

        for i in 1..5 {
            l.append(
                &mut clone_base
                    .iter()
                    .cloned()
                    .map(|d| increment_node(i, &d))
                    .collect::<Vec<Node>>(),
            );
        }

        grid.push_row(l);
    }

    let rows = grid.rows();

    for i in 1..5 {
        for r in 0..rows {
            let v: Vec<Node> = grid
                .iter_row(r)
                .cloned()
                .map(|d| increment_node(i, &d))
                .collect();

            grid.push_row(v);
        }
    }

    println!("Grid");
    render_grid(&grid);

    let goal = (grid.rows() - 1, grid.cols() - 1);

    println!("Goal: {:?}", goal);

    let result = dijkstra(&(0, 0), |&(x, y)| neighbors((x, y), &grid), |&p| p == goal);

    println!("Result: {:?}", result);
}

fn render_grid(grid: &Grid<Node>) {
    for i in 0..grid.rows() {
        for j in 0..grid.cols() {
            let n = grid.get(i, j).unwrap();

            print!("{} ", n.cost);
        }
        print!("\n");
    }
}

fn increment_node(c: usize, n: &Node) -> Node {
    Node {
        cost: if n.cost + c < 10 {
            n.cost + c
        } else {
            n.cost + c - 9
        },
    }
}

fn neighbors(coord: (usize, usize), grid: &Grid<Node>) -> Vec<((usize, usize), usize)> {
    // println!("Process: {:?}", coord);

    let mut res = vec![];

    // N
    if coord.0 >= 1 {
        let n = grid.get(coord.0 - 1, coord.1).unwrap();

        res.push(((coord.0 - 1, coord.1), n.cost));
    }

    // E

    if coord.1 < grid.cols() - 1 {
        let n = grid.get(coord.0, coord.1 + 1).unwrap();

        res.push(((coord.0, coord.1 + 1), n.cost));
    }

    // S
    if coord.0 < grid.rows() - 1 {
        let n = grid.get(coord.0 + 1, coord.1).unwrap();

        res.push(((coord.0 + 1, coord.1), n.cost));
    }

    // W
    if coord.1 > 0 {
        let n = grid.get(coord.0, coord.1 - 1).unwrap();

        res.push(((coord.0, coord.1 - 1), n.cost));
    }

    // println!("Return: {:?}", res);

    return res;
}
