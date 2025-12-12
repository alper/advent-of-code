use std::{cmp::max, fs::read_to_string, ops::Range};

use itertools::{Itertools, merge};

fn main() {
    let sample_input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
    let real_input = read_to_string("input.txt").unwrap();
    let real_input = real_input;

    let mut ranges: Vec<(usize, usize)> = Vec::new();
    let mut ids: Vec<usize> = Vec::new();

    let mut parsing_ranges = true;

    for line in real_input.split("\n") {
        println!("{}", line);

        if line == "" {
            parsing_ranges = false;
        } else {
            if parsing_ranges {
                let mut parts = line.split("-");
                let first = parts.next().unwrap().parse::<usize>().unwrap();
                let second = parts.next().unwrap().parse::<usize>().unwrap();

                ranges.push((first, second));

                // let range = line
                //     .split("-")
                //     .map(|p| p.parse::<usize>().unwrap())
                //     .tuples::<(usize, usize)>()
                //     .collect();

                // ranges.push(range);
            } else {
                ids.push(line.parse::<usize>().unwrap());
            }
        }
    }

    let fresh = ids
        .iter()
        .filter(|&n| ranges.iter().any(|r| *n >= r.0 && *n <= r.1));

    println!("Result 1: {}", fresh.count());

    ranges.sort();

    println!("Ranges: {:?}", ranges);

    let result: usize = ranges
        .into_iter()
        .coalesce(|x, y| {
            if x.1 >= y.0 {
                return Ok((x.0, max(x.1, y.1)));
            } else {
                return Err((x, y));
            }
        })
        .map(|r| (r.1 - r.0) + 1)
        .sum();

    println!("Res2: {}", result);
}
