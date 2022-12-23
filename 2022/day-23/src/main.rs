use std::{
    collections::{HashMap, HashSet},
    fs,
    hash::Hash,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    N,
    S,
    W,
    E,
}

impl Direction {
    fn move_offset(self) -> (isize, isize) {
        match self {
            Direction::N => (0, -1),
            Direction::S => (0, 1),
            Direction::W => (-1, 0),
            Direction::E => (1, 0),
        }
    }

    fn checks(self) -> [(isize, isize); 3] {
        match self {
            Direction::N => [(-1, -1), (0, -1), (1, -1)],
            Direction::S => [(-1, 1), (0, 1), (1, 1)],
            Direction::W => [(-1, -1), (-1, 0), (-1, 1)],
            Direction::E => [(1, -1), (1, 0), (1, 1)],
        }
    }

    fn next(self) -> Direction {
        match self {
            Direction::N => Self::S,
            Direction::S => Self::W,
            Direction::W => Self::E,
            Direction::E => Self::N,
        }
    }
}

fn get_bounding_box(locations: &HashSet<(isize, isize)>) -> (isize, isize, isize, isize) {
    (
        locations.iter().map(|l| l.0).min().unwrap(),
        locations.iter().map(|l| l.1).min().unwrap(),
        locations.iter().map(|l| l.0).max().unwrap(),
        locations.iter().map(|l| l.1).max().unwrap(),
    )
}

fn print_locations(locations: &HashSet<(isize, isize)>) {
    let (minx, miny, maxx, maxy) = get_bounding_box(locations);

    for j in miny..maxy + 1 {
        for i in minx..maxx + 1 {
            if locations.contains(&(i, j)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let input = fs::read_to_string("full_input.txt").expect("File not readable");

    let i = input.lines();
    let mut locations: HashSet<(isize, isize)> = HashSet::new();

    i.enumerate().for_each(|(i, line)| {
        line.chars().enumerate().for_each(|(j, c)| {
            if c == '#' {
                locations.insert((j.try_into().unwrap(), i.try_into().unwrap()));
            }
        })
    });

    println!("{:?}", locations);

    let mut direction = Direction::N;

    println!("Start");
    print_locations(&locations);

    let mut round = 0;
    loop {
        // println!("Number of elves: {}", locations.len());

        // Make the plan for all the new positions
        let mut plan: HashMap<(isize, isize), Vec<(isize, isize)>> = HashMap::new();

        for elf in &locations {
            let mut direction_to_check = direction;
            let mut elf_found_a_place = false;

            // Check if any elves are nearby
            let any_elves = [
                (-1, -1),
                (0, -1),
                (1, -1),
                (-1, 0),
                (1, 0),
                (-1, 1),
                (0, 1),
                (1, 1),
            ]
            .iter()
            .any(|&neighbor_loc| {
                locations.contains(&(elf.0 + neighbor_loc.0, elf.1 + neighbor_loc.1))
            });

            if any_elves {
                for _ in 0..4 {
                    // println!("Elf {:?}, checking direction {:?}", elf, direction_to_check);

                    if direction_to_check.checks().iter().all(|offset| {
                        let neighbor_target = (elf.0 + offset.0, elf.1 + offset.1);
                        let neighbor_value = locations.get(&neighbor_target);

                        // println!("Got {:?} for {:?}", neighbor_value, neighbor_target);

                        return neighbor_value.is_none();
                    }) {
                        let target = (
                            elf.0 + direction_to_check.move_offset().0,
                            elf.1 + direction_to_check.move_offset().1,
                        );

                        // println!("Trying to go to: {:?}", target);

                        let candidates = plan.entry(target).or_insert(vec![]);
                        candidates.push(*elf);

                        elf_found_a_place = true;
                        break;
                    }

                    direction_to_check = direction_to_check.next();
                }
            }

            if !elf_found_a_place {
                let candidates = plan.entry(*elf).or_insert(vec![]);
                candidates.push(*elf);
            }

            // println!();
        }

        // Deconflict the plan
        let mut final_plan: HashMap<(isize, isize), (isize, isize)> = HashMap::new();
        for (target, candidates) in plan.iter() {
            if candidates.len() > 1 {
                for c in candidates {
                    final_plan.insert(*c, *c);
                }
            } else {
                final_plan.insert(*target, *candidates.first().unwrap());
            }
        }

        // Check if anybody is moving
        let mut final_targets: Vec<&(isize, isize)> = final_plan.keys().collect();
        let mut original_locations: Vec<&(isize, isize)> = locations.iter().collect();
        final_targets.sort();
        original_locations.sort();

        if original_locations
            .iter()
            .zip(final_targets)
            .all(|(l, r)| *l == r)
        {
            println!("Answer part 2: {}", round + 1);
            break;
        }

        // Make the move
        locations.clear();
        for key in final_plan.keys() {
            locations.insert(*key);
        }

        direction = direction.next();

        // println!("End of Round {}", round + 1);
        // print_locations(&locations);

        if round == 10 {
            let (minx, miny, maxx, maxy) = get_bounding_box(&locations);

            println!(
                "Answer part 1: {:?}",
                ((maxx - minx + 1) * (maxy - miny + 1)) - locations.len() as isize
            );
        }

        round += 1;
    }

    // println!("Done");
    // print_locations(&locations);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
