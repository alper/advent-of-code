use std::{collections::HashSet, fs};

fn signal_start(input: &str, distinct: usize) -> usize {
    let c = input.chars().collect::<Vec<char>>();

    let w = c.windows(distinct);

    let mut counter = 0;
    for potential in w {
        let mut uniq = HashSet::new();
        if potential.into_iter().all(move |x| uniq.insert(x)) {
            println!("Found: {}", potential.iter().collect::<String>());
            break;
        }

        counter += 1;
    }

    counter + distinct
}

fn main() {
    let test_input = fs::read_to_string("test_input.txt").expect("File not readable");

    let v = test_input.lines().map(|l| signal_start(l, 4));
    println!("{:?}", v.collect::<Vec<usize>>());

    // Part 1
    println!("Part 1");

    let full_input = fs::read_to_string("full_input.txt").expect("File not readable");

    println!("Answer part 1: {:?}", signal_start(&full_input, 4));

    // Part 2
    println!("Part 2");

    let v = test_input.lines().map(|l| signal_start(l, 14));
    println!("{:?}", v.collect::<Vec<usize>>());

    println!("Answer part 2: {:?}", signal_start(&full_input, 14));
}
