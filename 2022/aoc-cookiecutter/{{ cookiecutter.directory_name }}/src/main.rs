use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("File not readable");

    let v: Vec<_> = input.lines();
    println!("{:?}", v);

    // Part 1
    println!("Part 1");

    println!("Answer part 1: {:?}", "");


    // Part 2
    println!("Part 2");

    println!("Answer part 2: {}", "");
}
