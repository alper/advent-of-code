use std::fs;

fn main() {
    let test_input = r"35
    20
    15
    25
    47
    40
    62
    55
    65
    95
    102
    117
    150
    182
    127
    219
    299
    277
    309
    576";

    let input = fs::read_to_string("input.txt").expect("Dead file");

    let parsed: Vec<u64> = input
        .lines()
        .map(|l| l.trim().parse::<u64>().unwrap())
        .collect();

    println!("Parsed: {:?}", parsed);

    let window_size = 25;
    let mut window_start = 0;

    let faulty_number = loop {
        let number_to_test = parsed[window_start + window_size];

        if !check_number(
            number_to_test,
            &parsed[window_start..window_start + window_size],
        ) {
            break number_to_test;
        }

        window_start += 1;
    };

    println!("The number {} is not correct", faulty_number);
}

fn check_number(num: u64, series: &[u64]) -> bool {
    assert!(series.len() >= 2);
    // println!("Checking {:?  }", series);

    let mut found = false;

    'outer: for i in 1..series.len() {
        for j in 0..i {
            // println!("{} + {}", series[i], series[j]);
            if series[i] + series[j] == num && series[i] != series[j] {
                found = true;
                break 'outer;
            }
        }
    }

    return found;
}

#[test]
fn test_check_number() {
    println!("In test");
    assert!(check_number(12, &[1, 2, 3, 4, 8]));

    assert!(!check_number(12, &[1, 2, 3, 4, 5]));
    assert!(!check_number(12, &[1, 2, 3, 4, 6, 6]));
}
