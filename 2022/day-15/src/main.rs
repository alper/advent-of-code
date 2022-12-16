use std::{
    cmp::{max, min},
    collections::VecDeque,
    fmt::Display,
    fs,
    ops::Range,
    str::FromStr,
};

use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map_res, opt, recognize},
    sequence::{preceded, tuple},
    IResult,
};
use rayon::prelude::*;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{} {}]", self.x, self.y)
    }
}

fn parse_isize(input: &str) -> IResult<&str, isize> {
    let (i, number) = map_res(recognize(preceded(opt(tag("-")), digit1)), |s| {
        isize::from_str(s)
    })(input)?;

    Ok((i, number))
}

fn parse_line(input: &str) -> IResult<&str, (Point, Point)> {
    let (i, (_, sx, _, sy, _, bx, _, by)) = tuple((
        tag("Sensor at x="),
        parse_isize,
        tag(", y="),
        parse_isize,
        tag(": closest beacon is at x="),
        parse_isize,
        tag(", y="),
        parse_isize,
    ))(input)?;

    Ok((i, (Point { x: sx, y: sy }, Point { x: bx, y: by })))
}

fn dist(p1: &Point, p2: &Point) -> isize {
    (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
}

fn normalize(p1: &Point, p2: &Point) -> Point {
    Point {
        x: p2.x - p1.x,
        y: p2.y - p1.y,
    }
}

fn in_origin_ball(dist: isize, point: &Point) -> bool {
    point.x.abs() + point.y.abs() <= dist
}

fn in_range(sensor: &Point, beacon: &Point, location: &Point) -> bool {
    // println!("In range: {} -> {} / {}", sensor, beacon, location);
    // let origin = Point { x: 0, y: 0 };
    // let normalized_beacon = normalize(sensor, beacon);
    let normalized_location = normalize(sensor, location);
    let dist = dist(sensor, beacon);

    // println!("Normalized: {} {}", normalized_beacon, normalized_location);
    // println!("Distance: {}", dist);

    in_origin_ball(dist, &normalized_location)
}

fn get_ranges(
    y: isize,
    x_range: &Range<isize>,
    sensor_beacons: &Vec<(Point, Point)>,
) -> Vec<(isize, isize)> {
    let mut ranges = vec![];

    for (sensor, beacon) in sensor_beacons {
        let distance = dist(sensor, beacon);

        let distanceToSensor = (y - sensor.y).abs();
        let offset_x = distance - distanceToSensor;
        let width = max(offset_x * 2 + 1, 0);

        if width > 0 {
            let from = sensor.x - offset_x;
            let to = sensor.x + offset_x;

            if to >= x_range.start && from <= x_range.end {
                ranges.push((max(x_range.start, from), min(x_range.end, to)));
            }
        }
    }
    ranges.sort();

    ranges
}

fn merge_ranges(ranges: &Vec<(isize, isize)>) -> Vec<(isize, isize)> {
    let mut ranges = VecDeque::from(ranges.clone());
    let mut merged = VecDeque::new();

    merged.push_back(ranges.pop_front().unwrap());

    for (next_from, next_to) in ranges {
        let (prev_from, mut prev_to) = merged.pop_back().unwrap();

        if next_from <= prev_to && next_to <= prev_to {
            // Do nothing since it's entirely in

            merged.push_back((prev_from, prev_to));
        } else if next_from <= prev_to + 1 {
            merged.push_back((prev_from, max(prev_to, next_to)));
        } else {
            merged.push_back((prev_from, prev_to));
            merged.push_back((next_from, next_to));
        }

        // println!("Merged: {:?}", merged);
    }

    merged.into()
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let full = true;

    let input = if full {
        fs::read_to_string("full_input.txt").expect("File not readable")
    } else {
        fs::read_to_string("test_input.txt").expect("File not readable")
    };

    let sensor_beacons: Vec<_> = input.lines().map(|l| parse_line(l).unwrap().1).collect();
    println!("{:?}", sensor_beacons);

    // Part 1
    println!("Part 1");

    // For each candidate point
    //   For each sensor, beacon combo
    //     Check if the candidate point is in the range of the sensor
    let candidate_y = if full { 2000000isize } else { 10 };
    let candidate_x_range = if full {
        -700_000..4_300_000isize
    } else {
        -10..30isize
    };

    let points_in_range = candidate_x_range
        .clone()
        .into_par_iter()
        .map(|candidate_x| Point {
            x: candidate_x,
            y: candidate_y,
        })
        .map(|point| {
            sensor_beacons.iter().any(move |(sensor, beacon)| {
                in_range(sensor, beacon, &point) && *sensor != point && *beacon != point
            })
        })
        .filter(|&p| p)
        .count();

    println!("Answer part 1: {:?}", points_in_range);

    // Part 2
    println!("Part 2");

    // For each candidate point
    //   For each sensor, beacon combo
    //     Check if the candidate point is in the range of the sensor
    let candidate_y_range = if full { 0..4_000_001isize } else { 0..21isize };
    let candidate_x_range = if full { 0..4_000_001isize } else { 0..21isize };

    for y in candidate_y_range {
        let ranges = get_ranges(y, &candidate_x_range, &sensor_beacons);
        let merged = merge_ranges(&ranges);

        if merged.len() == 2 {
            println!("Merged: {:?}", merged);
            let x = merged.first().unwrap().1 + 1;
            println!("Answer part 2: {:?}", 4000000 * x + y);
        }
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
