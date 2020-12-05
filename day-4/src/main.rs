extern crate nom;

use std::collections::HashSet;
use std::fs;
use std::io;
use std::iter::FromIterator;

use nom::{
    branch::alt,
    branch::permutation,
    bytes::complete::{tag, take_while_m_n},
    character::complete::{digit1, multispace0},
    combinator::map_res,
    dbg_dmp,
    sequence::{preceded, terminated},
    IResult,
};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Dead file");
    let count = input.split("\n\n").filter(|b| check(b)).count();

    println!("Valid passports: {:?}", count);
}

fn check(block: &str) -> bool {
    let required_fields: HashSet<&str> =
        HashSet::from_iter(vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]);

    let present_fields: HashSet<&str> = HashSet::from_iter(
        block
            .split_whitespace()
            .map(|f| f.split(':').next().unwrap()),
    );

    let diff = required_fields.difference(&present_fields);

    diff.copied().next().is_none()
}

fn check_contents(input: &str) -> IResult<&str, (&str, &str, u16, u16, u16)> {
    permutation((
        check_eye_color,
        check_passport_id,
        check_birth_year,
        check_issue_year,
        check_expiry_year,
    ))(input)
}

#[test]
fn test_check_contents() {
    assert_eq!(
        check_contents("pid:123456789 ecl:amb byr:1981 iyr:2020 eyr:2020"),
        Ok(("", ("amb", "123456789", 1981, 2020, 2020)))
    );

    assert_eq!(
        check_contents(
            r"iyr:2011
        pid:123456789 ecl:amb
        byr:1981 eyr:2025"
        ),
        Ok(("", ("amb", "123456789", 1981, 2011, 2025)))
    );

    assert_eq!(
        check_contents("pid:123456789    "),
        Err(nom::Err::Error(nom::error::Error::new(
            "",
            nom::error::ErrorKind::Tag,
        )))
    );
}

fn check_birth_year(input: &str) -> IResult<&str, u16> {
    check_year_record(input, "byr", 1920, 2002)
}

fn check_issue_year(input: &str) -> IResult<&str, u16> {
    check_year_record(input, "iyr", 2010, 2020)
}

fn check_expiry_year(input: &str) -> IResult<&str, u16> {
    check_year_record(input, "eyr", 2020, 2030)
}

fn check_year_record<'a>(
    input: &'a str,
    field: &str,
    lower_bound: u16,
    upper_bound: u16,
) -> IResult<&'a str, u16> {
    terminated(
        preceded(
            tag((field.to_string() + ":").as_str()),
            map_res(digit1, |s: &str| match s.parse::<u16>() {
                Ok(n) if n < lower_bound || n > upper_bound => {
                    Err(io::Error::new(io::ErrorKind::Other, "out of range"))
                }
                Ok(n) => Ok(n),
                Err(e) => Err(io::Error::new(io::ErrorKind::Other, e.to_string())),
            }),
        ),
        multispace0,
    )(input)
}

#[test]
fn test_check_year_record() {
    assert_eq!(
        check_year_record("byr:1981", "byr", 1920, 2002),
        Ok(("", 1981))
    );

    assert!(check_year_record("byr:1900", "byr", 1920, 2002).is_err());
}

fn check_eye_color(input: &str) -> IResult<&str, &str> {
    let color = alt((
        tag("amb"),
        tag("blu"),
        tag("brn"),
        tag("gry"),
        tag("grn"),
        tag("hzl"),
        tag("oth"),
    ));

    terminated(preceded(tag("ecl:"), color), multispace0)(input)
}

#[test]
fn test_check_eye_color() {
    assert_eq!(check_eye_color("ecl:amb"), Ok(("", "amb")));

    assert!(check_eye_color("  ecl:amb").is_err());

    assert_eq!(check_eye_color("ecl:amb  "), Ok(("", "amb")));

    assert!(check_eye_color("ecl:bul").is_err());
}

fn check_passport_id(input: &str) -> IResult<&str, &str> {
    let pid = take_while_m_n(9, 9, |c: char| c.is_digit(10));

    terminated(preceded(tag("pid:"), pid), multispace0)(input)
}

#[test]
fn test_check_passport_id() {
    assert_eq!(check_passport_id("pid:123456789"), Ok(("", "123456789")));
    assert_eq!(
        check_passport_id("pid:123456789    "),
        Ok(("", "123456789"))
    );
    // assert_eq!(check_passport_id("pid:12345678912    "), Ok(("12", "12")));
}
