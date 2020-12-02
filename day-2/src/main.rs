use std::fs;

fn main() {
    // let input = r"1-3 a: abcde
    // 1-3 b: cdefg
    // 2-9 c: ccccccccc"
    //     .lines();

    let input = fs::read_to_string("input.txt").expect("Dead file");

    let mut count_count = 0;
    let mut pos_count = 0;

    for line in input.lines() {
        let parts: Vec<&str> = line.trim().split_whitespace().collect();

        let count_parts: Vec<&str> = parts[0].split("-").collect();
        let mi = count_parts[0].parse::<i32>().unwrap();
        let ma = count_parts[1].parse::<i32>().unwrap();

        let letter = parts[1].trim().chars().next().unwrap();

        let pass = parts[2].trim();

        if check_count(mi, ma, letter, pass) {
            println!("Valid password: {}", pass);

            count_count += 1;
        }

        if check_pos(mi, ma, letter, pass) {
            pos_count += 1;
        }
    }
    println!("Number with correct count: {}", count_count);
    println!("Number with correct pos: {}", pos_count);
}

fn check_count(minimum: i32, maximum: i32, letter: char, password: &str) -> bool {
    let n = password.chars().filter(|c| *c == letter).count() as i32;

    return n >= minimum && n <= maximum;
}

fn check_pos(pos1: i32, pos2: i32, letter: char, password: &str) -> bool {
    (password.chars().collect::<Vec<char>>()[(pos1 - 1) as usize] == letter)
        ^ (password.chars().collect::<Vec<char>>()[(pos2 - 1) as usize] == letter)
}
