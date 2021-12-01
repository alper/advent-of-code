use std::fs;
use std::collections::HashSet;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Dead file");

    let first_result: u32 = input.split("\n\n").map(|group| unique_letter_count(group)).sum();
    println!("First result: {}", first_result);

    let second_result: u32 = input.split("\n\n").map(|group| all_letter_count(group)).sum();
    println!("Second result: {}", second_result);
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

fn all_letter_count(group: &str) -> u32 {
    let lines: Vec<&str> = group.lines().collect();
    assert!(lines.len() > 0);

    let first_line = lines[0];
    let mut count = 0;

    for c in first_line.chars() {
        let mut missing = false;

        for line in &lines { // We can iterate over all lines including the first one because by extension the first one too contains all the letters
            if !line.contains(c) {
                missing = true;
            }
        }

        if !missing {
            count += 1;
        }
    }

    count
}