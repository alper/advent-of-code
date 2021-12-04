use std::fs;
use grid::*;
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("input.txt").expect("File not readable");

    let test_input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    let v_v: Vec<Vec<u8>> = input.lines().map(|l| {l.chars().map(|c| {
        match c {
            '0' => 0,
            '1' => 1,
            _ => 0
        }
    }).collect::<Vec<u8>>() }).collect();

    let g = Grid::new_from(v_v);

    let gamma_array: Vec<usize> = g
        .cols()
        .map(|c| { c.filter(|&el| *el == 1 as u8).count() })
        .map(|count| {
            if count > g.rows/2 {
                return 1;
            } else {
                return 0;
            }})
        .collect();

    let gamma = isize::from_str_radix(&gamma_array.iter().join(""), 2).unwrap();

    let epsilon_array: Vec<usize> = gamma_array.iter().map(|d| {
        match d {
            1 => 0,
            0 => 1,
            _ => 0,
        }
    }).collect();

    let epsilon = isize::from_str_radix(&epsilon_array.iter().join(""), 2).unwrap();

    println!("Answer part 1: {:?}", gamma * epsilon);

    let mut oxygen_ratings: Vec<&str> = input.lines().collect();
    let mut oxygen_bit_index: usize = 0;

    loop {
        println!("Debug len ratings: {:?}", oxygen_ratings.len());

        let ones: Vec<char> = oxygen_ratings.iter().map(|l| l.chars().nth(oxygen_bit_index).unwrap()).filter(|&c| c == '1').collect();
        let number_of_1s = ones.len();
        println!("Number of 1s: {:?}", number_of_1s);


        let mut keep = '1';
        if number_of_1s >= oxygen_ratings.len() - number_of_1s {
            keep = '1';
        } else if number_of_1s < oxygen_ratings.len() - number_of_1s {
            keep = '0';
        }

        println!("Keep: {}", keep);

        oxygen_ratings.retain(|l| l.chars().nth(oxygen_bit_index).unwrap() == keep);

        if oxygen_ratings.len() <= 1 {
            break;
        }
        oxygen_bit_index += 1;
    }

    println!("Debug: {:?}", oxygen_ratings);


    let mut co2_ratings: Vec<&str> = input.lines().collect();
    let mut co2_bit_index: usize = 0;

    loop {
        println!("Debug: {:?}", co2_ratings.len());

        let ones: Vec<char> = co2_ratings.iter().map(|l| l.chars().nth(co2_bit_index).unwrap()).filter(|&c| c == '1').collect();
        let number_of_1s = ones.len();

        let mut keep = '0';
        if number_of_1s >= co2_ratings.len() - number_of_1s {
            keep = '0';
        } else if number_of_1s <= co2_ratings.len() - number_of_1s {
            keep = '1';
        }

        co2_ratings.retain(|l| l.chars().nth(co2_bit_index).unwrap() == keep);

        if co2_ratings.len() <= 1 {
            break;
        }
        co2_bit_index += 1;
    }

    println!("Debug: {:?}", co2_ratings);

    println!("Answer: {}", isize::from_str_radix(oxygen_ratings[0], 2).unwrap() * isize::from_str_radix(co2_ratings[0], 2).unwrap());
}
