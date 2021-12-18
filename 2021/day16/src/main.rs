extern crate nom;

use itertools::Itertools;
use nom::branch::alt;
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
    let input = fs::read_to_string("input.txt").expect("File not readable");

    let binary_input = hex_to_bin(&input);
    let p = parse_packet(&binary_input);

    println!("Packets: {:?}", p);

    let packet = p.unwrap().1;

    println!("Answer 1: {}", sum_packet_version(&packet));

    println!("Answer 2: {}", calculate_packet_value(&packet));
}

fn sum_packet_version(p: &Packet) -> usize {
    match p {
        Packet::Operator {
            version,
            operator_type,
            subpackets,
        } => {
            let sub_packet_version_sum: usize =
                subpackets.iter().map(|p| sum_packet_version(p)).sum();
            return version + sub_packet_version_sum;
        }
        Packet::Literal { version, value } => *version,
    }
}

fn calculate_packet_value(p: &Packet) -> usize {
    match p {
        Packet::Literal { version, value } => *version,
        Packet::Operator {
            version,
            operator_type,
            subpackets,
        } => match operator_type {
            PacketType::OperatorSum => subpackets.iter().map(|p| calculate_packet_value(p)).sum(),
            PacketType::OperatorProduct => subpackets
                .iter()
                .map(|p| calculate_packet_value(p))
                .product(),
            PacketType::OperatorMinimum => subpackets
                .iter()
                .map(|p| calculate_packet_value(p))
                .min()
                .unwrap(),
            PacketType::OperatorMaximum => subpackets
                .iter()
                .map(|p| calculate_packet_value(p))
                .max()
                .unwrap(),
            PacketType::OperatorLessThan => {
                if calculate_packet_value(&subpackets[0]) < calculate_packet_value(&subpackets[1]) {
                    return 1;
                } else {
                    return 0;
                }
            }
            PacketType::OperatorGreaterThan => {
                if calculate_packet_value(&subpackets[0]) > calculate_packet_value(&subpackets[1]) {
                    return 1;
                } else {
                    return 0;
                }
            }
            PacketType::OperatorEqualTo => {
                if calculate_packet_value(&subpackets[0]) == calculate_packet_value(&subpackets[1])
                {
                    return 1;
                } else {
                    return 0;
                }
            }
            _ => 0,
        },
    }
}

#[derive(Debug, Clone)]
enum Packet {
    Operator {
        version: usize,
        operator_type: PacketType,
        subpackets: Vec<Packet>,
    },
    Literal {
        version: usize,
        value: usize,
    },
}

#[test]
fn test_parse() {
    // println!("{:?}", literal_packet("110100101111111000101000"));

    // println!(
    //     "8A004A801A8002F478 {:?}",
    //     parse_packet(&hex_to_bin("8A004A801A8002F478"))
    // );

    println!(
        "620080001611562C8802118E34 {:?}",
        parse_packet(&hex_to_bin("620080001611562C8802118E34"))
    );

    // println!(
    //     "C0015000016115A2E0802F182340 {:?}",
    //     parse_packet(&hex_to_bin("C0015000016115A2E0802F182340"))
    // );

    // Take subpacket that errors
    // println!(
    //     "{:?}",
    //     parse_packet(
    //         "000000000000000001011000010001010110100010111000001000000000101111000110000010001101"
    //     )
    // );

    // println!(
    //     "A0016C880162017C3686B18A3D4780 {:?}",
    //     parse_packet(&hex_to_bin("A0016C880162017C3686B18A3D4780"))
    // );

    // println!(
    //     "38006F45291200 {:?}",
    //     parse_packet(&hex_to_bin("38006F45291200"))
    // );

    // println!(
    //     "EE00D40C823060 {:?}",
    //     parse_packet(&hex_to_bin("EE00D40C823060"))
    // );

    assert_eq!(1, 2);
}

fn parse_packet(input: &str) -> IResult<&str, Packet> {
    alt((literal_packet, operator_packet))(input)
}

