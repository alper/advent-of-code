use std::fs;
use std::cmp::{min, max};
use itertools::{Itertools};
use ndarray::{Array2, s};

type Point = (usize, usize);

fn main() {
    let input = fs::read_to_string("input.txt").expect("File not readable");

    let v: Vec<Vec<&str>> = input.lines().map(|l| l.split(" -> ").collect()).collect();

    println!("Input: {:?}", v);

    let mut a: Array2<usize> = Array2::zeros((1000, 1000));

    // println!("{:?}", a);

    // a[[1,2]] += 1;
    // a[[10, 10]] += 1;



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

    println!("Answer: {:?}", a.sum());


}
