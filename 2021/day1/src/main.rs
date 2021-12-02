use std::fs;
use itertools::{Itertools}; // TupleWindows

fn main() {
    let input = fs::read_to_string("input.txt").expect("File not readable");

    let depth_measurements: Vec<i32> = input.lines().map(|s|
        s.trim().parse::<i32>().unwrap()).collect();

    // println!("{:?}", depth_measurements);

    let simple_differences = depth_measurements
        .iter()
        .tuple_windows()
        .map(|(a, b)| (b - a))
        .filter(|a| *a > 0);

    println!("Answer to part 1: {:?}", simple_differences.count());

    // Part 2
    let measurement_triplets: Vec<(&i32, &i32, &i32)> = depth_measurements
        .iter()
        .tuple_windows()
        .collect();

    let triplet_differences = measurement_triplets
        .iter()
        .map(|&(a, b, c)| a + b + c)
        .tuple_windows()
        .filter(|(a, b)| b > a);

    println!("Answer to part 2: {:?}", triplet_differences.count());
}
