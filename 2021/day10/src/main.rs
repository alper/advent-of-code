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
    assert_eq!(corrupted("()"), ' ');
    assert_eq!(corrupted("(("), ' ');
    assert_eq!(corrupted("(())"), ' ');
    assert_eq!(corrupted("((}"), '}');

    assert_eq!(corrupted("([])"), ' ');
    assert_eq!(corrupted("{()()()}"), ' ');
    assert_eq!(corrupted("<([{}])>"), ' ');
    assert_eq!(corrupted("[<>({}){}[([])<>]]"), ' ');
    assert_eq!(corrupted("(((((((((())))))))))"), ' ');

    assert_eq!(corrupted("(]"), ']');
    assert_eq!(corrupted("{()()()>"), '>');
    assert_eq!(corrupted("(((()))}"), '}');
    assert_eq!(corrupted("<([]){()}[{}])"), ')');

}

fn corrupted(series: &str) -> char {
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
                    return c;
                }
            }
        }
    }
    return ' ';
}