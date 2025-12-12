use std::{
    collections::{HashMap, hash_map::Values},
    fs::read_to_string,
};

fn main() {
    let sample_input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    let real_input = read_to_string("input.txt").unwrap();

    let real_input = real_input;

    let mut grid: HashMap<(isize, isize), bool> = HashMap::new();

    for (i, row) in real_input.split("\n").enumerate() {
        for (j, val) in row.chars().enumerate() {
            grid.insert((i as isize, j as isize), val == '@');
        }
    }

    let c = grid
        .iter()
        .filter(|&(pos, val)| {
            println!("Pos: {:?}, val: {}", pos, val);

            if *val != true {
                return false;
            } else {
                let neighbors = [
                    *grid.get(&(pos.0 - 1, pos.1 - 1)).unwrap_or(&false),
                    *grid.get(&(pos.0, pos.1 - 1)).unwrap_or(&false),
                    *grid.get(&(pos.0 + 1, pos.1 - 1)).unwrap_or(&false),
                    *grid.get(&(pos.0 - 1, pos.1)).unwrap_or(&false),
                    *grid.get(&(pos.0 + 1, pos.1)).unwrap_or(&false),
                    *grid.get(&(pos.0 - 1, pos.1 + 1)).unwrap_or(&false),
                    *grid.get(&(pos.0, pos.1 + 1)).unwrap_or(&false),
                    *grid.get(&(pos.0 + 1, pos.1 + 1)).unwrap_or(&false),
                ];

                neighbors.into_iter().filter(|n| *n == true).count() < 4
            }
        })
        .count();

    println!("Part 1: {}", c);

    let mut rolls_removed = 0;

    while true {
        let mut pickable_this_round: Vec<(isize, isize)> = Vec::new();

        for (&pos, &val) in grid.iter() {
            let neighbors = [
                *grid.get(&(pos.0 - 1, pos.1 - 1)).unwrap_or(&false),
                *grid.get(&(pos.0, pos.1 - 1)).unwrap_or(&false),
                *grid.get(&(pos.0 + 1, pos.1 - 1)).unwrap_or(&false),
                *grid.get(&(pos.0 - 1, pos.1)).unwrap_or(&false),
                *grid.get(&(pos.0 + 1, pos.1)).unwrap_or(&false),
                *grid.get(&(pos.0 - 1, pos.1 + 1)).unwrap_or(&false),
                *grid.get(&(pos.0, pos.1 + 1)).unwrap_or(&false),
                *grid.get(&(pos.0 + 1, pos.1 + 1)).unwrap_or(&false),
            ];

            let pickable = neighbors.into_iter().filter(|n| *n == true).count() < 4;

            if val && pickable {
                pickable_this_round.push(pos);
            }
        }

        println!("Pickable: {:?}", pickable_this_round);

        for picked in &pickable_this_round {
            grid.insert(*picked, false);
        }

        rolls_removed += pickable_this_round.len();

        if pickable_this_round.len() == 0 {
            break;
        }
    }

    println!("Rolls removed: {}", rolls_removed);
}
