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
    println!("Genrate for: {:?} {:?}", start, end);

    if start.0 == end.0 {
        return (min(start.1, end.1)..(max(start.1, end.1) + 1))
            .map(|y| (start.0, y))
            .collect();
    } else {
        return (min(start.0, end.0)..(max(start.0, end.0)) + 1)
            .map(|x| (x, start.1))
            .collect();
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

    println!("{:?}", rocks);
    println!("X {min_x}-{max_x}, Y {min_y}-{max_y}");

    // Part 1
    println!("Part 1");

    let mut array = Array2D::filled_with('.', max_y + 1, max_x + 1);

    for rock in rocks {
        let segments = rock.windows(2);

        for segment in segments {
            let path_segment = generated_path(segment[0], segment[1]);

            println!("Segment: {:?}", path_segment);

            for point in path_segment {
                let res = array.set(point.1, point.0, '#');

                println!("Result: {:?}", res);
            }
        }
    }

    print_array(&array);

    println!("Answer part 1: {:?}", "");

    // Part 2
    println!("Part 2");

    println!("Answer part 2: {:?}", "");

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
