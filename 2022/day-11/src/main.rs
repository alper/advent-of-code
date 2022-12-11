use core::fmt;
use std::{collections::VecDeque, fmt::write, fs};

use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
enum Operation {
    Times(usize),
    Square,
    Add(usize),
}

struct Monkey {
    items: VecDeque<usize>,
    operation: Operation,
    test_div: usize,
    true_target: usize,
    false_target: usize,
    lcm: usize,
}

impl fmt::Debug for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Monkey X: ");

        write!(f, "{}", self.items.iter().join(", "))
    }
}

fn get_test_monkeys() -> [Monkey; 4] {
    [
        Monkey {
            items: VecDeque::from([79, 98]),
            operation: Operation::Times(19),
            test_div: 23,
            true_target: 2,
            false_target: 3,
            lcm: 96577,
        },
        Monkey {
            items: VecDeque::from([54, 65, 75, 74]),
            operation: Operation::Add(6),
            test_div: 19,
            true_target: 2,
            false_target: 0,
            lcm: 96577,
        },
        Monkey {
            items: VecDeque::from([79, 60, 97]),
            operation: Operation::Square,
            test_div: 13,
            true_target: 1,
            false_target: 3,
            lcm: 96577,
        },
        Monkey {
            items: VecDeque::from([74]),
            operation: Operation::Add(3),
            test_div: 17,
            true_target: 0,
            false_target: 1,
            lcm: 96577,
        },
    ]
}

fn get_monkeys() -> [Monkey; 8] {
    [
        Monkey {
            items: VecDeque::from([65, 58, 93, 57, 66]),
            operation: Operation::Times(7),
            test_div: 19,
            true_target: 6,
            false_target: 4,
            lcm: 9699690,
        },
        Monkey {
            items: VecDeque::from([76, 97, 58, 72, 57, 92, 82]),
            operation: Operation::Add(4),
            test_div: 3,
            true_target: 7,
            false_target: 5,
            lcm: 9699690,
        },
        Monkey {
            items: VecDeque::from([90, 89, 96]),
            operation: Operation::Times(5),
            test_div: 13,
            true_target: 5,
            false_target: 1,
            lcm: 9699690,
        },
        Monkey {
            items: VecDeque::from([72, 63, 72, 99]),
            operation: Operation::Square,
            test_div: 17,
            true_target: 0,
            false_target: 4,
            lcm: 9699690,
        },
        Monkey {
            items: VecDeque::from([65]),
            operation: Operation::Add(1),
            test_div: 2,
            true_target: 6,
            false_target: 2,
            lcm: 9699690,
        },
        Monkey {
            items: VecDeque::from([97, 71]),
            operation: Operation::Add(8),
            test_div: 11,
            true_target: 7,
            false_target: 3,
            lcm: 9699690,
        },
        Monkey {
            items: VecDeque::from([83, 68, 88, 55, 87, 67]),
            operation: Operation::Add(2),
            test_div: 5,
            true_target: 2,
            false_target: 1,
            lcm: 9699690,
        },
        Monkey {
            items: VecDeque::from([64, 81, 50, 96, 82, 53, 62, 92]),
            operation: Operation::Add(5),
            test_div: 7,
            true_target: 3,
            false_target: 0,
            lcm: 9699690,
        },
    ]
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    // Part 1
    let mut monkeys = get_monkeys();

    println!("{:?}", monkeys);

    println!("Part 1");
    let mut inspections = [0; 8];

    for round in 0..20 {
        println!("After round: {}", round);
        for monkey in monkeys.iter() {
            println!("{:?}", monkey);
        }
        println!("");

        for i in 0..monkeys.len() {
            let mut monkey_items = monkeys[i].items.clone();
            monkeys.get_mut(i).unwrap().items.clear();

            let operation = monkeys.get(i).unwrap().operation;
            let test_div = monkeys.get(i).unwrap().test_div;
            let true_target = monkeys.get(i).unwrap().true_target;
            let false_target = monkeys.get(i).unwrap().false_target;

            loop {
                match monkey_items.pop_front() {
                    Some(item) => {
                        let after_inspection = match operation {
                            Operation::Times(x) => item * x,
                            Operation::Square => item * item,
                            Operation::Add(x) => item + x,
                        };
                        // Count the inspection
                        inspections[i] += 1;

                        let reduced_worry = after_inspection / 3;
                        if reduced_worry % test_div == 0 {
                            monkeys
                                .get_mut(true_target as usize)
                                .unwrap()
                                .items
                                .push_back(reduced_worry)
                        } else {
                            monkeys
                                .get_mut(false_target as usize)
                                .unwrap()
                                .items
                                .push_back(reduced_worry)
                        }
                    }
                    None => break,
                }
            }
        }
    }

    println!(
        "Answer part 1: {:?}",
        inspections
            .iter()
            .sorted()
            .rev()
            .take(2)
            .fold(1, |acc, el| acc * el)
    );

    // Part 2
    println!("Part 2");

    let mut monkeys = get_monkeys();

    println!("{:?}", monkeys);

    let mut inspections = [0; 8];

    for round in 0..10000 {
        if round == 1 || round == 20 || round == 1000 || round == 2000 {
            println!("Round: {round}");
            println!("Inspections: {:?}", inspections);
        }

        for i in 0..monkeys.len() {
            let mut monkey_items = monkeys[i].items.clone();
            monkeys.get_mut(i).unwrap().items.clear();

            let operation = monkeys.get(i).unwrap().operation;
            let test_div = monkeys.get(i).unwrap().test_div;
            let true_target = monkeys.get(i).unwrap().true_target;
            let false_target = monkeys.get(i).unwrap().false_target;
            let lcm = monkeys.get(i).unwrap().lcm;

            loop {
                match monkey_items.pop_front() {
                    Some(item) => {
                        let after_inspection = match operation {
                            Operation::Times(x) => item * x,
                            Operation::Square => item * item,
                            Operation::Add(x) => item + x,
                        };
                        // Count the inspection
                        inspections[i] += 1;

                        if after_inspection % test_div == 0 {
                            monkeys
                                .get_mut(true_target as usize)
                                .unwrap()
                                .items
                                .push_back(after_inspection % lcm)
                        } else {
                            monkeys
                                .get_mut(false_target as usize)
                                .unwrap()
                                .items
                                .push_back(after_inspection % lcm)
                        }
                    }
                    None => break,
                }
            }
        }
    }

    println!("Inspections: {:?}", inspections);

    println!(
        "Answer part 2: {:?}",
        inspections
            .iter()
            .sorted()
            .rev()
            .take(2)
            .fold(1_u128, |acc, el| acc * el)
    );

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
