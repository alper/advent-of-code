use std::convert::TryFrom;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("File not readable");

    let mut p = parse_program(&input);
    p[1] = 12;
    p[2] = 2;

    run_program(&mut p, &mut vec![]);

    println!("Answer 1: {:?}", p[0]);

    let target_output = 19690720;

    'outer: for noun in 0..100 {
        for verb in 0..100 {
            let mut mem = parse_program(&input);
            mem[1] = noun;
            mem[2] = verb;

            run_program(&mut mem, &mut vec![]);

            if mem[0] == target_output {
                println!("Answer 2: {}", 100 * noun + verb);
                break 'outer;
            }
        }
    }
}

#[test]
fn test_run_program() {
    let mut p1 = parse_program("1,0,0,0,99");
    run_program(&mut p1, &mut vec![]);
    assert_eq!(p1, parse_program("2,0,0,0,99"));

    let mut p2 = parse_program("2,3,0,3,99");
    run_program(&mut p2, &mut vec![]);
    assert_eq!(p2, parse_program("2,3,0,6,99"));

    let mut p1 = parse_program("2,4,4,5,99,0");
    run_program(&mut p1, &mut vec![]);
    assert_eq!(p1, parse_program("2,4,4,5,99,9801"));

    let mut p1 = parse_program("1,1,1,4,99,5,6,0,99");
    run_program(&mut p1, &mut vec![]);
    assert_eq!(p1, parse_program("30,1,1,4,2,5,6,0,99"));
}

fn parse_program(string_series: &str) -> Vec<isize> {
    string_series
        .trim()
        .split(',')
        .map(|el| el.parse().unwrap())
        .collect::<Vec<isize>>()
}

fn run_program(memory: &mut Vec<isize>, inputs: &mut Vec<isize>) -> Vec<isize> {
    let mut instruction_pointer = 0;

    let mut outputs = vec![];

    loop {
        let opcode = memory[instruction_pointer];

        // TODO decode parameter mode

        match opcode {
            1 | 2 => {
                let lparam_addr = usize::try_from(memory[instruction_pointer + 1]).unwrap();
                let rparam_addr = usize::try_from(memory[instruction_pointer + 2]).unwrap();
                let dest_addr = usize::try_from(memory[instruction_pointer + 3]).unwrap();

                // println!(
                //     "{} => {} {} {}",
                //     opcode, lparam_addr, rparam_addr, dest_addr
                // );

                let lparam = memory[lparam_addr];
                let rvalue = memory[rparam_addr];

                let result = match opcode {
                    1 => lparam + rvalue,
                    2 => lparam * rvalue,
                    _ => 0,
                };

                memory[dest_addr] = result;
                instruction_pointer += 4;
            }
            3 => {
                // Input
                let dest_addr = usize::try_from(memory[instruction_pointer + 1]).unwrap();

                let input_value = inputs.pop().unwrap();

                memory[dest_addr] = input_value;

                instruction_pointer += 2;
            }
            4 => {
                // Output
                let dest_addr = usize::try_from(memory[instruction_pointer + 1]).unwrap();

                let output_value = memory[dest_addr];

                outputs.push(output_value);

                instruction_pointer += 2;
            }
            99 => {
                break;
            }
            _ => {
                println!("Something went wrong!");
                break;
            }
        }
    }

    return outputs;
}
