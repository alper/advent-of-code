use std::fs::read_to_string;

use itertools::{Itertools, any};

fn main() {
    let sample_input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    let real_input = read_to_string("02-input.txt").unwrap();

    let input = real_input;

    let numbers = input
        .split(",")
        .map(|p| {
            let mut n = p.split("-");

            let low = n.next().unwrap().parse::<usize>().unwrap();
            let high = n.next().unwrap().parse::<usize>().unwrap();

            low..=high
        })
        .flat_map(|r| r.collect::<Vec<usize>>())
        .collect::<Vec<_>>();
    println!("{numbers:?}");

    let res = numbers
        .into_iter()
        .filter(|n| is_invalid(*n))
        .collect::<Vec<usize>>();

    // println!("res: {res:?}");

    let s = res.iter().sum::<usize>();

    println!("sum: {s}");
}

fn is_invalid(i: usize) -> bool {
    // println!("Input: {i}");

    let s: &str = &i.to_string();
    let mut d = divisors::get_divisors(s.len());

    d.insert(0, 1);

    // println!("D: {d:?}");

    let same = d
        .into_iter()
        .map(|d| {
            let parts = s
                .chars()
                .chunks(d)
                .into_iter()
                .map(|chunk| chunk.collect::<String>())
                .collect::<Vec<String>>();

            // println!("parts: {parts:?}");

            if parts.len() <= 1 {
                return false;
            }

            let first = &parts[0];
            parts.iter().all(|item| item == first)
        })
        .collect::<Vec<bool>>();

    same.into_iter().any(|b| b)
}
