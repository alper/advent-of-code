use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("File not readable");

    let mut template: Vec<char> = vec![];

    let mut rules: HashMap<String, char> = HashMap::new();

    for line in input.lines() {
        if !line.contains('>') {
            if !line.trim().is_empty() {
                template = line.trim().chars().collect();
            }
        } else {
            let mut parts = line.split(" -> ");
            let left = parts.next().unwrap().to_string();
            let right = parts.next().unwrap().chars().next().unwrap();

            rules.insert(left, right);
        }
    }

    println!("Template: {:?}", template);

    for i in 0..40 {
        println!("Run {}", i);

        let mut temp_vec: Vec<char> = vec![];

        for c in template.windows(2) {
            // println!("{:?}", c);

            let doublet = String::from_iter(c);
            let insertee = rules.get(&doublet).unwrap();
            // println!("Intermediary: {}", insertee);

            temp_vec.push(c[0]);
            temp_vec.push(*insertee);
        }

        temp_vec.push(*template.iter().last().unwrap());

        template = temp_vec;

        // println!("Template: {:?}", template);
    }

    let counts: Vec<usize> = template
        .iter()
        .unique()
        .map(|l| template.iter().filter(|c| *c == l).count())
        .collect();

    println!("Counts: {:?}", counts);

    println!(
        "Answer 1: {}",
        counts.iter().max().unwrap() - counts.iter().min().unwrap()
    );

    // template.chars().collect::<Vec<char>>().windows(2)

    // println!("Template: {:?}", template);
    // println!("Rules: {:?}", rules);
}
