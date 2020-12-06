use std::fs;
use std::collections::HashSet;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Dead file");

    let first_result: u32 = input.split("\n\n").map(|group| unique_letter_count(group)).sum();
    println!("First result: {}", first_result);
}

fn unique_letter_count(group: &str) -> u32 {
    let mut letters = HashSet::new();

    for l in group.trim().chars() {
        if !l.is_whitespace() {
            letters.insert(l);
        }
    }

    letters.len() as u32
}