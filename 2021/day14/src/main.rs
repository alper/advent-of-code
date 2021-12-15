use itertools::Itertools;
use std::collections::HashMap;
use std::fs;
use std::slice::SliceIndex;

fn main() {
    let input = fs::read_to_string("input.txt").expect("File not readable");

    let mut pair_freqs: HashMap<[char; 2], usize> = HashMap::new();
    let mut first_char = ' ';
    let mut last_char = ' ';

    let mut rules: HashMap<[char; 2], char> = HashMap::new();

    for line in input.lines() {
        if !line.contains('>') {
            if !line.trim().is_empty() {
                for pair in line.trim().chars().tuple_windows::<(_, _)>() {
                    let e = pair_freqs.entry([pair.0, pair.1]).or_insert(0);

                    *e += 1;
                }

                // Get first and last
                first_char = line.chars().next().unwrap();
                last_char = line.chars().last().unwrap();
            }
        } else {
            let mut parts = line.split(" -> ");
            let left_couple = parts.next().unwrap();
            let left = [
                left_couple.chars().next().unwrap(),
                left_couple.chars().nth(1).unwrap(),
            ];
            let right = parts.next().unwrap().chars().next().unwrap();

            rules.insert(left, right);
        }
    }

    println!("Template: {:?}", pair_freqs);

    println!("Rules: {:?}", rules);

    for i in 0..40 {
        println!("Run: {}", i);
        let mut new_freqs: HashMap<[char; 2], usize> = HashMap::new();

        for (pair, f) in &pair_freqs {
            let gen = rules.get(pair).unwrap();

            let e1 = new_freqs.entry([pair[0], *gen]).or_insert(0);
            *e1 += *f;

            let e2 = new_freqs.entry([*gen, pair[1]]).or_insert(0);
            *e2 += *f;
        }

        pair_freqs = new_freqs;
        println!("New freqs: {:?}", pair_freqs);
    }

    let mut counts: HashMap<char, usize> = HashMap::new();

    for (pair, f) in &pair_freqs {
        let c1 = counts.entry(pair[0]).or_insert(0);
        *c1 += *f;

        let c2 = counts.entry(pair[1]).or_insert(0);
        *c2 += *f;
    }

    // Correct the counts
    *counts.get_mut(&first_char).unwrap() += 2;
    *counts.get_mut(&last_char).unwrap() += 2;

    // for (c, f) in &counts {
    //     println!("{} x {}", c, f / 2);
    // }

    let answer = (counts.values().max().unwrap() / 2) - (counts.values().min().unwrap() / 2);
    println!("Answer = {}", answer);
}
