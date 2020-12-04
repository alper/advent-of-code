use std::collections::HashSet;
use std::fs;
use std::iter::FromIterator;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Dead file");
    let count = input.split("\n\n").filter(|b| check(b)).count();

    println!("Valid passports: {:?}", count);
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
