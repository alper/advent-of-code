use std::fs;
use itertools::{Itertools};

fn main() {
    let input = fs::read_to_string("test_input.txt").expect("File not readable");

    println!("Input: {:?}", input);
}



// #[test]
// fn test_validate_line() {

// }

// fn validate_line(l: &str) -> bool {

// }

#[test]
fn test_balanced() {
    assert_eq!(balanced("()"), true);
    assert_eq!(balanced("(("), false);
    assert_eq!(balanced("(())"), true);
    assert_eq!(balanced("((}"), false);

    assert_eq!(balanced("([])"), true);
    assert_eq!(balanced("{()()()}"), true);
    assert_eq!(balanced("<([{}])>"), true);
    assert_eq!(balanced("[<>({}){}[([])<>]]"), true);
    assert_eq!(balanced("(((((((((())))))))))"), true);
}

fn balanced(series: &str) -> bool {
    let mut stack = vec![];

    for c in series.chars() {
        if c == '(' || c == '[' || c == '{' || c == '<' {
            stack.push(c);
        } else if c == ')' || c == ']' || c == '}' || c == '>' {
            if let Some(b) = stack.pop() {
                if (c == ')' && b == '(') ||
                (c == ']' && b == '[') ||
                (c == '}' && b == '{') ||
                (c == '>' && b == '<') {
                    // pass
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }
    }

    if stack.len() != 0 {
        return false;
    }

    return true;
}