use itertools::Itertools;
use ndarray::{Array3, Array4};

fn main() {
    // part1();
    part2();
}

fn part1() {
    let SIZE = 1_00;

    // Create vast space
    let mut current = Array3::<u32>::zeros((SIZE, SIZE, SIZE));
    let mut next = Array3::<u32>::zeros((SIZE, SIZE, SIZE));

    // Initialize space
    let input = vec![
        vec!['#', '.', '#', '.', '#', '.', '#', '#'],
        vec!['.', '#', '#', '#', '#', '.', '.', '#'],
        vec!['#', '#', '#', '#', '#', '.', '#', '.'],
        vec!['#', '#', '#', '#', '#', '.', '.', '#'],
        vec!['#', '.', '.', '.', '.', '#', '#', '#'],
        vec!['#', '#', '#', '.', '.', '.', '#', '#'],
        vec!['.', '.', '.', '#', '.', '#', '.', '#'],
        vec!['#', '.', '#', '#', '.', '.', '#', '#'],
    ];

    for row in 0..input.len() {
        for col in 0..input[0].len() {
            if input[row][col] == '#' {
                current[[SIZE / 2, (SIZE / 2) + row, (SIZE / 2) + col]] = 1;
            }
        }
    }

    // Count
    println!("Current size: {}", current.sum());

    let indices: Vec<_> = (-1..2)
        .cartesian_product(-1..2)
        .cartesian_product(-1..2)
        .map(|((a, b), c)| (a, b, c))
        .filter(|&t| t != (0, 0, 0))
        .collect();

    println!("Indices: {:?}, {}", indices, indices.len());

    // Cycle
    for i in 0..6 {
        for x in 2..SIZE - 2 {
            for y in 2..SIZE - 2 {
                for z in 2..SIZE - 2 {
                    // println!("{} {} {}", x, y, z);
                    let active_neighbors: u32 = indices
                        .iter()
                        .map(|(a, b, c)| {
                            // println!("{} {} {} {:?}", x, y, z, (a, b, c));
                            return current[[
                                (x as isize + a) as usize,
                                (y as isize + b) as usize,
                                (z as isize + c) as usize,
                            ]];
                        })
                        .sum();

                    if current[[x, y, z]] == 1 {
                        if active_neighbors >= 2 && active_neighbors <= 3 {
                            next[[x, y, z]] = 1;
                        } else {
                            next[[x, y, z]] = 0; // These should already be 0, but here for clarity
                        }
                    } else {
                        if active_neighbors == 3 {
                            next[[x, y, z]] = 1;
                        } else {
                            next[[x, y, z]] = 0; // These should already be 0, but here for clarity
                        }
                    }
                }
            }
        }

        current = next;
        next = Array3::<u32>::zeros((SIZE, SIZE, SIZE));

        println!("Current size at {}: {}", i, current.sum());
    }
}

fn part2() {
    let SIZE = 50;

    // Create vast space
    let mut current = Array4::<u32>::zeros((SIZE, SIZE, SIZE, SIZE));
    let mut next = Array4::<u32>::zeros((SIZE, SIZE, SIZE, SIZE));

    // Initialize space
    let input = vec![
        vec!['#', '.', '#', '.', '#', '.', '#', '#'],
        vec!['.', '#', '#', '#', '#', '.', '.', '#'],
        vec!['#', '#', '#', '#', '#', '.', '#', '.'],
        vec!['#', '#', '#', '#', '#', '.', '.', '#'],
        vec!['#', '.', '.', '.', '.', '#', '#', '#'],
        vec!['#', '#', '#', '.', '.', '.', '#', '#'],
        vec!['.', '.', '.', '#', '.', '#', '.', '#'],
        vec!['#', '.', '#', '#', '.', '.', '#', '#'],
    ];

    for row in 0..input.len() {
        for col in 0..input[0].len() {
            if input[row][col] == '#' {
                current[[SIZE / 2, SIZE / 2, (SIZE / 2) + row, (SIZE / 2) + col]] = 1;
            }
        }
    }

    // Count
    println!("Current size: {}", current.sum());

    let indices: Vec<_> = (-1..2)
        .cartesian_product(-1..2)
        .cartesian_product(-1..2)
        .cartesian_product(-1..2)
        .map(|(((a, b), c), d)| (a, b, c, d))
        .filter(|&t| t != (0, 0, 0, 0))
        .collect();

    println!("Indices: {:?}, {}", indices, indices.len());

    // Cycle
    for i in 0..6 {
        for w in 2..SIZE - 2 {
            for x in 2..SIZE - 2 {
                for y in 2..SIZE - 2 {
                    for z in 2..SIZE - 2 {
                        // println!("{} {} {}", x, y, z);
                        let active_neighbors: u32 = indices
                            .iter()
                            .map(|(a, b, c, d)| {
                                // println!("{} {} {} {:?}", x, y, z, (a, b, c));
                                return current[[
                                    (w as isize + a) as usize,
                                    (x as isize + b) as usize,
                                    (y as isize + c) as usize,
                                    (z as isize + d) as usize,
                                ]];
                            })
                            .sum();

                        if current[[w, x, y, z]] == 1 {
                            if active_neighbors >= 2 && active_neighbors <= 3 {
                                next[[w, x, y, z]] = 1;
                            } else {
                                next[[w, x, y, z]] = 0; // These should already be 0, but here for clarity
                            }
                        } else {
                            if active_neighbors == 3 {
                                next[[w, x, y, z]] = 1;
                            } else {
                                next[[w, x, y, z]] = 0; // These should already be 0, but here for clarity
                            }
                        }
                    }
                }
            }
        }

        current = next;
        next = Array4::<u32>::zeros((SIZE, SIZE, SIZE, SIZE));

        println!("Current size at {}: {}", i, current.sum());
    }
}
