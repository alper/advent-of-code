use std::fs;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let input = fs::read_to_string("test_input.txt").expect("File not readable");

    let v: Vec<_> = input.lines().collect();
    println!("{:?}", v);

    // Part 1
    println!("Part 1");

    println!("Answer part 1: {:?}", "");

    // Part 2
    println!("Part 2");

    println!("Answer part 2: {:?}", "");

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
