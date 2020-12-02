use std::fs;

fn main() {
    let mut input: Vec<&str> = r"1721
    979
    366
    299
    675
    1456"
        .lines()
        .collect();

    let real_input = fs::read_to_string("input/first.txt").expect("File dead");

    input = real_input.lines().collect();

    let target = 2020;

    for pos1 in 0..input.len() {
        // Walk the same list but from one index further to get all 2-combinations
        for pos2 in (0..input.len()).skip(pos1 + 1) {
            let one = input[pos1].trim().parse::<i32>().unwrap();
            let other = input[pos2].trim().parse::<i32>().unwrap();

            if one + other == target {
                println!(
                    "{} + {} = {} so the answer is {}",
                    one,
                    other,
                    target,
                    one * other
                )
            }

            // Walk one further to select three numbers
            for pos3 in (0..input.len()).skip(pos2 + 1) {
                let third = input[pos3].trim().parse::<i32>().unwrap();

                if one + other + third == target {
                    println!(
                        "{} + {} + {} = {} so the answer is {}",
                        one,
                        other,
                        third,
                        target,
                        one * other * third
                    );
                }
            }
        }
    }
}
