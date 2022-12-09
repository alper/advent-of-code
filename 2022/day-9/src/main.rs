use std::{collections::HashSet, fs};

use itertools::Itertools;

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct Pos {
    x: isize,
    y: isize,
}

impl Default for Pos {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
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

    // Diagonal two
    if head_pos.x - tail_pos.x == 2 && head_pos.y - tail_pos.y == 2 {
        return Pos {
            x: tail_pos.x + 1,
            y: tail_pos.y + 1,
        };
    }

    if head_pos.x - tail_pos.x == 2 && head_pos.y - tail_pos.y == -2 {
        return Pos {
            x: tail_pos.x + 1,
            y: tail_pos.y - 1,
        };
    }

    if head_pos.x - tail_pos.x == -2 && head_pos.y - tail_pos.y == 2 {
        return Pos {
            x: tail_pos.x - 1,
            y: tail_pos.y + 1,
        };
    }

    if head_pos.x - tail_pos.x == -2 && head_pos.y - tail_pos.y == -2 {
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
    // println!("Do step: {:?}", step);

    let new_head = head_move(head, step);
    let new_tail = tail_move(new_head, tail);

    // println!("New Head: {:?}, New Tail: {:?}", new_head, new_tail);
    // println!("Head: {:?}", new_head);
    // println!("Tail: {:?}", new_tail);

    (new_head, new_tail)
}

fn render_snake(snake: [Pos; 10]) {
    for i in 0..snake.len() {
        print!("{}x{} ", snake[i].x, snake[i].y);
    }
    println!("");
}

fn main() {
    let input = fs::read_to_string("full_input.txt").expect("File not readable");

    // Part 1
    println!("Part 1");

    let mut visited: HashSet<Pos> = HashSet::new();

    let mut head = Default::default();
    let mut tail = Default::default();

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

    let mut visited: HashSet<Pos> = HashSet::new();

    let mut snake: [Pos; 10] = Default::default();

    visited.insert(snake[9]);

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

        println!("Move: {amount} in {:?}", step);

        for _ in 0..amount {
            snake[0] = head_move(snake[0], step);
            snake[1] = tail_move(snake[0], snake[1]);
            snake[2] = tail_move(snake[1], snake[2]);
            snake[3] = tail_move(snake[2], snake[3]);
            snake[4] = tail_move(snake[3], snake[4]);
            snake[5] = tail_move(snake[4], snake[5]);
            snake[6] = tail_move(snake[5], snake[6]);
            snake[7] = tail_move(snake[6], snake[7]);
            snake[8] = tail_move(snake[7], snake[8]);
            snake[9] = tail_move(snake[8], snake[9]);

            render_snake(snake);

            visited.insert(snake[9]);
        }
        println!("");
    }

    println!("Answer part 2: {:?}", visited.len());
}
