use std::fs;
use itertools::{Itertools};
use std::collections::HashMap;

fn main() {
    let input = fs::read_to_string("input.txt").expect("File not readable");

    let numbers: Vec<usize> = input.trim().split(',').map(|d| d.parse::<usize>().unwrap()).collect();

    let mut lanternfish = HashMap::new();

    for n in numbers {
        let count = lanternfish.entry(n).or_insert(0);

        *count += 1;
    }

    println!("Input: {:?}", lanternfish);

    for _ in 0..256 {
        let mut new_gen = HashMap::new();

        for (k, v) in lanternfish {
            if k == 0 {
                let c = new_gen.entry(6).or_insert(0);
                *c += v;

                let c2 = new_gen.entry(8).or_insert(0);
                *c2 += v;
            } else {
                let c = new_gen.entry(k-1).or_insert(0);
                *c += v
            }
        }

        lanternfish = new_gen;
    }

    let counts = lanternfish.iter().map(|(_, v)| *v).collect::<Vec<usize>>();
    println!("Input: {:?}", counts.iter().sum::<usize>());
}
