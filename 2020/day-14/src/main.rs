use std::fs;
use std::collections::HashMap;
use itertools::Itertools;

fn main() {
    part2();
}

fn part1() {
    let file_contents = fs::read_to_string("input.txt").expect("Dead file");

    let mut memory: HashMap<&str, String> = HashMap::new();
    let mut mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX";

    for line in file_contents.lines() {
        if line.starts_with("mask") {
            mask = &line[line.find('=').unwrap()+2..];

            println!("M: {}", mask);
        } else if line.starts_with("mem") {
            let address = &line[line.find('[').unwrap()+1..line.find(']').unwrap()];
            let value = &line[line.find('=').unwrap()+2..];

            println!("Address: {}", address);
            let binary_value = &format!("{:#038b}", value.parse::<u64>().unwrap())[2..];

            println!("B: {}", binary_value);

            let masked_value: String = mask.chars().zip(binary_value.chars()).map(|(m,v)| {
                if m == '0' || m == '1' {
                    return m;
                } else {
                    return v;
                }
            }).collect();

            println!("R: {}", masked_value);

            memory.insert(address, masked_value.clone());
        }
    }

    // Sum memory
    let mut sum = 0;
    for (_address, value) in memory {
        sum += u64::from_str_radix(&value, 2).unwrap()
    }

    println!("Part 1 result: {}", sum);
}

fn part2() {
    let file_contents = fs::read_to_string("input.txt").expect("Dead file");

    let mut memory: HashMap<String, u64> = HashMap::new();
    let mut mask = "000000000000000000000000000000000000";

    for line in file_contents.lines() {
        if line.starts_with("mask") {
            mask = &line[line.find('=').unwrap()+2..];

            println!("M: {}", mask);
        } else if line.starts_with("mem") {
            let value: u64 = line[line.find('=').unwrap()+2..].parse().unwrap();
            println!("Value: {}", value);

            let address = &line[line.find('[').unwrap()+1..line.find(']').unwrap()];
            let binary_address = &format!("{:#038b}", address.parse::<u64>().unwrap())[2..];
            println!("B: {}", binary_address);

            let masked_address: String = mask.chars().zip(binary_address.chars()).map(|(m,v)| {
                if m == '0' {
                    return v;
                } else if m == '1' {
                    return '1';
                } else if m == 'X' {
                    return 'X';
                }
                v
            }).collect();

            println!("R: {}", masked_address);

            // Cartesian product
            let target_to_vec: Vec<Vec<char>> = masked_address.chars().map(|c| {
                if c == 'X' {
                    return vec!['0', '1'];
                }
                return vec![c];
            }).collect();

            println!("CP: {:?}", target_to_vec);

            for target_address in target_to_vec.into_iter().map(IntoIterator::into_iter).multi_cartesian_product() {
                let address_value = target_address.iter().collect::<String>();

                memory.insert(address_value, value);
            }

        }
    }

    // Sum memory
    let mut sum = 0;
    for (_address, value) in memory {
        sum += value
    }

    println!("Part 2 result: {}", sum);
}