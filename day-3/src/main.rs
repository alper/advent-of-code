use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Dead file");
    let mut data: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        data.push(line.trim().chars().collect());
    }

    // 1 down, 3 right
    let sled = (1, 3);
    let count = tree_count(sled, &data);

    println!("Count for {:?}: {}", sled, count);

    // Part 2
    // Courses
    let courses = vec![(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];

    let multiplication: i64 = courses
        .into_iter()
        .map(|jump| tree_count(jump, &data))
        .fold(1, |acc, el| acc * (el as i64));

    println!("Product of all the results: {:?}", multiplication);
}

fn tree_count(jump: (usize, usize), data: &[Vec<char>]) -> i32 {
    let mut pos = (0, 0);
    let band_width = data[0].len();

    // println!("Field height: {} and width: {}", data.len(), band_width);

    let mut count = 0;

    while pos.0 < data.len() {
        // println!("Pos: {:?}", pos);

        let landing = data[pos.0][pos.1 % band_width];

        if landing == '#' {
            count += 1;
        }

        pos = (pos.0 + jump.0, pos.1 + jump.1);
    }

    count
}
