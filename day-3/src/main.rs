use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Dead file");
    let mut data: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        data.push(line.trim().chars().collect());
    }

    // 1 down, 3 right
    let sled = (1, 3);

    let mut pos = (0, 0);
    let band_width = data[0].len();

    println!("Field height: {} and width: {}", data.len(), band_width);

    let mut count = 0;

    while pos.0 < data.len() {
        println!("Pos: {:?}", pos);

        let landing = data[pos.0][pos.1 % band_width];

        if landing == '#' {
            count += 1;
        }

        pos = (pos.0 + sled.0, pos.1 + sled.1);
    }

    println!("Data: {:?}", data);
    println!("Counts: {}", count);
}

fn tree_count(jump: (i32, i32), data: Vec<Vec<char>>) -> i32 {}
