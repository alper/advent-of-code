use std::fs;

use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("full_input.txt").expect("File not readable");

    // Part 1
    println!("Part 1");

    let elves = input.split("\n\n").map(|pack| pack.lines().map(|cal| cal.parse::<usize>().unwrap()).sum::<usize>());

    println!("Answer part 1: {:?}", elves.clone().max().unwrap());

    // Part 2

    let max_three = &elves.sorted().rev().collect::<Vec<_>>()[0..3];
    println!("Part 2");

    println!("Answer part 2: {:?}", max_three.iter().sum::<usize>());
}
