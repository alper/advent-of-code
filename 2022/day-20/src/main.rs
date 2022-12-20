use cdll::{list, CircularList};
use std::fs;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let input = fs::read_to_string("test_input.txt").expect("File not readable");

    let mut numbers: CircularList<_> = input
        .lines()
        .map(|l| (str::parse::<isize>(l).unwrap(), false))
        .collect();
    println!("{:?}", numbers);

    println!("{:?}", numbers.cursor().unwrap().value());

    while numbers.iter().any(|n| n.1 == false) {
        println!("Numbers: {:?}", numbers);
        // Rotate left until we are good
        while numbers.cursor().unwrap().value().1 != false {
            numbers.left_rot(1);
        }

        println!("Nrot {:?}", numbers);

        let mut n = numbers.remove().unwrap();
        n.1 = true;
        let mut new_list = list![n];

        println!("Newl {:?}", new_list);

        if n.0 > 0 {
            numbers.left_rot(n.0 as usize)
        } else if n.0 < 0 {
            let n_abs = n.0.abs() as usize;
            numbers.right_rot(n_abs);
        }

        println!("Nrot {:?}", numbers);
        println!();

        new_list.append(numbers);
        numbers = new_list;
    }

    println!("Final: {:?}", numbers);

    // Part 1
    println!("Part 1");

    println!("Answer part 1: {:?}", "");

    // Part 2
    println!("Part 2");

    println!("Answer part 2: {:?}", "");

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
