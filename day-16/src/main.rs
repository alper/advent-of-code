use std::fs;

fn main() {
    let file_contents = fs::read_to_string("input.txt").expect("Dead file");

    let mut nearby_tickets = false;
    let mut nearbys: Vec<Vec<u64>> = vec![];
    let mut ranges: Vec<(u64, u64)> = vec![];

    let mut error_sum = 0;

    for line in file_contents.lines() {
        if line.contains("or") {
            let parts: Vec<_> = line.split(" ").collect();

            println!("Parts: {:?}", parts);

            let first_range = parts[parts.len() - 3];
            let second_range = parts[parts.len() - 1];

            // println!(
            //     "First range /{:?}/ second range /{:?}/",
            //     parse_range(first_range),
            //     parse_range(second_range)
            // );

            ranges.push(parse_range(first_range));
            ranges.push(parse_range(second_range));
        }

        if line.contains("nearby tickets:") {
            nearby_tickets = true;
        } else if nearby_tickets {
            nearbys.push(
                line.trim()
                    .split(',')
                    .map(|n| n.parse::<u64>().unwrap())
                    .collect(),
            );
        }
    }

    for ticket in nearbys {
        for val in ticket {
            println!("Checking value: {}", val);

            let mut valid = false;

            for range in ranges.iter() {
                if val >= range.0 && val <= range.1 {
                    valid = true;

                    break;
                }
            }

            if !valid {
                error_sum += val;
            }
        }
    }

    println!("Answer to part 1: {}", error_sum);
}

fn parse_range(fragment: &str) -> (u64, u64) {
    let parts: Vec<_> = fragment.split('-').collect();

    (
        parts[0].parse::<u64>().unwrap(),
        parts[1].parse::<u64>().unwrap(),
    )
}
