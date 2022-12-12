use array2d::{self, Array2D};
use itertools::Itertools;
use pathfinding::prelude::dijkstra;
use std::fs;

fn print_array(array: &Array2D<u32>) {
    for row in array.rows_iter() {
        for item in row {
            print!("{:2} ", item);
        }
        println!()
    }
}

fn print_visited_array(array: &Array2D<bool>) {
    for row in array.rows_iter() {
        for item in row {
            if *item {
                print!("1")
            } else {
                print!("0")
            }
        }
        println!()
    }
}

fn path_length(array: &Array2D<u32>, from: (usize, usize), to: (usize, usize)) -> Option<usize> {
    match dijkstra(
        &from,
        |&(x, y)| {
            let mut result = vec![];
            let current_height = *array.get(x, y).unwrap();

            if x > 0 {
                if let Some(n) = array.get(x - 1, y) {
                    if current_height == *n || current_height > *n || current_height + 1 == *n {
                        result.push((x - 1, y))
                    }
                }
            }

            if let Some(n) = array.get(x + 1, y) {
                if current_height == *n || current_height + 1 == *n || current_height > *n {
                    result.push((x + 1, y))
                }
            }
            if y > 0 {
                if let Some(n) = array.get(x, y - 1) {
                    if current_height == *n || current_height + 1 == *n || current_height > *n {
                        result.push((x, y - 1))
                    }
                }
            }
            if let Some(n) = array.get(x, y + 1) {
                if current_height == *n || current_height + 1 == *n || current_height > *n {
                    result.push((x, y + 1))
                }
            }

            // println!("Found: {:?}", result);
            // println!();

            return result.into_iter().map(|p| (p, 1));
        },
        |&p| p == to,
    ) {
        Some(result) => Some(result.0.len()),
        None => None,
    }
}

fn array_find(array: &Array2D<u32>, needle: u32) -> (usize, usize) {
    for i in 0..array.num_rows() {
        for j in 0..array.num_columns() {
            if needle == *array.get(i, j).unwrap() {
                return (i, j);
            }
        }
    }
    panic!("Shouldn't happen.")
}

fn main() {
    let input = fs::read_to_string("test_input.txt").expect("File not readable");

    let v: Vec<_> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    'S' => 0,
                    'E' => 27,
                    _ => (c as u32) - ('a' as u32) + 1,
                })
                .collect::<Vec<u32>>()
        })
        .collect();
    let array = Array2D::from_rows(&v);

    let mut visited = Array2D::filled_with(false, array.num_rows(), array.num_columns());

    print_array(&array);

    let start = array_find(&array, 0);
    let goal = array_find(&array, 27);

    println!("Start: {:?}", start);
    println!("Goal: {:?}", goal);

    // Part 1
    println!("Part 1");

    let result = dijkstra(
        &start,
        |&(x, y)| {
            visited.set(x, y, true);

            let mut result = vec![];
            let current_height = *array.get(x, y).unwrap();

            // println!("Successors for: {x} {y} at {current_height}");

            if x > 0 {
                if let Some(n) = array.get(x - 1, y) {
                    if current_height == *n || current_height > *n || current_height + 1 == *n {
                        result.push((x - 1, y))
                    }
                }
            }

            if let Some(n) = array.get(x + 1, y) {
                if current_height == *n || current_height + 1 == *n || current_height > *n {
                    result.push((x + 1, y))
                }
            }
            if y > 0 {
                if let Some(n) = array.get(x, y - 1) {
                    if current_height == *n || current_height + 1 == *n || current_height > *n {
                        result.push((x, y - 1))
                    }
                }
            }
            if let Some(n) = array.get(x, y + 1) {
                if current_height == *n || current_height + 1 == *n || current_height > *n {
                    result.push((x, y + 1))
                }
            }

            // println!("Found: {:?}", result);
            // println!();

            return result.into_iter().map(|p| (p, 1));
        },
        |&p| p == goal,
    );

    print_visited_array(&visited);

    println!("Answer part 1: {:?}", result.unwrap().0.len() - 1);

    // Part 2
    println!("Part 2");

    let candidates = (0..(array.num_rows()))
        .cartesian_product(0..(array.num_columns()))
        .filter(|&(x, y)| *array.get(x, y).unwrap() == 1);

    println!("Answer part 2: {:?}", candidates.collect::<Vec<_>>());
}
