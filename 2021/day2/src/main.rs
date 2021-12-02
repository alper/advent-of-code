use std::fs;
use itertools::Itertools;

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
        .map(|m| {
            match m[0] {
                "down" => m[1].parse::<i32>().unwrap(),
                "up" => -(m[1].parse::<i32>().unwrap()),
                _ => 0
            }
        }).collect();

    let down_delta = down_movements.iter().sum::<i32>();

    println!("Answer part 1: {:?}", forward_delta * down_delta);
}
