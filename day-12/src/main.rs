use std::fs;

fn main() {
    part1();
    part2();
}

fn part1() {
    let _test_input = r"F10
N3
F7
R90
F11";

    let input = fs::read_to_string("input.txt").expect("Dead file");

    let actions = input.lines().map(|l| {
        // println!("/{}/", l);
        let val = l[1..].trim().parse::<i32>().unwrap();

        match &l[..1] {
            "N" => Action::N(val),
            "E" => Action::E(val),
            "S" => Action::S(val),
            "W" => Action::W(val),
            "L" => Action::L(val),
            "R" => Action::R(val),
            "F" => Action::F(val),
            _ => Action::F(0)
        }
    });

    let mut ship = Ship {dir: Direction::E, x: 0, y: 0};
    for action in actions {
        println!("Action: {:?}", action);

        match action {
            Action::N(n) => ship.y -= n,
            Action::S(n) => ship.y += n,
            Action::E(n) => ship.x += n,
            Action::W(n) => ship.x -= n,
            Action::F(n) => {
                match ship.dir {
                    Direction::N => ship.y -= n,
                    Direction::S => ship.y += n,
                    Direction::E => ship.x += n,
                    Direction::W => ship.x -= n,
                }
            },
            Action::R(deg) => ship.dir = ship.dir.get_direction(Action::R(deg)),
            Action::L(deg) => ship.dir = ship.dir.get_direction(Action::L(deg))
        }
        println!("Ship: {:?}", ship);
    }

    println!("Answer part 1: {}", ship.x.abs() + ship.y.abs());
}

fn part2() {
    let _test_input = r"F10
N3
F7
R90
F11";

    let input = fs::read_to_string("input.txt").expect("Dead file");

    let actions = input.lines().map(|l| {
        // println!("/{}/", l);
        let val = l[1..].trim().parse::<i32>().unwrap();

        match &l[..1] {
            "N" => Action::N(val),
            "E" => Action::E(val),
            "S" => Action::S(val),
            "W" => Action::W(val),
            "L" => Action::L(val),
            "R" => Action::R(val),
            "F" => Action::F(val),
            _ => Action::F(0)
        }
    });

    let mut ship = Ship {dir: Direction::E, x: 0, y: 0};
    let mut waypoint = WayPoint {x: 10, y: -1};

    for action in actions {
        println!("Action: {:?}", action);

        match action {
            Action::N(n) => waypoint.y -= n,
            Action::S(n) => waypoint.y += n,
            Action::E(n) => waypoint.x += n,
            Action::W(n) => waypoint.x -= n,
            Action::F(n) => {
                ship.x += waypoint.x * n;
                ship.y += waypoint.y * n;
            },
            Action::R(deg) => waypoint = waypoint.rotate(Action::R(deg)),
            Action::L(deg) => waypoint = waypoint.rotate(Action::L(deg)),
        }
        println!("Ship: {:?}", ship);
        println!("Waypoint: {:?}", waypoint);
    }

    println!("Answer part 2: {}", ship.x.abs() + ship.y.abs());
}

#[derive(Debug)]
enum Action {
    N(i32),
    E(i32),
    S(i32),
    W(i32),
    L(i32),
    R(i32),
    F(i32)
}

#[derive(Debug)]
enum Direction {
    N,
    E,
    S,
    W
}

impl Direction {
    fn get_direction(&self, act: Action) -> Direction {
        let rotation = match act {
            Action::L(deg) => 360 - deg,
            Action::R(deg) => deg,
            _ => 0
        };

        match self {
            Direction::N => {
                match rotation {
                    90 => Direction::E,
                    180 => Direction::S,
                    270 => Direction::W,
                    _ => Direction::N
                }
            }
            Direction::E => {
                match rotation {
                    90 => Direction::S,
                    180 => Direction::W,
                    270 => Direction::N,
                    _ => Direction::E
                }
            }
            Direction::S => {
                match rotation {
                    90 => Direction::W,
                    180 => Direction::N,
                    270 => Direction::E,
                    _ => Direction::S
                }
            }
            Direction::W => {
                match rotation {
                    90 => Direction::N,
                    180 => Direction::E,
                    270 => Direction::S,
                    _ => Direction::W
                }
            }
        }
    }
}

#[derive(Debug)]
struct Ship {
    x: i32,
    y: i32,
    dir: Direction,
}

#[derive(Debug)]
struct WayPoint {
    x: i32,
    y: i32,
}

impl PartialEq for WayPoint {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for WayPoint{}

impl WayPoint {
    fn rotate(&self, act: Action) -> WayPoint {
        let rotation = match act {
            Action::L(deg) => 360 - deg,
            Action::R(deg) => deg,
            _ => 0
        };

        match rotation {
            90 => WayPoint{x: -self.y, y: self.x},
            180 => WayPoint{x: -self.x, y: -self.y},
            270 => WayPoint{x: self.y, y: -self.x},
            _ => WayPoint{x: self.x, y: self.y},
        }
    }
}

#[test]
fn test_rotate() {
    assert_eq!(WayPoint{x: 10, y: -4}.rotate(Action::R(90)), WayPoint{x: 4, y: 10});
}