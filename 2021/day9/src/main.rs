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

fn basin_size(point: (usize, usize), arr: &mut Array2D<isize>) -> usize {


    1
}

#[test]
fn test_basin_size_rec() {
    let rows: Vec<Vec<isize>> = vec![
        vec![1, 9, 3],
        vec![4, 9, 6]
        ];
    let mut array = Array2D::from_rows(&rows);

    println!("Basin size: {}", basin_size_rec((0, 0), &mut array));

    assert_eq!(1, 2);
}

fn basin_size_rec(point: (usize, usize), arr: &mut Array2D<isize>) -> usize {
    println!("Got arr: {:?}", arr);
    println!("Got point: {:?}", point);

    if let Some(p) = arr.get_mut(point.0, point.1) {
        if *p == 9 || *p == -1 {
            return 0;
        } else {
            arr.set(point.0, point.1, -1);

            let mut count = 1;

            if point.0 > 0 {
                count += basin_size_rec((point.0-1, point.1), arr);
            }

            count += basin_size_rec((point.0+1, point.1), arr);

            if point.1 > 0 {
                count += basin_size_rec((point.0, point.1-1), arr);

            }

            count += basin_size_rec((point.0, point.1+1), arr);

            return count;
        }
    } else {
        return 0;
    }

}