use std::fs;
use itertools::{Itertools};

fn main() {
    let input = fs::read_to_string("input.txt").expect("File not readable");

    let v: Vec<_> = input
        .lines()
        .map(|l| corrupted(l))
        .map(|c| match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => 0,
        })
        .collect();

    println!("Answer 1: {:?}", v.iter().sum::<usize>());

    let scores: Vec<usize> = input.lines().filter(|l| corrupted(l) == ' ').map(|l| score(l)).sorted().collect();

    println!("Score: {:?}", scores);
    println!("Answer 2: {:?}", scores[scores.len()/2]);
}





#[test]
fn test_corrupted() {
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

fn score(series: &str) -> usize {
    let mut stack = vec![];

    for c in series.chars() {
        if c == '(' || c == '[' || c == '{' || c == '<' {
            stack.push(c);
        } else if c == ')' || c == ']' || c == '}' || c == '>' {
            stack.pop();
        }
    }

    let mut score = 0;

    loop {
        match stack.pop().unwrap() {
            '(' => {
                score *= 5;
                score += 1;
            },
            '[' => {
                score *= 5;
                score += 2;
            },
            '{' => {
                score *= 5;
                score += 3;
            },
            '<' => {
                score *= 5;
                score += 4;
            },
            _ => {},
        }

        if stack.len() == 0 {
            break;
        }
    }

    score
}