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

    let test_input_parsed: Vec<u64> = test_input
        .lines()
        .map(|l| l.trim().parse::<u64>().unwrap())
        .collect();

    let input = fs::read_to_string("input.txt").expect("Dead file");

    let input_parsed: Vec<u64> = input
        .lines()
        .map(|l| l.trim().parse::<u64>().unwrap())
        .collect();

    let faulty_number = find_faulty_number(25, &input_parsed);
    println!("The number {} is not correct", faulty_number);

    println!(
        "Contiguous set on test: {:?}",
        find_contiguous_set(127, &test_input_parsed)
    );

    let contiguous_set = find_contiguous_set(faulty_number, &input_parsed);
    println!("Contiguous set on real: {:?}", contiguous_set);
    println!(
        "Answer on part 2: {}",
        contiguous_set.iter().min().unwrap() + contiguous_set.iter().max().unwrap()
    );
}

fn find_faulty_number(window_size: usize, series: &[u64]) -> u64 {
    let mut window_start = 0;

    loop {
        let number_to_test = series[window_start + window_size];

        if !check_number(
            number_to_test,
            &series[window_start..window_start + window_size],
        ) {
            break number_to_test;
        }

        window_start += 1;
    }
}

fn find_contiguous_set(target: u64, series: &[u64]) -> &[u64] {
    let mut window_size = 3;

    loop {
        for r in series.windows(window_size) {
            if r.iter().sum::<u64>() == target {
                return r;
            }
        }

        window_size += 1;

        if window_size > series.len() {
            break;
        }
    }

    &[]
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

    found
}

#[test]
fn test_check_number() {
    println!("In test");
    assert!(check_number(12, &[1, 2, 3, 4, 8]));

    assert!(!check_number(12, &[1, 2, 3, 4, 5]));
    assert!(!check_number(12, &[1, 2, 3, 4, 6, 6]));
}
