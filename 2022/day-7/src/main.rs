use itertools::Itertools;
use std::{collections::VecDeque, fs};

const TOTAL_SPACE: usize = 70000000;
const TARGET_SPACE: usize = 30000000;

fn main() {
    let input = fs::read_to_string("full_input.txt").expect("File not readable");

    let mut winners: Vec<(&str, usize)> = vec![];
    let mut sizes: Vec<usize> = vec![];

    let mut stack: VecDeque<(&str, usize)> = VecDeque::new();

    for l in input.lines() {
        println!("Line: {l}");

        if l.trim() == "$ cd /" {
            stack.push_back(("/", 0));
        } else if l.trim() == "$ cd .." {
            let (dir_name, dir_size) = stack.pop_back().unwrap();

            if dir_size <= 100_000 {
                winners.push((dir_name, dir_size));
            }

            sizes.push(dir_size);
        } else if l.trim() == "$ ls" {
        } else if l.starts_with("dir ") {
            let (_, folder_name) = l.split_at(4);

            // println!("{folder_name}");
        } else if l.starts_with("$ cd ") {
            let (_, folder_name) = l.split_at(5);

            stack.push_back((folder_name, 0))

            // println!("{folder_name}");
        } else {
            let (file_size, file_name) = l.split(" ").collect_tuple().unwrap();

            let file_size_parsed = str::parse::<usize>(file_size).unwrap();

            println!("File {file_name}: {file_size}");

            for (name, size) in stack.iter_mut() {
                *size += file_size_parsed;
            }
        }
        println!("Stack: {:?}", stack);
    }

    // Find root size
    let total_occupied_size = stack.front().unwrap().1;

    // Catch any stragglers
    for (dir_name, dir_size) in stack.iter() {
        if *dir_size <= 100_000 {
            winners.push((dir_name, *dir_size));
        }

        sizes.push(*dir_size);
    }

    // let v: Vec<_> = input.lines().collect();
    println!("Part 1: {:?}", winners.iter().map(|el| el.1).sum::<usize>());

    // Part 2
    println!("Part 2");

    let unused_space = TOTAL_SPACE - total_occupied_size;
    println!("Unused space: {unused_space}");

    let space_to_free_up = TARGET_SPACE - unused_space;
    println!("Space to free: {space_to_free_up}");

    println!("Sizes: {:?}", sizes);

    println!(
        "Answer part 2: {:?}",
        sizes
            .iter()
            .filter(|s| **s >= space_to_free_up)
            .sorted()
            .rev()
            .last()
            .unwrap()
    );
}
