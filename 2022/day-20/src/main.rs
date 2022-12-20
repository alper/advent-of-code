use std::fs;

use itertools::Itertools;

fn print_numbers(v: &Vec<(isize, bool)>) -> String {
    v.iter()
        .map(|np| {
            if np.1 {
                format!("{}", np.0)
            } else {
                format!("({})", np.0)
            }
        })
        .join(", ")
}

fn do_part1(input: &str) {
    let mut numbers: Vec<_> = input
        .lines()
        .map(|l| (str::parse::<isize>(l).unwrap(), false))
        .collect();
    println!("{:?}", numbers);

    while let Some(pos) = numbers.iter().position(|p| p.1 == false) {
        // println!("Numbers: {:?}", print_numbers(&numbers));

        let i = numbers.remove(pos);
        // println!("Got {:?}", i);
        // println!("Numbers after remove: {:?}", print_numbers(&numbers));

        let new_index = ((pos as isize) + i.0).rem_euclid(numbers.len() as isize);

        // println!("New index: {}", new_index);

        numbers.insert(new_index as usize, (i.0, true));
        // println!("Numbers after insert: {:?}", print_numbers(&numbers));
        // println!();
    }

    println!("Final: {:?}", numbers);

    let zero_index = numbers.iter().position(|np| np.0 == 0).unwrap();
    let length = numbers.len();

    numbers.rotate_left(1000 % length);
    let first = numbers[zero_index].0;
    println!("1000th number after zero is: {:?}", first);

    numbers.rotate_left(1000 % length);
    let second = numbers[zero_index].0;
    println!("2000th number after zero is: {:?}", second);

    numbers.rotate_left(1000 % length);
    let third = numbers[zero_index].0;
    println!("3000th number after zero is: {:?}", third);

    println!("Answer part 1: {:?}", first + second + third);
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let input = fs::read_to_string("full_input.txt").expect("File not readable");

    do_part1(&input);

    // Part 2
    println!("Part 2");

    let mut numbers: Vec<_> = input
        .lines()
        .enumerate()
        .map(|(i, l)| (str::parse::<isize>(l).unwrap() * 811589153, i))
        .collect();
    println!("{:?}", numbers);

    println!("Numbers: {}", numbers.iter().map(|np| np.0).join(", "));

    for i in 0..10 {
        let mut current_to_move = 0;

        while let Some(pos) = numbers.iter().position(|p| p.1 == current_to_move) {
            // println!("Numbers: {:?}", print_numbers(&numbers));

            let i = numbers.remove(pos);
            // println!("Got {:?}", i);
            // println!("Numbers after remove: {:?}", print_numbers(&numbers));

            let new_index = ((pos as isize) + i.0).rem_euclid(numbers.len() as isize);

            // println!("New index: {}", new_index);

            numbers.insert(new_index as usize, (i.0, i.1));
            // println!("Numbers after insert: {:?}", print_numbers(&numbers));
            // println!();

            current_to_move += 1;
        }

        // println!("Numbers1: {}", numbers.iter().map(|np| np.0).join(", "));
    }

    println!("Numbersdone: {}", numbers.iter().map(|np| np.0).join(", "));

    let zero_index = numbers.iter().position(|np| np.0 == 0).unwrap();
    let length = numbers.len();

    numbers.rotate_left(1000 % length);
    let first = numbers[zero_index].0;
    println!("1000th number after zero is: {:?}", first);

    numbers.rotate_left(1000 % length);
    let second = numbers[zero_index].0;
    println!("2000th number after zero is: {:?}", second);

    numbers.rotate_left(1000 % length);
    let third = numbers[zero_index].0;
    println!("3000th number after zero is: {:?}", third);

    println!("Answer part 2: {:?}", first + second + third);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
