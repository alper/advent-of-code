use std::fs;
use itertools::{Itertools};
use array2d::Array2D;

fn main() {
    let input = fs::read_to_string("test_input.txt").expect("File not readable");

    let parsed = input
        .lines()
        .map(|l| l.chars()
            .map(|c| c.to_digit(10).unwrap() as isize)
            .collect::<Vec<isize>>())
        .collect::<Vec<_>>();

    println!("Input: {:?}", parsed);

    let a = Array2D::from_rows(&parsed);

    println!("Array: {:?}", a);
    for row in a.as_rows() {
        println!("{:?}", row);
    }


    let mut low_points: Vec<(usize, usize)> = vec![];

    for i in 0..a.num_rows() {
        for j in 0..a.num_columns() {
            let val = a.get(i, j).unwrap();

            let mut lower = false;

            // Up
            if i > 0 {
                if let Some(val_u) = a.get(i-1, j) {
                    if val_u <= val {
                        lower = true;
                    }
                }
            }

            // Down
            if let Some(val_d) = a.get(i+1, j) {
                if val_d <= val {
                    lower = true;
                }
            }

            // Left
            if j > 0 {
                if let Some(val_l) = a.get(i, j-1) {
                    if val_l <= val {
                        lower = true;
                    }
                }
            }

            // Right
            if let Some(val_r) = a.get(i, j+1) {
                if val_r <= val {
                    lower = true;
                }
            }

            if !lower {
                println!("Lowest: {} at ({}, {})", val, i, j);
                low_points.push((i, j));
            }
        }
    }

    println!("Low points: {:?}", low_points);

    println!("Answer 1: {}", low_points.iter().map(|(i, j)| a.get(*i, *j).unwrap()+1).sum::<isize>());

    // Part 2

}

fn basin_size(point: (usize, usize), arr: Array2D) {

}