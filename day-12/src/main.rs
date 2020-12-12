fn main() {
    let test_input = r"F10
N3
F7
R90
F11";

    let mut ship = Ship {dir: Direction::E, x: 0, y: 0};

    let actions = test_input.lines().map(|l| {
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
                    90 => return Direction::E,
                    180 => return Direction::S,
                    270 => return Direction::W,
                    _ => return Direction::N
                }
            }
            Direction::E => {
                match rotation {
                    90 => return Direction::S,
                    180 => return Direction::W,
                    270 => return Direction::N,
                    _ => return Direction::E
                }
            }
            Direction::S => {
                match rotation {
                    90 => return Direction::W,
                    180 => return Direction::N,
                    270 => return Direction::E,
                    _ => Direction::S
                }
            }
            Direction::W => {
                match rotation {
                    90 => return Direction::N,
                    180 => return Direction::E,
                    270 => return Direction::S,
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
    dir: Direction
}