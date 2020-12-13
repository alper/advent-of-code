use std::collections::HashMap;
use std::fs;

fn main() {
    part2();
}

fn part1() {
    let mut _test_input = r"939
    7,13,x,x,59,x,31,19".lines();

    let file_contents = fs::read_to_string("input.txt").expect("Dead file");
    let mut real_input = file_contents.lines();

    let departure_time = real_input.next().unwrap().trim().parse::<u32>().unwrap();
    println!("Departure time: {}", departure_time);

    let buses: Vec<u32> = real_input.next().unwrap().trim().split(',').filter_map(|b| b.parse().ok()).collect();
    println!("Buses: {:?}", buses);

    let mut waiting_times = HashMap::new();

    for bus in buses {
        waiting_times.insert(bus, ((departure_time / bus) + 1) * bus);
    }

    println!("Waiting times: {:?}", waiting_times);

    let bus = waiting_times.iter().min_by(|x, y| x.1.cmp(y.1)).unwrap();
    println!("Bus: {:?}", bus);

    let wait = bus.1 - departure_time;
    println!("Wait: {:?}", wait);

    println!("Answer: {:?}", wait * bus.0);
}

// Tak: https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust
fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}

fn part2() {
    let mut _test_input = r"X
    2,3,x,7".lines();

    let file_contents = fs::read_to_string("input.txt").expect("Dead file");
    let mut _test_input = file_contents.lines();

    let _ = _test_input.next(); // Don't care about the first line

    let buses: Vec<(i64, i64)> = _test_input.next().unwrap().trim().split(',').map(|s| s.parse().ok()).enumerate().filter_map(|b| {
        match b.1 {
            Some(n) => Some((b.0 as i64, n)),
            _ => None
        }
    }).collect();
    println!("Buses: {:?}", buses);

    let moduli: Vec<_> = buses.iter().map(|&(_, b)| b).collect();
    // No idea why this has to be (b-1) but fuck it.
    let remainders: Vec<_> = buses.iter().map(|&(i, b)| (b - i)).collect();
    println!("Remainders: {:?}", remainders);
    println!("Moduli: {:?}", moduli);

    println!("Solution: {:?}", chinese_remainder(&remainders, &moduli));
}