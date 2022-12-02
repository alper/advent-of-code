use std::fs;

fn main() {
    let input = fs::read_to_string("full_input.txt").expect("File not readable");

    // Part 1
    println!("Part 1");

    let v: Vec<usize> = input.lines().map(|moves| moves.split(" ").collect::<Vec<_>>()).map(|v| {
        let shape_score = match v[1] {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => 0
        };

        let mut round_score = 0;

        if v[0] == "A" {
            if v[1] == "X" {
                round_score = 3;
            } else if v[1] == "Y" {
                round_score = 6;
            } else if v[1] == "Z" {
                round_score = 0;
            }
        }

        if v[0] == "B" {
            if v[1] == "X" {
                round_score = 0;
            } else if v[1] == "Y" {
                round_score = 3;
            } else if v[1] == "Z" {
                round_score = 6;
            }
        }

        if v[0] == "C" {
            if v[1] == "X" {
                round_score = 6;
            } else if v[1] == "Y" {
                round_score = 0;
            } else if v[1] == "Z" {
                round_score = 3;
            }
        }

        return round_score + shape_score;
    }).collect();

    println!("Answer part 1: {:?}", v.iter().sum::<usize>());


    // Part 2
    println!("Part 2");

    let v: Vec<usize> = input.lines().map(|moves| moves.split(" ").collect::<Vec<_>>()).map(|v| {
        let round_score = match v[1] {
            "X" => 0,
            "Y" => 3,
            "Z" => 6,
            _ => 0
        };

        println!("Decoded: {}, Round Score: {}", v[1], round_score);

        #[derive(Debug)]
        enum Shape {
            Rock,
            Paper,
            Scissors
        }

        let mut my_move: Shape = Shape::Rock;

        if v[0] == "A" {
            if v[1] == "X" {
                my_move = Shape::Scissors;
            } else if v[1] == "Y" {
                my_move = Shape::Rock;
            } else if v[1] == "Z" {
                my_move = Shape::Paper;
            }
        }

        if v[0] == "B" {
            if v[1] == "X" {
                my_move = Shape::Rock;
            } else if v[1] == "Y" {
                my_move = Shape::Paper;
            } else if v[1] == "Z" {
                my_move = Shape::Scissors;
            }
        }

        if v[0] == "C" {
            if v[1] == "X" {
                my_move = Shape::Paper;
            } else if v[1] == "Y" {
                my_move = Shape::Scissors;
            } else if v[1] == "Z" {
                my_move = Shape::Rock;
            }
        }

        println!("My move: {:?}", my_move);

        let shape_score = match my_move {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        };

        return round_score + shape_score;
    }).collect();

    println!("Answer part 2: {:?}", v.iter().sum::<usize>());
}
