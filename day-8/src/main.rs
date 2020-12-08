use std::fs;

fn main() {
    // let input = fs::read_to_string("input.txt").expect("Dead file");

    let input = r"nop +0
    acc +1
    jmp +4
    acc +3
    jmp -3
    acc -99
    acc +1
    jmp -4
    acc +6";

    for line in input.lines() {
        println!("{:?}", parse_line(line));
    }
}

#[derive(PartialEq, Debug)]
enum Instruction {
    Nop,
    Acc(i32),
    Jmp(i32),
}

fn parse_line(l: &str) -> Instruction {
    let mut parts = l.split_whitespace();
    let ins = parts.next();
    let offs = parts.next().unwrap().parse::<i32>().unwrap();

    return match ins {
        Some("nop") => Instruction::Nop,
        Some("acc") => Instruction::Acc(offs),
        Some("jmp") => Instruction::Jmp(offs),
        _ => Instruction::Nop,
    };
}

#[test]
fn test_parse_line() {
    assert_eq!(parse_line("jmp +1"), Instruction::Jmp(1));
    assert_eq!(parse_line("jmp -21"), Instruction::Jmp(-21));
    assert_eq!(parse_line("nop +1000"), Instruction::Nop);
}
