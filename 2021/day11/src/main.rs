use std::fs;
use itertools::{Itertools};
use grid::*;
use std::fmt;

#[derive(Clone, Copy)]
struct Octopus {
    energy_level: usize,
    flashed: bool,
}

impl Default for Octopus {
    fn default() -> Self {
        Octopus {
            energy_level: 0,
            flashed: false,
        }
    }
}

impl fmt::Debug for Octopus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.energy_level)
    }
}

type OctopusGrid = Grid<Octopus>;

fn main() {
    let input = fs::read_to_string("input.txt").expect("File not readable");

    let a: Vec<_> = input
        .replace("\n", "")
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .map(|d| Octopus{ energy_level: d, flashed: false})
        .collect();

    let mut g: OctopusGrid = Grid::from_vec(a.clone(), 10);

    println!("Input: {:?}", g);

    let mut flashes = 0;
    for _ in 0..100 {
        flashes += step_octopuses(&mut g);
    }

    println!("Answer 1: {:?}", flashes);

    // Part 2
    let mut g: OctopusGrid = Grid::from_vec(a, 10);
    let mut steps = 1;
    loop {
        step_octopuses(&mut g);

        // Check grid
        if g.iter().all(|o| o.energy_level == 0) {
            println!("Answer 2: {}", steps);

            break;
        }

        steps += 1;
    }
}

fn step_octopuses(g: &mut OctopusGrid) -> usize {
    // Increase each by 1
    for o in g.iter_mut() {
        o.energy_level +=1 ;
    }

    let mut total_flash_count = 0;

    // Process flashes
    loop {
        let mut local_flash_count = 0;

        for i in 0..g.rows() {
            for j in 0..g.cols() {
                let mut o = g.get_mut(i, j).unwrap();

                if o.energy_level > 9 && !o.flashed {
                    // Flash
                    o.flashed = true;
                    local_flash_count += 1;

                    // Increment the neighbors
                    let neighbors = get_8_neighbors(i, j, g.rows(), g.cols());
                    for n in neighbors {
                        // println!("N: {:?}", n);
                        let mut octo_neighbor = g.get_mut(n.0, n.1).unwrap();
                        octo_neighbor.energy_level += 1;
                    }
                }
            }
        }

        println!("Flash count: {}", local_flash_count);
        total_flash_count += local_flash_count;
        if local_flash_count == 0 {
            break;
        }
    }

    // Set all flashed octopuses back
    for o in g.iter_mut() {
        if o.flashed {
            o.energy_level = 0;
            o.flashed = false;
        }
    }

    total_flash_count
}

#[test]
fn test_get_neighbors() {
    // 0 1 2
    // 3 4 5
    // 6 7 8

    assert_eq!(get_8_neighbors(0, 0, 3, 3), vec![(0, 1), (1, 0), (1, 1)]);
    assert_eq!(get_8_neighbors(1, 0, 3, 3), vec![(0, 0), (0, 1), (1, 1), (2, 0), (2, 1)]);
    assert_eq!(get_8_neighbors(2, 2, 3, 3), vec![(1, 1), (1, 2), (2, 1)]);
    assert_eq!(get_8_neighbors(1, 1, 3, 3), vec![(0, 0), (0, 1), (0, 2), (1, 0), (1, 2), (2, 0), (2, 1), (2, 2)]);
}

fn get_8_neighbors(r: usize, c: usize, rows: usize, cols: usize) -> Vec<(usize, usize)> {
    let mut v = vec![];

    if r > 0 && c > 0 { // NW
        v.push((r-1, c-1));
    }

    if r > 0 { // N
        v.push((r-1, c));
    }

    if r > 0 && c < cols-1 { // NE
        v.push((r-1, c+1));
    }

    if c > 0 { // E
        v.push((r, c-1));
    }

    if c < cols-1 { // W
        v.push((r, c+1));
    }

    if r < rows-1 && c > 0 { // SW
        v.push((r+1, c-1));
    }

    if r < rows-1 { // S
        v.push((r+1, c));
    }

    if r < rows-1 && c < cols-1 { // SE
        v.push((r+1, c+1));
    }

    v
}