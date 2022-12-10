use itertools::Itertools;
use std::{
    collections::VecDeque,
    fmt::{self, Result},
    fs,
};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Instruction {
    Noop,
    Addx(isize),
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result {
        match self {
            Self::Noop => write!(f, "noop"),
            Self::Addx(x) => write!(f, "addx {}", x),
        }
    }
}

struct VideoSystem {
    cycle: isize,

    register_X: isize,

    instructions: VecDeque<Instruction>,
    current_instruction: Instruction,
    add_turns: usize,

    signal_strength_accumulator: isize,

    crt: [char; 240],
}

impl fmt::Display for VideoSystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result {
        write!(
            f,
            "Cycle: {}, X: {} (curr: {}, add_turns: {}, sig: {})",
            self.cycle,
            self.register_X,
            self.current_instruction,
            self.add_turns,
            self.signal_strength_accumulator
        )
    }
}

impl VideoSystem {
    fn run(&mut self) {
        loop {
            // Print current state
            // println!("Start/ {}", self);

            // Begin execution
            match self.current_instruction {
                Instruction::Noop => {
                    if let Some(i) = self.instructions.pop_front() {
                        self.current_instruction = i;

                        match self.current_instruction {
                            Instruction::Addx(x) => {
                                self.add_turns = 2;
                            }
                            Instruction::Noop => {}
                        }
                    } else {
                        if self.add_turns == 0 {
                            break;
                        }
                    }
                }
                Instruction::Addx(_) => {
                    if self.add_turns == 0 {
                        if let Some(i) = self.instructions.pop_front() {
                            self.current_instruction = i;

                            match self.current_instruction {
                                Instruction::Addx(x) => {
                                    self.add_turns = 2;
                                }
                                Instruction::Noop => {}
                            }
                        } else {
                            if self.add_turns == 0 {
                                break;
                            }
                        }
                    }
                }
            }

            // Perform side effects
            if ((self.cycle as isize) - 20) % 40 == 0 {
                // println!(
                //     "Signal strength at cycle {} is {}",
                //     self.cycle,
                //     self.cycle * self.register_X
                // );

                self.signal_strength_accumulator += self.cycle * self.register_X;
            }

            if (self.cycle - 1) % 40 == self.register_X - 1
                || (self.cycle - 1) % 40 == self.register_X
                || (self.cycle - 1) % 40 == self.register_X + 1
            {
                // println!("Drawing '#' in pos: {}", self.cycle - 1);

                self.crt[usize::try_from(self.cycle - 1).unwrap()] = '#';
            } else {
                // println!("Drawing '.' in pos: {}", self.cycle - 1);
            }

            // println!(
            //     "Current CRT row: {}",
            //     &self.crt[..40].iter().collect::<String>()
            // );

            // Complete execution
            match self.current_instruction {
                Instruction::Noop => {}
                Instruction::Addx(x) => {
                    if self.add_turns == 1 {
                        self.register_X += x;
                    }
                }
            }

            // Decrement cycle counter
            if self.add_turns > 0 {
                self.add_turns -= 1;
            }

            // println!("End/ {}", self);
            // println!("");

            self.cycle += 1;
        }
    }
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let input = fs::read_to_string("full_input.txt").expect("File not readable");

    let mut instructions: VecDeque<Instruction> = input
        .lines()
        .map(|l| {
            let instruction = &l[..4];

            match instruction {
                "noop" => Instruction::Noop,
                "addx" => {
                    let (_, amount) = l.split(" ").collect_tuple().unwrap();
                    Instruction::Addx(str::parse::<isize>(amount).unwrap())
                }
                _ => unimplemented!("Should not happen!"),
            }
        })
        .collect();
    // println!("{:?}", instructions);

    // Part 1

    let mut v = VideoSystem {
        cycle: 1,
        register_X: 1,
        instructions: instructions.clone(),
        current_instruction: Instruction::Noop,
        add_turns: 0,
        signal_strength_accumulator: 0,
        crt: ['.'; 240],
    };

    v.run();

    println!("Answer part 1: {:?}", v.signal_strength_accumulator);

    println!("Answer part 2");

    for cs in v.crt.chunks(40) {
        println!("{}", cs.iter().collect::<String>());
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
