use std::fs;
use itertools::{Itertools};

fn main() {
    let input = fs::read_to_string("input.txt").expect("File not readable");

    let output_digits: Vec<_> = input.lines().map(|l| l.split('|').nth(1).unwrap().trim()).collect();

    let simple_digits: Vec<usize> = output_digits
        .iter()
        .map(|ds|
            ds.split(' ')
                .filter(|d| matches!(d.len(), 2 | 3 | 4 | 7)).count()
                )
        .collect();

    println!("Input: {:?}", simple_digits.iter().sum::<usize>());
}
