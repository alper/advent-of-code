use std::fs;

fn main() {
    let file_contents = fs::read_to_string("input.txt").expect("Dead file");

    println!(
        "Answer part 1: {}",
        file_contents
            .lines()
            .map(|l| calculate_value(l.trim()))
            .sum::<u64>()
    );
}

fn calculate_value(input: &str) -> u64 {
    let mut s = input.to_owned();

    loop {
        if s.contains("(") {
            s = reduce_parens_expression(&s);
        } else {
            // return evaluate_string_expression(&s);
            return evaluate_string_expression2(&s);
        }
    }
}

#[test]
fn test_calculate_value() {
    assert_eq!(calculate_value("(1 + 2)"), 3);
    assert_eq!(calculate_value("(1 + (2 * 3)) + (1 + 2)"), 10);
}

fn reduce_parens_expression(input: &str) -> String {
    assert!(input.contains("("));

    println!("In: {}", input);

    let mut found_lpar = false;
    let mut nlpar = 0;
    let mut nrpar = 0;
    for (i, c) in input.char_indices() {
        if !found_lpar && c == '(' {
            found_lpar = true;
            nlpar = i;
        } else if found_lpar && c == '(' {
            nlpar = i;
        } else if found_lpar && c == ')' {
            nrpar = i;
            break;
        }
    }

    println!("Indexes end: {} {}", nlpar, nrpar);

    // let val = evaluate_string_expression(&input[nlpar + 1..nrpar]);
    let val = evaluate_string_expression2(&input[nlpar + 1..nrpar]);

    println!("Val: {}", val);

    let mut s = String::from(input);
    s.replace_range(nlpar..nrpar + 1, &val.to_string());

    s
}

#[test]
fn test_reduce_parens_expression() {
    assert_eq!(reduce_parens_expression("(1 + 2)"), "3");
    assert_eq!(reduce_parens_expression("7 + (1 + 2)"), "7 + 3");
    assert_eq!(
        reduce_parens_expression("(1 + (2 * 3)) + (1 + 2)"),
        "(1 + 6) + (1 + 2)"
    );
}

fn evaluate_string_expression(input: &str) -> u64 {
    let mut parts = input.split(" ").collect::<Vec<_>>();
    println!("Parts: {:?}", parts);

    let mut accumulator = parts[0].parse::<u64>().unwrap();
    parts.remove(0);

    for c in parts.chunks(2) {
        match c[..] {
            [op, right] => {
                println!("Matched: {} {}", op, right);

                match op {
                    // The only two operands in the input.
                    "+" => accumulator += right.parse::<u64>().unwrap(),
                    "*" => accumulator *= right.parse::<u64>().unwrap(),
                    _ => {}
                };
            }
            _ => {}
        }
    }

    accumulator
}

#[test]
fn test_evaluate_string_expression() {
    assert_eq!(evaluate_string_expression("2 * 2"), 4);
    assert_eq!(evaluate_string_expression("12 * 2"), 24);
    assert_eq!(evaluate_string_expression("2 + 2 * 3"), 12);
    assert_eq!(evaluate_string_expression("10 * 2 + 4 * 3"), 72);
}

fn evaluate_string_expression2(input: &str) -> u64 {
    let mut parts = input
        .split(" ")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    println!("Parts: {:?}", parts);

    while parts.contains(&String::from("+")) {
        match parts.iter().position(|p| p == "+") {
            Some(n) => {
                let val =
                    parts[n - 1].parse::<u64>().unwrap() + parts[n + 1].parse::<u64>().unwrap();
                println!("Val: {}", val);

                let range = n - 1..n + 2;
                parts.splice(range, [val.to_string()].iter().cloned());
            }
            None => {}
        }
    }

    println!("Parts: {:?}", parts);

    while parts.contains(&String::from("*")) {
        match parts.iter().position(|p| p == "*") {
            Some(n) => {
                let val =
                    parts[n - 1].parse::<u64>().unwrap() * parts[n + 1].parse::<u64>().unwrap();
                println!("Val: {}", val);

                let range = n - 1..n + 2;
                parts.splice(range, [val.to_string()].iter().cloned());
            }
            None => {}
        }
    }

    parts[0].parse::<u64>().unwrap()
}

#[test]
fn test_evaluate_string_expression2() {
    assert_eq!(evaluate_string_expression2("2 + 2"), 4);
    assert_eq!(evaluate_string_expression2("12 * 2"), 24);
    assert_eq!(evaluate_string_expression2("2 + 2 * 3"), 12);
    assert_eq!(evaluate_string_expression2("10 * 2 + 4 * 3"), 180);
}
