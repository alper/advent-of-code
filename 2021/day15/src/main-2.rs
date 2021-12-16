use grid::*;
use itertools::Itertools;
use std::cmp::min;
use std::collections::BinaryHeap;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result;
use std::fs;

#[derive(Clone, Copy, Eq, PartialEq)]
struct Node {
    visited: bool,
    cost: u32,
    cumulative_cost: u32,
    coord: (usize, usize),
}

impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self.coord)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cumulative_cost
            .cmp(&self.cumulative_cost)
            .then_with(|| self.coord.cmp(&other.coord))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let input = fs::read_to_string("test_input.txt").expect("File not readable");

    // let input = "123\n782\n456";

    // Part 2
    let mut grid: Grid<Node> = grid![];

    for line in input.lines() {
        let mut l: Vec<Node> = line
            .trim()
            .chars()
            .map(|c| Node {
                cost: c.to_digit(10).unwrap(),
                visited: false,
                cumulative_cost: u32::MAX,
                coord: (0, 0),
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
    // render_grid(CostToRender::COST, &grid);

    // println!("Last row");
    // println!(
    //     "{:?}",
    //     grid.iter_row(grid.rows() - 1).collect::<Vec<&Node>>()
    // );

    // grid.get_mut(0, 0).unwrap().visited = true;
    grid.get_mut(0, 0).unwrap().cumulative_cost = 0;

    let mut queue: BinaryHeap<Node> = BinaryHeap::new();
    queue.push(*grid.get(0, 0).unwrap());

    let mut counter = 0;

    'outer: while let Some(node) = queue.pop() {
        let nodes = process_node(node.coord, &mut grid);

        // println!("Nodes: {:?}", nodes);

        for n in nodes {
            // if n.coord == (grid.rows() - 1, grid.cols() - 1) {
            //     break 'outer;
            // }

            if !grid.get(n.coord.0, n.coord.1).unwrap().visited {
                queue.push(n);
            }
        }

        counter += 1;
        // render_grid(CostToRender::CUMULATIVE_COST, &grid);
        // render_queue(&queue);
        // println!("Queue: {:?}", queue);
    }

    println!(
        "Answer 2: {}",
        grid.get(grid.rows() - 1, grid.cols() - 1)
            .unwrap()
            .cumulative_cost
    );
}

fn increment_node(c: u32, n: &Node) -> Node {
    Node {
        visited: n.visited,
        cost: if n.cost + c < 10 {
            n.cost + c
        } else {
            n.cost + c - 9
        },
        cumulative_cost: n.cumulative_cost,
        coord: n.coord,
    }
}

enum CostToRender {
    COST,
    CUMULATIVE_COST,
}

fn render_grid(c: CostToRender, grid: &Grid<Node>) {
    for i in 0..grid.rows() {
        for j in 0..grid.cols() {
            let n = grid.get(i, j).unwrap();

            if n.visited {
                print!("/");
            }

            match c {
                CostToRender::COST => print!("{} ", n.cost),
                CostToRender::CUMULATIVE_COST => {
                    print!("{} ", n.cumulative_cost)
                }
            }
        }
        print!("\n");
    }
}

fn render_queue(queue: &BinaryHeap<Node>) {
    print!("Queue: ");

    for n in queue {
        print!("{:?}:{} ", n.coord, n.cumulative_cost);
    }

    print!("\n");
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

    return min_coord;
}

fn process_node(coord: (usize, usize), grid: &mut Grid<Node>) -> Vec<Node> {
    // println!("Process: {:?}", coord);

    let base_cost = grid.get_mut(coord.0, coord.1).unwrap().cumulative_cost;

    let mut res = vec![];

    // N
    if coord.0 >= 1 {
        let n = grid.get_mut(coord.0 - 1, coord.1).unwrap();

        if !n.visited {
            n.cumulative_cost = min(n.cost + base_cost, n.cumulative_cost);
            n.coord = (coord.0 - 1, coord.1);
            res.push(n.clone());
        }
    }

    // E

    if coord.1 < grid.cols() - 1 {
        let n = grid.get_mut(coord.0, coord.1 + 1).unwrap();

        if !n.visited {
            n.cumulative_cost = min(n.cost + base_cost, n.cumulative_cost);
            n.coord = (coord.0, coord.1 + 1);
            res.push(n.clone());
        }
    }

    // S
    if coord.0 < grid.rows() - 1 {
        let n = grid.get_mut(coord.0 + 1, coord.1).unwrap();

        if !n.visited {
            n.cumulative_cost = min(n.cost + base_cost, n.cumulative_cost);
            n.coord = (coord.0 + 1, coord.1);
            res.push(n.clone());
        }
    }

    // W
    if coord.1 > 0 {
        let n = grid.get_mut(coord.0, coord.1 - 1).unwrap();

        if !n.visited {
            n.cumulative_cost = min(n.cost + base_cost, n.cumulative_cost);
            n.coord = (coord.0, coord.1 - 1);
            res.push(n.clone());
        }
    }

    grid.get_mut(coord.0, coord.1).unwrap().visited = true;

    return res;
}
