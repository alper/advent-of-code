use std::collections::HashMap;
use std::fs;

fn main() {
    let file_contents = fs::read_to_string("input.txt").expect("Dead file");

    let mut nearby_tickets = false;
    let mut nearbys: Vec<Vec<u64>> = vec![];
    let mut valid_nearbys: Vec<Vec<u64>> = vec![];

    let mut ranges: HashMap<&str, (u64, u64, u64, u64)> = HashMap::new();

    let mut error_sum = 0;

    for line in file_contents.lines() {
        if line.contains("or") {
            let previous_parts: Vec<_> = line.split(':').collect();
            let field_name = previous_parts[0];

            let parts: Vec<_> = previous_parts[1].split(' ').collect();

            println!("Parts: {:?}", parts);

            let first_range = parse_range(parts[1]);
            let second_range = parse_range(parts[3]);

            println!(
                "First range /{:?}/ second range /{:?}/",
                first_range, second_range
            );

            ranges.insert(
                field_name,
                (first_range.0, first_range.1, second_range.0, second_range.1),
            );
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
        let mut entire_ticket_valid = true;

        for val in &ticket {
            println!("Checking value: {}", val);

            let mut valid = false;

            for range in ranges.values() {
                if val >= &range.0 && val <= &range.1 {
                    valid = true;

                    break;
                }

                if val >= &range.2 && val <= &range.3 {
                    valid = true;

                    break;
                }
            }

            if !valid {
                error_sum += val;
                entire_ticket_valid = false;
            }
        }

        if entire_ticket_valid {
            valid_nearbys.push(ticket);
        }
    }

    println!("Answer to part 1: {}", error_sum);

    println!("Valid number of tickets: {}", valid_nearbys.len());
}

fn parse_range(fragment: &str) -> (u64, u64) {
    let parts: Vec<_> = fragment.split('-').collect();

    (
        parts[0].parse::<u64>().unwrap(),
        parts[1].parse::<u64>().unwrap(),
    )
}
