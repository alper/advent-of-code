use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::sync::atomic::{AtomicUsize, Ordering};

static MAX_8: AtomicUsize = AtomicUsize::new(0);
static MAX_11: AtomicUsize = AtomicUsize::new(0);

fn main() {
    let rules_contents = fs::read_to_string("rules.txt").expect("Dead file");

    let rule = parse_rules(&rules_contents);
    println!("Rules: {}", rule);

    let data_contents = fs::read_to_string("data.txt").expect("Dead file");

    let re = Regex::new(&rule).unwrap();
    let mut count = 0;
    for line in data_contents.lines() {
        if re.is_match(line.trim()) {
            println!("Yes: {}", line);
            count += 1;
        } else {
            println!("No: {}", line);
        }
    }

    println!("Count of correct data lines: {}", count);
}

fn parse_rules(input: &str) -> String {
    let mut rule_map: HashMap<u32, String> = HashMap::new();

    for line in input.lines() {
        println!("Line: {}", line);
        let mut parts = line.split(':');

        let index = parts.next().unwrap().parse::<u32>().unwrap();
        let rule = parts.next().unwrap().trim().to_string();

        rule_map.insert(index, rule);
    }

    println!("Map: {:?}", rule_map);

    return String::from("^") + &generate_regex(0, &rule_map) + "$";
}

#[test]
fn test_parse_rules() {
    assert_eq!(
        parse_rules(
            r##"0: 4 1
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b""##
        ),
        "^a((aa|bb)(ab|ba)|(ab|ba)(aa|bb))$"
    )
}

fn generate_regex(index: u32, rules: &HashMap<u32, String>) -> String {
    let rule = rules.get(&index).unwrap();
    println!("base /{}/", rule);

    if *rule == "\"a\"" {
        println!("a {}", rule);

        return String::from("a");
    } else if *rule == "\"b\"" {
        println!("b {}", rule);
        return String::from("b");
    } else if rule.split(' ').count() == 2 {
        println!("rule2: {}", rule);

        let res = rule
            .split(' ')
            .map(|i| generate_regex(i.parse::<u32>().unwrap(), rules))
            .collect::<String>();
        println!("Res2: {}", res);

        return res;
    } else if rule.split(' ').count() == 5 {
        println!("rule5: {}", rule);

        let res = rule
            .split('|')
            .map(|b| {
                println!("B: {}", b);
                let res = b
                    .trim()
                    .split(' ')
                    .map(|lr| {
                        println!("lr: {}", lr);
                        return generate_regex(lr.parse::<u32>().unwrap(), rules);
                    })
                    .collect::<String>();

                println!("b: {}", res);
                return res;
            })
            .join("|");

        println!("Res5: {}", res);
        return String::from("(") + &res + ")";
    } else if rule.split(' ').count() == 1 {
        return generate_regex(rule.trim().parse::<u32>().unwrap(), rules);
    } else if rule.split(' ').count() == 3 {
        println!("rule3: {}", rule);

        let res = rule
            .split('|')
            .map(|b| {
                return generate_regex(b.trim().parse::<u32>().unwrap(), rules);
            })
            .join("|");

        println!("Res3: {}", res);
        return String::from("(") + &res + ")";
    } else if index == 8 {
        let counts = MAX_8.fetch_add(1, Ordering::SeqCst);

        if counts < 10 {
            return String::from("(")
                + &generate_regex(42, rules)
                + "|"
                + &generate_regex(42, rules)
                + &generate_regex(8, rules)
                + ")";
        } else {
            return generate_regex(42, rules);
        }
    } else if index == 11 {
        let counts = MAX_11.fetch_add(1, Ordering::SeqCst);

        if counts < 10 {
            return String::from("(")
                + &generate_regex(42, rules)
                + &generate_regex(31, rules)
                + "|"
                + &generate_regex(42, rules)
                + &generate_regex(11, rules)
                + &generate_regex(31, rules)
                + ")";
        } else {
            return generate_regex(42, rules) + &generate_regex(31, rules);
        }
    } else {
        println!("Rule unimplemented: {}", rule);
        unimplemented!();
    }
}

#[test]
fn test_generate_regex() {
    let mut test_map = HashMap::new();
    test_map.insert(0, "4 1".to_string());
    test_map.insert(1, "2 3 | 3 2".to_string());
    test_map.insert(2, "4 4 | 5 5".to_string());
    test_map.insert(3, "4 5 | 5 4".to_string());
    test_map.insert(4, "\"a\"".to_string());
    test_map.insert(5, "\"b\"".to_string());

    assert_eq!(
        generate_regex(0, &test_map),
        "a((aa|bb)(ab|ba)|(ab|ba)(aa|bb))"
    );
}
