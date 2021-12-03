use std::fs;
use grid::*;
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("input.txt").expect("File not readable");

    let v_v: Vec<Vec<u8>> = input.lines().map(|l| {l.chars().map(|c| {
        match c {
            '0' => 0,
            '1' => 1,
            _ => 0
        }
    }).collect::<Vec<u8>>() }).collect();

    // println!("Input: {:?}", v_v);

    let g = Grid::new_from(v_v);

    // let c = g.cols().next().unwrap().collect::<Vec<&u8>>();

    let gamma_array: Vec<usize> = g
        .cols()
        .map(|c| { c.filter(|&el| *el == 1 as u8).count() })
        .map(|count| {
            if count > g.rows/2 {
                return 1;
            } else {
                return 0;
            }})
        .collect();

    let gamma = isize::from_str_radix(&gamma_array.iter().join(""), 2).unwrap();

    let epsilon_array: Vec<usize> = gamma_array.iter().map(|d| {
        match d {
            1 => 0,
            0 => 1,
            _ => 0,
        }
    }).collect();

    let epsilon = isize::from_str_radix(&epsilon_array.iter().join(""), 2).unwrap();

    println!("Answer part 1: {:?}", gamma * epsilon);
}
