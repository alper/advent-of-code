extern crate nom;

use std::fs;

use nom::{
    bytes::complete::tag, character::complete::digit1, combinator::map_res, sequence::tuple,
    IResult,
};

fn parse_section_assignment(input: &str) -> IResult<&str, ((u32, u32), (u32, u32))> {
    let (input, (start1, _, end1, _, start2, _, end2)) = tuple((
        map_res(digit1, str::parse),
        tag("-"),
        map_res(digit1, str::parse),
        tag(","),
        map_res(digit1, str::parse),
        tag("-"),
        map_res(digit1, str::parse),
    ))(input)?;

    Ok((input, ((start1, end1), (start2, end2))))
}

fn main() {
    let input = fs::read_to_string("full_input.txt").expect("File not readable");

    // Part 1
    println!("Part 1");

    let parsed: Vec<_> = input
        .lines()
        .map(|l| parse_section_assignment(l).unwrap().1)
        .collect();

    println!("{:?}", parsed);

    let fully_contained = parsed.iter().filter(|((start1, end1), (start2, end2))| {
        (start1 >= start2 && end1 <= end2) || (start2 >= start1 && end2 <= end1)
    });

    println!("Answer part 1: {:?}", fully_contained.count());

    // Part 2
    println!("Part 2");

    let partially_contained = parsed.iter().filter(|((start1, end1), (start2, end2))| {
        (start1 >= start2 && start1 <= end2)
            || (end1 >= start2 && end1 <= end2)
            || (start2 >= start1 && start2 <= end1)
            || (end2 >= start1 && end2 <= end1)
    });

    println!("Answer part 2: {:?}", partially_contained.count());
}
