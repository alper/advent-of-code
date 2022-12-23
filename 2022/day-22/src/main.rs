use array2d::Array2D;
use nom::{branch::alt, bytes::complete::tag, character::complete::digit1, multi::many1, IResult};
use std::{fs, ops};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

impl ops::Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    N,
    E,
    S,
    W,
}

impl ops::Add<Move> for Direction {
    type Output = Direction;

    fn add(self, rhs: Move) -> Self::Output {
        match self {
            Direction::N => match rhs {
                Move::L => Direction::W,
                Move::R => Direction::E,
                Move::Walk(_) => self,
            },
            Direction::E => match rhs {
                Move::L => Direction::N,
                Move::R => Direction::S,
                Move::Walk(_) => self,
            },
            Direction::S => match rhs {
                Move::L => Direction::E,
                Move::R => Direction::W,
                Move::Walk(_) => self,
            },
            Direction::W => match rhs {
                Move::L => Direction::S,
                Move::R => Direction::N,
                Move::Walk(_) => self,
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Move {
    L,
    R,
    Walk(usize),
}

fn print_array(array: &Array2D<char>) {
    for row in array.as_rows() {
        println!("{}", row.iter().collect::<String>());
    }
}

fn move_as_far_as_possible(
    field: &Array2D<char>,
    start: Point,
    dir: Direction,
    steps: usize,
) -> Point {
    match dir {
        Direction::N => {
            let x = start.x;
            let mut y = start.y;

            let reentry_y = field
                .as_columns()
                .iter()
                .nth(x.try_into().unwrap())
                .unwrap()
                .iter()
                .enumerate()
                .filter(|&(_, c)| *c == '.' || *c == '#')
                .last()
                .unwrap()
                .0;

            for _ in 0..steps {
                y -= 1;

                match field.get(
                    y.try_into().map_or(usize::MAX, |y| y),
                    x.try_into().unwrap(),
                ) {
                    Some('.') => {}
                    Some('#') => return Point { x: x, y: y + 1 },
                    Some(' ') | None => {
                        if *field
                            .get(reentry_y.try_into().unwrap(), x.try_into().unwrap())
                            .unwrap()
                            != '#'
                        {
                            y = reentry_y as isize;
                        } else {
                            return Point { x: x, y: y + 1 };
                        }
                    }
                    _ => todo!(),
                }
            }
            // We made it through
            return Point { x: x, y: y };
        }
        Direction::E => {
            let mut x = start.x;
            let y = start.y;

            let reentry_x = field
                .as_rows()
                .iter()
                .nth(y.try_into().unwrap())
                .unwrap()
                .iter()
                .enumerate()
                .filter(|&(_, c)| *c == '.' || *c == '#')
                .next()
                .unwrap()
                .0;

            for _ in 0..steps {
                x += 1;

                match field.get(y.try_into().unwrap(), x.try_into().unwrap()) {
                    Some('.') => {}
                    Some('#') => return Point { x: x - 1, y: y },
                    Some(' ') | None => {
                        if *field
                            .get(y.try_into().unwrap(), reentry_x.try_into().unwrap())
                            .unwrap()
                            != '#'
                        {
                            x = reentry_x as isize;
                        } else {
                            return Point { x: x - 1, y: y };
                        }
                    }
                    _ => todo!(),
                }
            }
            // We made it through
            return Point { x: x, y: y };
        }
        Direction::S => {
            let x = start.x;
            let mut y = start.y;

            let reentry_y = field
                .as_columns()
                .iter()
                .nth(x.try_into().unwrap())
                .unwrap()
                .iter()
                .enumerate()
                .filter(|&(_, c)| *c == '.' || *c == '#')
                .next()
                .unwrap()
                .0;

            for _ in 0..steps {
                y += 1;

                match field.get(y.try_into().unwrap(), x.try_into().unwrap()) {
                    Some('.') => {}
                    Some('#') => return Point { x: x, y: y - 1 },
                    Some(' ') | None => {
                        if *field
                            .get(reentry_y.try_into().unwrap(), x.try_into().unwrap())
                            .unwrap()
                            != '#'
                        {
                            y = reentry_y as isize;
                        } else {
                            return Point { x: x, y: y - 1 };
                        }
                    }
                    _ => todo!(),
                }
            }
            // We made it through
            return Point { x: x, y: y };
        }
        Direction::W => {
            let mut x = start.x;
            let y = start.y;

            let reentry_x = field
                .as_rows()
                .iter()
                .nth(y.try_into().unwrap())
                .unwrap()
                .iter()
                .enumerate()
                .filter(|&(_, c)| *c == '.' || *c == '#')
                .last()
                .unwrap()
                .0;

            for _ in 0..steps {
                x -= 1;

                match field.get(
                    y.try_into().unwrap(),
                    x.try_into().map_or(usize::MAX, |x| x),
                ) {
                    Some('.') => {}
                    Some('#') => return Point { x: x + 1, y: y },
                    Some(' ') | None => {
                        if *field
                            .get(y.try_into().unwrap(), reentry_x.try_into().unwrap())
                            .unwrap()
                            != '#'
                        {
                            x = reentry_x as isize;
                        } else {
                            return Point { x: x + 1, y: y };
                        }
                    }
                    _ => todo!(),
                }
            }
            // We made it through
            return Point { x: x, y: y };
        }
    }
}

fn parse_moves(input: &str) -> IResult<&str, Vec<Move>> {
    let (i, v) = many1(alt((digit1, tag("R"), tag("L"))))(input)?;

    Ok((
        i,
        v.iter()
            .map(|&s| match s {
                "L" => Move::L,
                "R" => Move::R,
                n => Move::Walk(str::parse::<usize>(n).unwrap()),
            })
            .collect(),
    ))
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let input = fs::read_to_string("full_input.txt").expect("File not readable");

    let mut field_lines = vec![];
    let mut moves: Vec<Move> = Vec::new();

    let mut blank_seen = false;
    for line in input.lines() {
        if line.is_empty() {
            blank_seen = true;
            continue;
        }

        if !blank_seen {
            field_lines.push(line);
        } else {
            moves = parse_moves(line).unwrap().1;
        }
    }

    // println!("Lines: {:?}", field_lines);
    println!("Moves: {:?}", moves);

    let mut field = Array2D::filled_with(
        ' ',
        field_lines.len(),
        field_lines.iter().map(|l| l.len()).max().unwrap(),
    );

    for (i, line) in field_lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            let _ = field.set(i, j, c);
        }
    }

    let start_x = field_lines[0]
        .chars()
        .enumerate()
        .filter(|&(_, c)| c == '.')
        .next()
        .unwrap()
        .0;

    let mut pos = Point {
        x: start_x as isize,
        y: 0,
    };
    let mut heading = Direction::E;

    // field.set(
    //     start_pos.y.try_into().unwrap(),
    //     start_pos.x.try_into().unwrap(),
    //     'V',
    // );

    print_array(&field);

    // println!(
    //     "{:?}",
    //     move_as_far_as_possible(&field, pos, Direction::N, 5)
    // );

    for m in moves {
        match m {
            Move::L => heading = heading + m,
            Move::R => heading = heading + m,
            Move::Walk(n) => pos = move_as_far_as_possible(&field, pos, heading, n),
        }
    }

    println!("End pos: {:?}", pos);
    println!(
        "End score: {:?}",
        1000 * (pos.y + 1)
            + 4 * (pos.x + 1)
            + match heading {
                Direction::N => 3,
                Direction::E => 0,
                Direction::S => 1,
                Direction::W => 2,
            }
    );

    // let v: Vec<_> = input.lines().collect();
    // println!("{:?}", v);

    // Part 1
    println!("Part 1");

    println!("Answer part 1: {:?}", "");

    // Part 2
    println!("Part 2");

    // Reddit: https://www.reddit.com/r/adventofcode/comments/zsct8w/2022_day_22_solutions/?sort=confidence
    // Yeah fuck that.

    println!("Answer part 2: {:?}", "");

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
