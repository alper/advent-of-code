use std::collections::HashMap;
use std::collections::HashSet;
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

            // println!("Parts: {:?}", parts);

            let first_range = parse_range(parts[1]);
            let second_range = parse_range(parts[3]);

            // println!(
            //     "First range /{:?}/ second range /{:?}/",
            //     first_range, second_range
            // );

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
            // println!("Checking value: {}", val);

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

    let my_ticket: Vec<u64> = vec![
        149, 73, 71, 107, 113, 151, 223, 67, 163, 53, 173, 167, 109, 79, 191, 233, 83, 227, 229,
        157,
    ];
    valid_nearbys.push(my_ticket.clone());

    // println!("Valid number of tickets: {}", valid_nearbys.len());

    let ticket_fields: Vec<Vec<u64>> = (0..valid_nearbys[0].len())
        .map(|n| valid_nearbys.iter().map(move |v| v[n]).collect())
        .collect();

    // println!("Ticket fields: {:?}", ticket_fields);

    let mut possible_rules: Vec<HashSet<&str>> = Vec::new();

    for (i, field_vals) in ticket_fields.iter().enumerate() {
        // println!("Checking {}th values: {:?}", i, field_vals);

        let mut rules_for_field = HashSet::new();

        for (fname, franges) in ranges.iter() {
            if validate_values(&field_vals, *franges) {
                // println!("+: {}", fname);
                rules_for_field.insert(*fname);
            } else {
                // println!("X: {}", fname);
            }
        }

        possible_rules.push(rules_for_field);
    }

    println!(
        "possible rules: {:?}",
        possible_rules
            .iter()
            .map(|r| r.len())
            .collect::<Vec<usize>>()
    );

    let mut allocations: HashMap<usize, &str> = HashMap::new();
    loop {
        let single = possible_rules
            .iter()
            .enumerate()
            .filter(|(i, f)| f.len() == 1)
            .next()
            .unwrap();
        println!("Single: {:?}", single);

        let field_name = single.1.iter().next().unwrap().clone();
        allocations.insert(single.0, field_name);

        // remove from all other possible
        for i in 0..possible_rules.len() {
            {
                let s = &mut possible_rules[i];
                s.remove(field_name);
            }
        }

        if possible_rules.iter().all(|s| s.len() == 0) {
            break;
        }
    }

    println!("Allocations: {:?}", allocations);

    println!(
        "Answer 2? {}",
        allocations
            .iter()
            .filter(|(i, s)| s.starts_with("departure"))
            .map(|(i, s)| my_ticket[*i])
            .product::<u64>()
    );
}

fn parse_range(fragment: &str) -> (u64, u64) {
    let parts: Vec<_> = fragment.split('-').collect();

    (
        parts[0].parse::<u64>().unwrap(),
        parts[1].parse::<u64>().unwrap(),
    )
}

fn validate_values(values: &[u64], range: (u64, u64, u64, u64)) -> bool {
    let mut valid = true;

    for &value in values {
        if !((value >= range.0 && value <= range.1) || (value >= range.2 && value <= range.3)) {
            valid = false;
        }
    }

    valid
}
