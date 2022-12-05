extern crate nom;

use itertools::Itertools;
use nom::{
    bytes::complete::tag, character::complete::digit1, combinator::map_res, sequence::tuple,
    IResult,
};
use std::{collections::VecDeque, fs};

fn get_test_start_state() -> Vec<VecDeque<char>> {
    vec![
        VecDeque::from(vec!['N', 'Z']),
        VecDeque::from(vec!['D', 'C', 'M']),
        VecDeque::from(vec!['P']),
    ]
}

fn get_full_start_state() -> Vec<VecDeque<char>> {
    vec![
        VecDeque::from(vec!['M', 'F', 'C', 'W', 'T', 'D', 'L', 'B']),
        VecDeque::from(vec!['L', 'B', 'N']),
        VecDeque::from(vec!['V', 'L', 'T', 'H', 'C', 'J']),
        VecDeque::from(vec!['W', 'J', 'P', 'S']),
        VecDeque::from(vec!['R', 'L', 'T', 'F', 'C', 'S', 'Z']),
        VecDeque::from(vec!['Z', 'N', 'H', 'B', 'G', 'D', 'W']),
        VecDeque::from(vec!['N', 'C', 'G', 'V', 'P', 'S', 'M', 'F']),
        VecDeque::from(vec!['Z', 'C', 'V', 'F', 'J', 'R', 'Q', 'W']),
        VecDeque::from(vec!['H', 'L', 'M', 'P', 'R']),
    ]
}

fn parse_move_line(input: &str) -> IResult<&str, (u8, u8, u8)> {
    let (input, (_, amount, _, from, _, to)) = tuple((
        tag("move "),
        map_res(digit1, str::parse),
        tag(" from "),
        map_res(digit1, str::parse),
        tag(" to "),
        map_res(digit1, str::parse),
    ))(input)?;

    Ok((input, (amount, from, to)))
}

fn main() {
    let input = fs::read_to_string("full_input.txt").expect("File not readable");

    // Part 1
    println!("Part 1");

    let moves_text = input
        .split("\n\n")
        .collect_tuple::<(&str, &str)>()
        .unwrap()
        .1;

    let moves: Vec<_> = moves_text
        .lines()
        .map(|l| parse_move_line(l).unwrap().1)
        .collect();
    println!("Moves: {:?}", moves);

    let mut state: Vec<VecDeque<char>> = get_full_start_state();
    println!("State: {:?}", state);

    for (amount, from, to) in moves.clone() {
        println!("Moving {amount} from {from} to {to}");

        for _ in 0..amount {
            let el = state
                .get_mut((from - 1) as usize)
                .unwrap()
                .pop_front()
                .unwrap();

            state.get_mut((to - 1) as usize).unwrap().push_front(el);
        }

        println!("State after: {:?}", state);
        println!("");
    }

    println!(
        "Answer part 1: {:?}",
        state.iter().map(|vd| vd[0]).collect::<String>()
    );

    // Part 2
    println!("Part 2");

    let mut state2 = get_full_start_state();

    for (amount, from, to) in moves.clone() {
        println!("Moving {amount} from {from} to {to}");

        let mut to_move: Vec<char> = vec![];

        for _ in 0..amount {
            to_move.push(
                state2
                    .get_mut((from - 1) as usize)
                    .unwrap()
                    .pop_front()
                    .unwrap(),
            );
        }

        for _ in 0..amount {
            state2
                .get_mut((to - 1) as usize)
                .unwrap()
                .push_front(to_move.pop().unwrap());
        }

        println!("State after: {:?}", state);
        println!("");
    }

    println!(
        "Answer part 2: {:?}",
        state2.iter().map(|vd| vd[0]).collect::<String>()
    );
}
