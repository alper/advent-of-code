use std::collections::HashSet;
use std::fs;

fn main() {
    part1();
    part2();
}

fn part1() {
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

    println!(
        "Accumulator after halt: {}",
        run_program_until_loop(&instructions).1
    );
}

fn part2() {
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

    for (pos, ins) in instructions.iter().enumerate() {
        match ins {
            Instruction::Jmp(n) => {
                let mut alt_program = instructions.clone();
                alt_program[pos] = Instruction::Nop(*n);

                let (lp, acc) = run_program_until_loop(&alt_program);

                if !lp {
                    println!("Terminated without loop with accumulater: {} by changing the instruction {:?} at index: {}", acc, ins, pos);
                }
            }
            Instruction::Nop(n) => {
                let mut alt_program = instructions.clone();
                alt_program[pos] = Instruction::Jmp(*n);

                let (lp, acc) = run_program_until_loop(&alt_program);

                if !lp {
                    println!("Terminated without loop with accumulater: {} by changing the instruction {:?} at index: {}", acc, ins, pos);
                }
            }
            Instruction::Acc(_n) => {}
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
enum Instruction {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

/// Run program
/// Return true if it looped and was halted, false if it ran through
fn run_program_until_loop(prog: &Vec<Instruction>) -> (bool, i32) {
    // Run program
    let mut accumulator = 0i32;
    let mut instruction_pointer = 0i32;

    let mut instruction_history = HashSet::new();
    instruction_history.insert(instruction_pointer);

    loop {
        match prog[instruction_pointer as usize] {
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
            return (true, accumulator);
        }

        if instruction_pointer == prog.len() as i32 {
            return (false, accumulator);
        }
    }
}

fn parse_line(l: &str) -> Instruction {
    let mut parts = l.split_whitespace();
    let ins = parts.next();
    let offs = parts.next().unwrap().parse::<i32>().unwrap();

    match ins {
        Some("nop") => Instruction::Nop(offs),
        Some("acc") => Instruction::Acc(offs),
        Some("jmp") => Instruction::Jmp(offs),
        _ => Instruction::Nop(0),
    }
}

#[test]
fn test_parse_line() {
    assert_eq!(parse_line("jmp +1"), Instruction::Jmp(1));
    assert_eq!(parse_line("jmp -21"), Instruction::Jmp(-21));
    assert_eq!(parse_line("nop +1000"), Instruction::Nop);
}