fn literal_packet(input: &str) -> IResult<&str, Packet> {
    println!("Parse literal: {}", input);

    let (input, packet_version) = packet_version(input)?;
    let (input, packet_type) = tag("100")(input)?;
    let (input, number_blocks) = many0(preceded(tag("1"), four_digits))(input)?;
    let (input, number_end_block) = preceded(tag("0"), four_digits)(input)?;

    let mut v = Vec::from(number_blocks);
    v.push(number_end_block);
    let n = usize::from_str_radix(&v.join(""), 2).unwrap();

    Ok((
        input,
        Packet::Literal {
            version: packet_version,
            value: n,
        },
    ))
}

fn packet_version(input: &str) -> IResult<&str, usize> {
    map(count(one_of("01"), 3), |v| {
        usize::from_str_radix(&v.iter().collect::<String>(), 2).unwrap()
    })(input)
}

#[derive(Debug, Copy, Clone)]
enum PacketType {
    Literal,
    OperatorSum,
    OperatorProduct,
    OperatorMinimum,
    OperatorMaximum,
    OperatorGreaterThan,
    OperatorLessThan,
    OperatorEqualTo,
}

fn packet_type(input: &str) -> IResult<&str, PacketType> {
    let (input, t) = count(one_of("01"), 3)(input)?;

    let packet_type_id = usize::from_str_radix(&t.iter().join(""), 2).unwrap();
    let packet_type = match packet_type_id {
        0 => PacketType::OperatorSum,
        1 => PacketType::OperatorProduct,
        2 => PacketType::OperatorMinimum,
        3 => PacketType::OperatorMaximum,
        5 => PacketType::OperatorGreaterThan,
        6 => PacketType::OperatorLessThan,
        7 => PacketType::OperatorEqualTo,
        _ => PacketType::Literal,
    };

    Ok((input, packet_type))
}

#[derive(Debug, Copy, Clone)]
enum LengthTypeID {
    TotalLength(usize),
    NumberOfSubpackets(usize),
}

fn packet_length_type_id(input: &str) -> IResult<&str, LengthTypeID> {
    println!("Packet length type input: {}", input);

    let (input, c) = one_of("01")(input)?;

    if c == '0' {
        let (input, n) = map(count(one_of("01"), 15), |v| {
            usize::from_str_radix(&v.iter().collect::<String>(), 2).unwrap()
        })(input)?;

        return Ok((input, LengthTypeID::TotalLength(n)));
    } else {
        let (input, n) = map(count(one_of("01"), 11), |v| {
            usize::from_str_radix(&v.iter().collect::<String>(), 2).unwrap()
        })(input)?;

        return Ok((input, LengthTypeID::NumberOfSubpackets(n)));
    }
}

fn operator_packet(input: &str) -> IResult<&str, Packet> {
    println!("Parse operator: {}", input);

    let (input, packet_version) = packet_version(input)?;
    println!("Packet version: {}", packet_version);

    let (input, p_type) = packet_type(input)?;
    println!("Packet type: {:?}", p_type);

    let (input, p_length) = packet_length_type_id(input)?;
    println!("Length type: {:?}", p_length);

    if let LengthTypeID::TotalLength(n) = p_length {
        let length_to_parse = &input[..n];
        let (_, subpackets) = many0(parse_packet)(length_to_parse)?;

        let rest_to_parse = &input[n..];

        return Ok((
            rest_to_parse,
            Packet::Operator {
                version: packet_version,
                operator_type: p_type,
                subpackets: subpackets,
            },
        ));
    } else if let LengthTypeID::NumberOfSubpackets(n) = p_length {
        let (input, subpackets) = count(parse_packet, n)(input)?;

        return Ok((
            input,
            Packet::Operator {
                version: packet_version,
                operator_type: p_type,
                subpackets: subpackets,
            },
        ));
    } else {
        // Fall back case, should not be reached
        unimplemented!();
    }
}

fn four_digits(input: &str) -> IResult<&str, String> {
    map(count(one_of("01"), 4), |v| v.iter().collect::<String>())(input)
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
