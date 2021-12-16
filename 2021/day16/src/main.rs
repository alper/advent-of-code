extern crate nom;

use std::fs;
use nom::{
    IResult,
};
use chumsky::prelude::*;

fn main() {
    let input = fs::read_to_string("test_input.txt").expect("File not readable");

    println!("Input: {:?}", input);
}

#[derive(Debug)]
enum PacketType {
    Operator,
    Literal(usize),
}

#[test]
fn test_parse() {
    println!("{:?}", literal_packet("110100101111111000101000"));

    assert_eq!(1, 2);
}

fn literal_packet(input: &str) -> IResult<&str, PacketType> {
    let (input, packet_version) = count(one_of("01"), 3)(input)?;
    let (input, packet_type) = tag("100")(input)?;

    let (input, number_blocks) = many0(preceded(tag("1"), count(one_of("01"), 3)))(input)?;
    let (input, number_end_block) = preceded(tag("0"), count(one_of("01"), 3))(input)?;

    PacketType::Literal
}

#[test]
fn test_hex_to_bin() {
    assert_eq!(hex_to_bin("D2FE28"), "110100101111111000101000");
    assert_eq!(hex_to_bin("38006F45291200"), "00111000000000000110111101000101001010010001001000000000");
    assert_eq!(hex_to_bin("EE00D40C823060"), "11101110000000001101010000001100100000100011000001100000");
}

fn hex_to_bin(h: &str) -> String {
    h.chars().map(|c| match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    }).join("")
}