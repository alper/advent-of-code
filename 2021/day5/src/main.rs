use std::fs;
use std::cmp::{min, max};
use itertools::{Itertools};
use ndarray::{Array2};

fn main() {
    let input = fs::read_to_string("input.txt").expect("File not readable");

    let v: Vec<Vec<&str>> = input.lines().map(|l| l.split(" -> ").collect()).collect();

    println!("Input: {:?}", v);

    // Part 1
    let mut a: Array2<usize> = Array2::zeros((1000, 1000));

    for l in &v {
        let start: (usize, usize) = l[0].split(',').map(|d| d.parse::<usize>().unwrap()).collect_tuple().unwrap();
        let end: (usize, usize) = l[1].split(',').map(|d| d.parse::<usize>().unwrap()).collect_tuple().unwrap();

        println!("{:?} -> {:?}", start, end);

        if start.0 == end.0 {
            for i in min(start.1, end.1)..=max(start.1, end.1) {
                a[[i, start.0]] += 1;
            }
        }

        if start.1 == end.1 {
            for j in min(start.0, end.0)..=max(start.0, end.0) {
                a[[start.1, j]] += 1;
            }
        }
    }

    println!("{:?}", a);

    a.mapv_inplace(|d| {
        if d > 1 {
            1
        } else {
            0
        }
    });

    println!("Answer 1: {:?}", a.sum());


        // Part 1
    let mut a: Array2<usize> = Array2::zeros((1000, 1000));

    for l in &v {
        let start: (usize, usize) = l[0].split(',').map(|d| d.parse::<usize>().unwrap()).collect_tuple().unwrap();
        let end: (usize, usize) = l[1].split(',').map(|d| d.parse::<usize>().unwrap()).collect_tuple().unwrap();

        println!("{:?} -> {:?}", start, end);

        if start.0 == end.0 {
            for i in min(start.1, end.1)..=max(start.1, end.1) {
                a[[i, start.0]] += 1;
            }
        }

        if start.1 == end.1 {
            for j in min(start.0, end.0)..=max(start.0, end.0) {
                a[[start.1, j]] += 1;
            }
        }
    }

    println!("{:?}", a);

    a.mapv_inplace(|d| {
        if d > 1 {
            1
        } else {
            0
        }
    });

    println!("Answer 1: {:?}", a.sum());


    // Part 2
    println!("Part 2");

    let mut a: Array2<usize> = Array2::zeros((1000, 1000));

    for l in &v {
        let start: (usize, usize) = l[0].split(',').map(|d| d.parse::<usize>().unwrap()).collect_tuple().unwrap();
        let end: (usize, usize) = l[1].split(',').map(|d| d.parse::<usize>().unwrap()).collect_tuple().unwrap();

        println!("{:?} -> {:?}", start, end);

        if start.0 == end.0 {
            for i in min(start.1, end.1)..=max(start.1, end.1) {
                a[[i, start.0]] += 1;
            }
        } else if start.1 == end.1 {
            for j in min(start.0, end.0)..=max(start.0, end.0) {
                a[[start.1, j]] += 1;
            }
        } else {
            // Diag
            println!("Diag: {:?}", l);

            for i in 0..=(max(start.0, end.0)-min(start.0, end.0)) {
                let (left, right) = match start.0 < end.0 {
                    true => (start, end),
                    false => (end, start),
                };
                let x = left.0 + i;

                let y = match right.1 > left.1 {
                    true => left.1 + i,
                    false => left.1 -i,
                };

                println!("Step: {} x {}", x, y);
                a[[y, x]] += 1;
            }
        }
    }

    println!("{:?}", a);

    a.mapv_inplace(|d| {
        if d > 1 {
            1
        } else {
            0
        }
    });

    println!("Answer 2: {:?}", a.sum());



}
