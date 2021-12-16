extern crate nom;

use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::one_of;
use nom::combinator::map;
use nom::combinator::map_res;
use nom::multi::count;
use nom::multi::many0;
use nom::sequence::preceded;
use nom::IResult;
use std::fs;

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
    let (input, packet_version) =
        map(count(one_of("01"), 3), |v| v.iter().collect::<String>())(input)?;
    println!("Packet version: {:?}", packet_version);

    let (input, packet_type) = tag("100")(input)?;

    let (input, number_blocks) = many0(preceded(tag("1"), four_digits))(input)?;
    println!("Number blocks: {:?}", number_blocks);

    let (input, number_end_block) = preceded(tag("0"), four_digits)(input)?;
    println!("Number end block: {:?}", number_end_block);

    let mut v = Vec::from(number_blocks);
    v.push(number_end_block);
    let n = usize::from_str_radix(&v.join(""), 2).unwrap();

    Ok((input, PacketType::Literal(n)))
}

fn four_digits(input: &str) -> IResult<&str, String> {
    map(count(one_of("01"), 4), |v| v.iter().collect::<String>())(input)
}

fn operator_packet(input: &str) -> IResult<&str, PacketType> {
    let (input, packet_version) = count(one_of("01"), 3)(input)?;

    Ok((input, PacketType::Operator))
}

#[test]
fn test_hex_to_bin() {
    assert_eq!(hex_to_bin("D2FE28"), "110100101111111000101000");
    assert_eq!(
        hex_to_bin("38006F45291200"),
        "00111000000000000110111101000101001010010001001000000000"
    );
    assert_eq!(
        hex_to_bin("EE00D40C823060"),
        "11101110000000001101010000001100100000100011000001100000"
    );
}

fn hex_to_bin(h: &str) -> String {
    h.chars()
        .map(|c| match c {
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
        })
        .join("")
}
