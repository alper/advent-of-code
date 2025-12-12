use std::{fs::read_to_string, ops::Index};

fn main() {
    let sample_input = "987654321111111
811111111111119
234234234234278
818181911112111";

    let real_input = read_to_string("input.txt").unwrap();

    let real_input = real_input;

    let j: Vec<usize> = real_input
        .split("\n")
        .map(|l| highest_joltage_1(l))
        .collect();

    // println!("{j:?}");

    let s: usize = j.iter().sum();

    println!("Result 1: {s}");

    let s2: usize = real_input
        .split("\n")
        .map(|bank| highest_joltage_2(bank))
        .sum();

    println!("Result 2: {s2}");
}

fn highest_joltage_1(bank: &str) -> usize {
    // println!("Bank: {bank}");

    let n: Vec<usize> = bank
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    let max = n.iter().max().unwrap();
    let max_pos = n.iter().position(|j| j == max).unwrap();

    // println!("Max {max} at {max_pos}");

    if max_pos == n.len() - 1 {
        // Search left of max
        let first_max = n[0..max_pos].into_iter().max().unwrap();

        // println!("First max: {first_max}");

        return first_max * 10 + max;
    } else {
        // Search right of max
        let second_max = n[max_pos + 1..].into_iter().max().unwrap();

        // println!("Second max: {second_max}");

        return max * 10 + second_max;
    }
}

fn highest_joltage_2(bank: &str) -> usize {
    let bank_numbers: Vec<usize> = bank
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    println!("Numbers: {:?}", bank_numbers);

    let mut digit_collector = Vec::<usize>::new();

    let mut remaining: usize = 12;
    let mut start: usize = 0;

    while remaining > 0 {
        println!("Start {}, remaining {}", start, remaining);

        let relevant_part = &bank_numbers[start..(bank.len() - (remaining - 1))];
        println!("Rel part: {:?}", relevant_part);

        let max_val = relevant_part.iter().max().unwrap();
        let max_pos = relevant_part.iter().position(|x| x == max_val).unwrap();
        println!("Max val {} pos {}", max_val, max_pos);

        digit_collector.push(*max_val);
        start += max_pos + 1;
        remaining -= 1;
    }

    println!("Result: {:?}", digit_collector);

    let res_str: String = digit_collector.iter().map(|&d| d.to_string()).collect();

    res_str.parse().unwrap()
}
