use std::{fmt::Display, fs, str::FromStr};

use itertools::Itertools;
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

fn dist(p1: Point, p2: Point) -> isize {
    (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
}

fn normalize(p1: Point, p2: Point) -> Point {
    Point {
        x: p2.x - p1.x,
        y: p2.y - p1.y,
    }
}

fn in_origin_ball(dist: isize, point: Point) -> bool {
    point.x.abs() + point.y.abs() <= dist
}

fn in_range(sensor: Point, beacon: Point, location: Point) -> bool {
    // println!("In range: {} -> {} / {}", sensor, beacon, location);
    // let origin = Point { x: 0, y: 0 };
    // let normalized_beacon = normalize(sensor, beacon);
    let normalized_location = normalize(sensor, location);
    let dist = dist(sensor, beacon);

    // println!("Normalized: {} {}", normalized_beacon, normalized_location);
    // println!("Distance: {}", dist);

    in_origin_ball(dist, normalized_location)
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
    let mut covered_count = 0;

    let points_in_range = candidate_x_range
        .clone()
        .into_par_iter()
        .map(|candidate_x| Point {
            x: candidate_x,
            y: candidate_y,
        })
        .map(|point| {
            sensor_beacons.iter().any(move |(sensor, beacon)| {
                in_range(*sensor, *beacon, point) && *sensor != point && *beacon != point
            })
        })
        .filter(|&p| p)
        .count();

    // println!("Points in range {}", points_in_range);

    // for candidate_x in candidate_x_range {
    //     let candidate_point = Point {
    //         x: candidate_x,
    //         y: candidate_y,
    //     };
    //     // println!("Candidate point: {}", candidate_point);
    //     let mut covered = false;

    //     for (sensor, beacon) in &sensor_beacons {
    //         if in_range(*sensor, *beacon, candidate_point)
    //             && sensor_beacons
    //                 .iter()
    //                 .filter(|&(s, b)| *s == candidate_point || *b == candidate_point)
    //                 .count()
    //                 == 0
    //         {
    //             covered = true;
    //             covered_count += 1;
    //             break;
    //         }
    //     }
    //     println!("Location: {} is covered: {}", candidate_point, covered);
    // }

    println!("Answer part 1: {:?}", points_in_range);

    // Part 2
    println!("Part 2");

    // For each candidate point
    //   For each sensor, beacon combo
    //     Check if the candidate point is in the range of the sensor
    let candidate_y_range = if full { 0..4_000_001isize } else { 0..21isize };
    let candidate_x_range = if full { 0..4_000_001isize } else { 0..21isize };

    let candidate_points =
        candidate_x_range
            .cartesian_product(candidate_y_range)
            .map(|(candidate_x, candidate_y)| Point {
                x: candidate_x,
                y: candidate_y,
            });

    for candidate in candidate_points {
        if candidate.y % 100_000 == 0 {
            println!("Trying out candidate: {}", candidate)
        }
        let mut point_free = true;

        for (sensor, beacon) in &sensor_beacons {
            // Check whether a sensor or beacon coincides with the point
            if *sensor == candidate || *beacon == candidate {
                point_free = false;
            }

            // Check whether the sensor can see the point
            if in_range(*sensor, *beacon, candidate) {
                point_free = false;
            }
        }

        if point_free {
            println!("Point {} is free", candidate);
            println!("Answer part 2: {:?}", candidate.x * 4_000_000 + candidate.y);
        }
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
