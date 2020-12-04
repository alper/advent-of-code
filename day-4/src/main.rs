use std::collections::HashSet;
use std::fs;
use std::iter::FromIterator;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Dead file");
    let blocks: Vec<&str> = input.split("\n\n").collect();

    let mut count = 0;

    for el in blocks {
        if !check(&el) {
            println!("Failed");
        } else {
            count += 1
        }
    }

    println!("Valid passports: {}", count);
}

fn check(block: &str) -> bool {
    let required_fields: HashSet<&str> =
        HashSet::from_iter(vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]);

    let present_fields: HashSet<&str> = HashSet::from_iter(
        block
            .split_whitespace()
            .map(|f| f.split(':').next().unwrap()),
    );

    let diff = required_fields
        .difference(&present_fields)
        .copied()
        .collect::<Vec<&str>>();

    diff.is_empty()
}
