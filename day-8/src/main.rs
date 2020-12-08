use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Dead file");

    // let input = r"nop +0
    // acc +1
    // jmp +4
    // acc +3
    // jmp -3
    // acc -99
    // acc +1
    // jmp -4
    // acc +6";

    let mut instructions: Vec<Instruction> = Vec::new();

    for line in input.lines() {
        instructions.push(parse_line(line));
    }

    // Run program
    let mut accumulator = 0i32;
    let mut instruction_pointer = 0i32;

    let mut instruction_history = HashSet::new();
    instruction_history.insert(instruction_pointer);

    loop {
        match instructions[instruction_pointer as usize] {
            Instruction::Nop(_n) => {
                instruction_pointer += 1;
            }
            Instruction::Acc(n) => {
                accumulator += n;
                instruction_pointer += 1;
            }
            Instruction::Jmp(n) => {
                instruction_pointer += n;
            }
        }

        if !instruction_history.insert(instruction_pointer) {
            break;
        }
    }
    println!("Accumulator after halt: {}", accumulator);
}

#[derive(PartialEq, Debug)]
enum Instruction {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

fn parse_line(l: &str) -> Instruction {
    let mut parts = l.split_whitespace();
    let ins = parts.next();
    let offs = parts.next().unwrap().parse::<i32>().unwrap();

    return match ins {
        Some("nop") => Instruction::Nop(offs),
        Some("acc") => Instruction::Acc(offs),
        Some("jmp") => Instruction::Jmp(offs),
        _ => Instruction::Nop(0),
    };
}

#[test]
fn test_parse_line() {
    assert_eq!(parse_line("jmp +1"), Instruction::Jmp(1));
    assert_eq!(parse_line("jmp -21"), Instruction::Jmp(-21));
    assert_eq!(parse_line("nop +1000"), Instruction::Nop);
}
