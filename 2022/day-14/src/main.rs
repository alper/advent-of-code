use std::{
    cmp::{max, min},
    fs,
};

use array2d::Array2D;
use nom::{
    bytes::complete::tag, character::complete::digit1, multi::separated_list1, sequence::tuple,
    IResult,
};

fn parse_number_list(input: &str) -> IResult<&str, Vec<(usize, usize)>> {
    let coords = tuple((digit1, tag(","), digit1));
    let (i, r) = separated_list1(tag(" -> "), coords)(input)?;

    Ok((
        i,
        r.iter()
            .map(|&(x, _, y)| {
                (
                    str::parse::<usize>(x).unwrap(),
                    str::parse::<usize>(y).unwrap(),
                )
            })
            .collect(),
    ))
}

fn print_array(array: &Array2D<char>) {
    for row in array.as_rows() {
        println!("{}", row.iter().collect::<String>());
    }
}

fn generated_path(start: (usize, usize), end: (usize, usize)) -> Vec<(usize, usize)> {
    // println!("Genrate for: {:?} {:?}", start, end);

    if start.0 == end.0 {
        (min(start.1, end.1)..(max(start.1, end.1) + 1))
            .map(|y| (start.0, y))
            .collect()
    } else {
        (min(start.0, end.0)..(max(start.0, end.0)) + 1)
            .map(|x| (x, start.1))
            .collect()
    }
}

fn drop_sand(array: &Array2D<char>, sand_start: (usize, usize)) -> Option<(usize, usize)> {
    let mut sand_pos = sand_start;

    loop {
        match array.get(sand_pos.1 + 1, sand_pos.0) {
            Some('.') => {
                sand_pos = (sand_pos.0, sand_pos.1 + 1);
            }
            Some('#') => match array.get(sand_pos.1 + 1, sand_pos.0 - 1) {
                Some('.') => {
                    sand_pos = (sand_pos.0 - 1, sand_pos.1 + 1);
                }
                Some('#') => match array.get(sand_pos.1 + 1, sand_pos.0 + 1) {
                    Some('.') => {
                        sand_pos = (sand_pos.0 + 1, sand_pos.1 + 1);
                    }
                    Some('#') => {
                        return Some(sand_pos);
                    }
                    Some(_) => todo!(),
                    None => return None,
                },
                Some(_) => todo!(),
                None => return None,
            },
            Some(_) => todo!(),
            None => return None,
        }
    }
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let input = fs::read_to_string("full_input.txt").expect("File not readable");

    let rocks: Vec<Vec<(usize, usize)>> = input
        .lines()
        .map(|l| parse_number_list(l).unwrap().1)
        .collect();

    let min_x = rocks
        .iter()
        .map(|l| l.iter().map(|c| c.0).min().unwrap())
        .min()
        .unwrap();
    let max_x = rocks
        .iter()
        .map(|l| l.iter().map(|c| c.0).max().unwrap())
        .max()
        .unwrap();

    let min_y = rocks
        .iter()
        .map(|l| l.iter().map(|c| c.1).min().unwrap())
        .min()
        .unwrap();
    let max_y = rocks
        .iter()
        .map(|l| l.iter().map(|c| c.1).max().unwrap())
        .max()
        .unwrap();

    let start_x = min_x - 5;

    // println!("{:?}", rocks);
    // println!("X {min_x}-{max_x}, Y {min_y}-{max_y}");

    // Part 1
    println!("Part 1");

    let mut array_part1 = Array2D::filled_with('.', max_y + 1, max_x - start_x + 5);

    let sand_start = (500 - start_x, 0);
    let _ = array_part1.set(sand_start.1, sand_start.0, '+');

    for rock in rocks.clone() {
        let segments = rock.windows(2);

        for segment in segments {
            let path_segment = generated_path(segment[0], segment[1]);

            // println!("Segment: {:?}", path_segment);

            for point in path_segment {
                let _ = array_part1.set(point.1, point.0 - start_x, '#');
            }
        }
    }

    let mut counter = 0;

    while let Some(end_pos) = drop_sand(&array_part1, (sand_start.0, sand_start.1)) {
        counter += 1;

        let _ = array_part1.set(end_pos.1, end_pos.0, '#');
    }

    // print_array(&array_part1);

    println!("Answer part 1: {:?}", counter);

    // Part 2
    println!("Part 2");

    let mut array_part2 = Array2D::filled_with('.', max_y + 3, max_x + 500);

    let sand_start = (500, 0);
    let _ = array_part2.set(sand_start.1, sand_start.0, '+');

    // Add bottom
    for i in 0..array_part2.num_columns() {
        let _ = array_part2.set(array_part2.num_rows() - 1, i, '#');
    }

    for rock in rocks.clone() {
        let segments = rock.windows(2);

        for segment in segments {
            let path_segment = generated_path(segment[0], segment[1]);

            // println!("Segment: {:?}", path_segment);

            for point in path_segment {
                let _ = array_part2.set(point.1, point.0, '#');
            }
        }
    }
    // println!("Array ready");
    // print_array(&array_part2);

    let mut counter2 = 0;

    while let Some(end_pos) = drop_sand(&array_part2, (sand_start.0, sand_start.1)) {
        if end_pos == sand_start {
            break;
        }
        counter2 += 1;

        let _ = array_part2.set(end_pos.1, end_pos.0, '#');
    }

    // println!("Array done");
    // print_array(&array_part2);

    println!("Answer part 2: {:?}", counter2 + 1);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
