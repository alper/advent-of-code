extern crate nom;

use std::{fs, cmp::Ordering, slice::Windows, collections::binary_heap::Drain};
use nom::{IResult, error::{self, ErrorKind}, complete::take, sequence::tuple, bytes::streaming::take_while, character::{is_alphabetic, complete::{alpha1, space1}, is_space}};
use toodee::DrainRow;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Play {
    Rock,
    Paper,
    Scissors
}

impl Play {
    fn points(&self) -> usize {
        match &self {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scissors => 3
        }
    }
}

fn round_result(me: Play, opponent: Play) -> usize {
    match me.cmp(&opponent) {
        Ordering::Less => 0,
        Ordering::Equal => 3,
        Ordering::Greater => 6
    }
}

fn round_move(opponent: Play, res: Outcome) -> Play {
    match res {
        Outcome::Loss => match opponent {
                Play::Rock => Play::Scissors,
                Play::Paper => Play::Rock,
                Play::Scissors => Play::Paper,
            }
        Outcome::Draw => opponent.clone(),
        Outcome::Win => match opponent {
            Play::Rock => Play::Paper,
            Play::Paper => Play::Scissors,
            Play::Scissors => Play::Rock
        }
    }
}

impl Ord for Play {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self == other {
            return Ordering::Equal;
        } else if *self == Play::Paper && *other == Play::Scissors ||
        *self == Play::Rock && *other == Play::Paper ||
        *self == Play::Scissors && *other == Play::Rock {
            return Ordering::Less;
        } else if *self == Play::Rock && *other == Play::Scissors ||
        *self == Play::Paper && *other == Play::Rock ||
        *self == Play::Scissors && *other == Play::Paper {
            return Ordering::Greater
        }

        return Ordering::Equal;
    }
}

impl PartialOrd for Play {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Copy, Clone)]
enum Outcome {
    Loss,
    Draw,
    Win
}

impl Outcome {
    fn points(self) -> usize {
        match self {
            Outcome::Loss => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6
        }
    }
}

fn parse_move(input: &str) -> IResult<&str, Play> {
    let (input, m) = alpha1(input)?;

    match m {
        "A" => Ok((input, Play::Rock)),
        "B" => Ok((input, Play::Paper)),
        "C" => Ok((input, Play::Scissors)),
        "X" => Ok((input, Play::Rock)),
        "Y" => Ok((input, Play::Paper)),
        "Z" => Ok((input, Play::Scissors)),
        _ => unimplemented!()
    }
}

fn parse_outcome(input: &str) -> IResult<&str, Outcome> {
    let (input, o) = alpha1(input)?;

    match o {
        "X" => Ok((input, Outcome::Loss)),
        "Y" => Ok((input, Outcome::Draw)),
        "Z" => Ok((input, Outcome::Win)),
        _ => unimplemented!()
    }
}

fn rps_move(input: &str) -> IResult<&str, (Play, Play)> {
    let (input, (move1, _, move2)) = tuple((parse_move, space1, parse_move))(input)?;

    Ok((input, (move1, move2)))
}

fn rps_move_part2(input: &str) -> IResult<&str, (Play, Outcome)> {
    let (input, (move1, _, outcome)) = tuple((parse_move, space1, parse_outcome))(input)?;

    Ok((input, (move1, outcome)))
}

fn main() {
    let input = fs::read_to_string("full_input.txt").expect("File not readable");

    // Part 1
    println!("Part 1");

    let parsed = input.lines().map(|l| rps_move(l).unwrap().1);

    let points = parsed.map(|moves| moves.1.points() + round_result(moves.1, moves.0));

    println!("Answer part 1: {:?}", points.sum::<usize>());


    // Part 2
    println!("Part 2");

    let parsed2 = input.lines().map(|l| rps_move_part2(l).unwrap().1);

    let points = parsed2.map(|moves| round_move(moves.0, moves.1).points() + moves.1.points());

    println!("Answer part 2: {:?}", points.sum::<usize>());
}
