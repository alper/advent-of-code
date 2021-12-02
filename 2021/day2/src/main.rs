use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("File not readable");

    let movements: Vec<Vec<&str>> = input.lines().map(|l| l.split(" ").collect()).collect();

    let forward_movements: Vec<i32> = movements
        .iter()
        .filter(|m| m[0] == "forward")
        .map(|m| m[1].parse::<i32>().unwrap())
        .collect();

    let forward_delta = forward_movements.iter().sum::<i32>();

    let down_movements: Vec<i32> = movements
        .iter()
        .map(|m| match m[0] {
            "down" => m[1].parse::<i32>().unwrap(),
            "up" => -(m[1].parse::<i32>().unwrap()),
            _ => 0,
        })
        .collect();

    let down_delta = down_movements.iter().sum::<i32>();

    println!("Answer part 1: {:?}", forward_delta * down_delta);

    // tuple: (down, aim, forward)
    let result = movements.iter().fold((0, 0, 0), |t, mov| {
        let amount = mov[1].parse::<i32>().unwrap();

        match mov[0] {
            "down" => (t.0, t.1 + amount, t.2),
            "up" => (t.0, t.1 - amount, t.2),
            "forward" => (t.0 + (t.1 * amount), t.1, t.2 + amount),
            _ => t,
        }
    });

    println!("Forward: {}, Down: {}", result.2, result.0);
    println!("Answer 2: {}", result.2 * result.0);
}
