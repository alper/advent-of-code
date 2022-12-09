use std::{collections::HashSet, fs};

use itertools::Itertools;

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct Pos {
    x: isize,
    y: isize,
}

// Grid to the top right
fn head_move(pos: Pos, step: Step) -> Pos {
    match step {
        Step::Right => Pos {
            x: pos.x + 1,
            y: pos.y,
        },
        Step::Left => Pos {
            x: pos.x - 1,
            y: pos.y,
        },
        Step::Up => Pos {
            x: pos.x,
            y: pos.y + 1,
        },
        Step::Down => Pos {
            x: pos.x,
            y: pos.y - 1,
        },
    }
}

fn tail_move(head_pos: Pos, tail_pos: Pos) -> Pos {
    // 90 degree shift

    if head_pos.x - tail_pos.x == 2 && head_pos.y == tail_pos.y {
        return Pos {
            x: tail_pos.x + 1,
            y: tail_pos.y,
        };
    }

    if head_pos.x - tail_pos.x == -2 && head_pos.y == tail_pos.y {
        return Pos {
            x: tail_pos.x - 1,
            y: tail_pos.y,
        };
    }

    if head_pos.y - tail_pos.y == 2 && head_pos.x == tail_pos.x {
        return Pos {
            x: tail_pos.x,
            y: tail_pos.y + 1,
        };
    }

    if head_pos.y - tail_pos.y == -2 && head_pos.x == tail_pos.x {
        return Pos {
            x: tail_pos.x,
            y: tail_pos.y - 1,
        };
    }

    // Horse jump shift
    // Horizontal 2 - vertical 1
    if head_pos.x - tail_pos.x == 2 && head_pos.y - tail_pos.y == 1 {
        return Pos {
            x: tail_pos.x + 1,
            y: tail_pos.y + 1,
        };
    }

    if head_pos.x - tail_pos.x == 2 && head_pos.y - tail_pos.y == -1 {
        return Pos {
            x: tail_pos.x + 1,
            y: tail_pos.y - 1,
        };
    }

    if head_pos.x - tail_pos.x == -2 && head_pos.y - tail_pos.y == 1 {
        return Pos {
            x: tail_pos.x - 1,
            y: tail_pos.y + 1,
        };
    }

    if head_pos.x - tail_pos.x == -2 && head_pos.y - tail_pos.y == -1 {
        return Pos {
            x: tail_pos.x - 1,
            y: tail_pos.y - 1,
        };
    }

    // Vertical 2 - horizontal 1

    if head_pos.y - tail_pos.y == 2 && head_pos.x - tail_pos.x == 1 {
        return Pos {
            x: tail_pos.x + 1,
            y: tail_pos.y + 1,
        };
    }

    if head_pos.y - tail_pos.y == 2 && head_pos.x - tail_pos.x == -1 {
        return Pos {
            x: tail_pos.x - 1,
            y: tail_pos.y + 1,
        };
    }

    if head_pos.y - tail_pos.y == -2 && head_pos.x - tail_pos.x == 1 {
        return Pos {
            x: tail_pos.x + 1,
            y: tail_pos.y - 1,
        };
    }

    if head_pos.y - tail_pos.y == -2 && head_pos.x - tail_pos.x == -1 {
        return Pos {
            x: tail_pos.x - 1,
            y: tail_pos.y - 1,
        };
    }

    // Coinciding or diagonal touching
    return tail_pos;
}

#[derive(Debug, Copy, Clone)]
enum Step {
    Right,
    Left,
    Up,
    Down,
}

fn do_step(head: Pos, tail: Pos, step: Step) -> (Pos, Pos) {
    println!("Do step: {:?}", step);

    let new_head = head_move(head, step);
    let new_tail = tail_move(new_head, tail);

    println!("New Head: {:?}, New Tail: {:?}", new_head, new_tail);
    // println!("Head: {:?}", new_head);
    // println!("Tail: {:?}", new_tail);

    (new_head, new_tail)
}

fn main() {
    let input = fs::read_to_string("full_input.txt").expect("File not readable");

    // Part 1
    println!("Part 1");

    let mut visited: HashSet<Pos> = HashSet::new();

    let mut head = Pos { x: 0, y: 0 };
    let mut tail = Pos { x: 0, y: 0 };

    visited.insert(tail);

    for l in input.lines() {
        let (dir, amount) = l.split(' ').collect_tuple().unwrap();

        let step = match dir {
            "U" => Step::Up,
            "D" => Step::Down,
            "L" => Step::Left,
            "R" => Step::Right,
            _ => unimplemented!("Should not happen!"),
        };

        let amount = str::parse::<isize>(amount).unwrap();

        for _ in 0..amount {
            (head, tail) = do_step(head, tail, step);

            visited.insert(tail);
        }
    }

    println!("Answer part 1: {:?}", visited.len());

    // Part 2
    println!("Part 2");

    println!("Answer part 2: {:?}", "");
}
