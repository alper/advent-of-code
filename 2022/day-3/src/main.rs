use std::fs;

use itertools::Itertools;

fn char_to_value(c: char) -> usize {
    let c_val: u32 = c.into();

    if c.is_ascii_lowercase() {
        let a_val: u32 = 'a'.into();

        return (c_val - a_val + 1).try_into().unwrap();
    }

    if c.is_ascii_uppercase() {
        let A_val: u32 = 'A'.into();
        println!("{c}, {c_val}, {A_val}");

        return (c_val - A_val + 27).try_into().unwrap();
    }

    0
}

fn main() {
    let input = fs::read_to_string("full_input.txt").expect("File not readable");

    // Part 1
    println!("Part 1");

    let splits: Vec<(&str, &str)> = input.lines().map(|l| l.split_at(l.len() / 2)).collect();

    let items_in_both: Vec<usize> = splits
        .iter()
        .map(|s| {
            s.0.chars()
                .filter(|c| s.1.contains(*c))
                .unique()
                .collect::<Vec<char>>()
                .first()
                .unwrap()
                .clone()
        })
        .map(|c| char_to_value(c))
        .collect();

    println!("Answer part 1: {:?}", items_in_both.iter().sum::<usize>());

    // Part 2
    println!("Part 2");

    let groups: Vec<Vec<_>> = input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|chunk| chunk.collect())
        .collect();

    let badges: Vec<_> = groups
        .iter()
        .map(|g| {
            g[0].chars()
                .filter(|c| g[1].contains(*c) && g[2].contains(*c))
                .unique()
                .collect::<Vec<_>>()
                .first()
                .unwrap()
                .clone()
        })
        .map(|c| char_to_value(c))
        .collect();

    println!("Answer part 2: {:?}", badges.iter().sum::<usize>());
}
