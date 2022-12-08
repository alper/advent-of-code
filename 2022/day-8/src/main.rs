use array2d::Array2D;
use std::fs;

fn debug_array_usize(array: &Array2D<usize>) {
    print!("[\n");

    for i in 0..array.num_rows() {
        for j in 0..array.num_columns() {
            let height = array.get(i, j).unwrap();

            print!("{height}");
        }
        println!("");
    }

    println!("]");
}

fn debug_array_bool(array: &Array2D<bool>) {
    print!("[\n");

    for i in 0..array.num_rows() {
        for j in 0..array.num_columns() {
            if *array.get(i, j).unwrap() {
                print!("T");
            } else {
                print!("F");
            }
        }
        println!("");
    }

    println!("]");
}

fn num_visible_trees(array: &Array2D<usize>, row: usize, col: usize) -> usize {
    println!("Visible for {row}x{col}");

    let height = *array.get(row, col).unwrap();

    let mut count_north = 0;

    for i in (0..row).rev() {
        println!("at {i}x{col} {}", *array.get(i, col).unwrap());
        count_north += 1;

        if *array.get(i, col).unwrap() >= height {
            break;
        }
    }

    let mut count_south = 0;
    for i in row + 1..array.num_rows() {
        println!("at {i}x{col} {}", *array.get(i, col).unwrap());

        count_south += 1;

        if *array.get(i, col).unwrap() >= height {
            break;
        }
    }

    let mut count_west = 0;
    for i in (0..col).rev() {
        println!("at {row}x{i} {}", *array.get(row, i).unwrap());
        count_west += 1;

        if *array.get(row, i).unwrap() >= height {
            break;
        }
    }

    let mut count_east = 0;
    for i in col + 1..array.num_columns() {
        println!("at {row}x{i} {}", *array.get(row, i).unwrap());
        count_east += 1;

        if *array.get(row, i).unwrap() >= height {
            break;
        }
    }

    count_north * count_south * count_west * count_east
}

fn main() {
    let input = fs::read_to_string("full_input.txt").expect("File not readable");

    let v: Vec<Vec<usize>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();

    let tree_heights = Array2D::from_rows(&v);

    // Part 1
    println!("Part 1");

    let mut tree_visibility =
        Array2D::filled_with(false, tree_heights.num_rows(), tree_heights.num_columns());

    for i in 0..tree_heights.num_columns() {
        let mut max = -1;

        for j in 0..tree_heights.num_rows() {
            let height = *tree_heights.get(j, i).unwrap() as i128;

            // println!("{j}x{i} height: {height} max: {max}");

            if height > max {
                tree_visibility.set(j, i, true);
            }

            max = std::cmp::max(max, height);
        }
    }

    for i in 0..tree_heights.num_columns() {
        let mut max = -1;

        for j in (0..tree_heights.num_rows()).rev() {
            let height = *tree_heights.get(j, i).unwrap() as i128;

            // println!("{j}x{i} height: {height} max: {max}");

            if height > max {
                tree_visibility.set(j, i, true);
            }

            max = std::cmp::max(max, height);
        }
    }

    for i in 0..tree_heights.num_rows() {
        let mut max = -1;

        for j in 0..tree_heights.num_columns() {
            let height = *tree_heights.get(i, j).unwrap() as i128;

            if height > max {
                tree_visibility.set(i, j, true);
            }

            max = std::cmp::max(max, height);
        }
    }

    for i in 0..tree_heights.num_rows() {
        let mut max = -1;

        for j in (0..tree_heights.num_columns()).rev() {
            let height = *tree_heights.get(i, j).unwrap() as i128;

            if height > max {
                tree_visibility.set(i, j, true);
            }

            max = std::cmp::max(max, height);
        }
    }

    debug_array_usize(&tree_heights);

    println!("Visibilities");

    debug_array_bool(&tree_visibility);
    // println!("{:?}", tree_visibility);

    println!(
        "Answer part 1: {:?}",
        tree_visibility
            .elements_row_major_iter()
            .filter(|el| **el == true)
            .count()
    );

    // Part 2
    // println!("Part 2");

    println!("Trees for 1x2, {}", num_visible_trees(&tree_heights, 1, 2));
    println!("Trees for 3x2, {}", num_visible_trees(&tree_heights, 3, 2));

    let mut max_score = 0_usize;

    for i in 0..tree_heights.num_rows() {
        for j in 0..tree_heights.num_columns() {
            let new_score = num_visible_trees(&tree_heights, i, j);

            max_score = std::cmp::max(max_score, new_score);
        }
    }

    println!("Answer part 2: {:?}", max_score);
}
