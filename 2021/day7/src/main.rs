use std::fs;
use itertools::{Itertools};

fn main() {
    let input = fs::read_to_string("input.txt").expect("File not readable");

    let x_pos: Vec<_> = input.split(',').map(|s| s.parse::<isize>().unwrap()).collect();

    let min = x_pos.iter().min().unwrap().abs() as usize;
    let max = x_pos.iter().max().unwrap().abs() as usize;

    println!("Input: {:?}", x_pos);

    let s_deltas: Vec<isize> = (min..=max).map(|l| { x_pos.iter().map(|x| (x-(l as isize)).abs()).sum() }).collect();
    // let s_deltas: Vec<_> = (min..=max).map(|l| l).collect();

    println!("S Deltas: {:?}", s_deltas);

    println!("Answer 1: {}", s_deltas.iter().min().unwrap());

    // let s_deltas2: Vec<isize> = (min..=max)
    //     .map(|l| {
    //         x_pos
    //             .iter()
    //             .map(|x| (0..(x-(l as isize)).abs()).sum())
    //             .collect()
    //     })
    //     .collect();
    // println!("S Deltas2: {:?}", s_deltas2);
}
