use array2d::Array2D;
use std::fs;

fn is_visible(array: &Array2D<usize>, row: usize, col: usize) -> bool {
    let height = *array.get(row, col).unwrap();

    let mut vis_north = true;

    for i in 0..row {
        // println!("Got: {:?}", *array.get(i, col).unwrap());

        if *array.get(i, col).unwrap() >= height {
            vis_north = false;
        }
    }

    let mut vis_south = true;
    for i in row + 1..array.num_rows() {
        // println!("Got: {:?}", *array.get(i, col).unwrap());

        if *array.get(i, col).unwrap() >= height {
            vis_south = false;
        }
    }

    let mut vis_west = true;
    for i in 0..col {
        // println!("Got: {:?}", *array.get(row, i).unwrap());

        if *array.get(row, i).unwrap() >= height {
            vis_west = false;
        }
    }

    let mut vis_east = true;
    for i in col + 1..array.num_columns() {
        // println!("Got: {:?}", *array.get(row, i).unwrap());

        if *array.get(row, i).unwrap() >= height {
            vis_east = false;
        }
    }

    vis_north || vis_south || vis_east || vis_west
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

    let array = Array2D::from_rows(&v);

    // println!("{:?}", is_visible(&array, 1, 1));
    // println!("Visible {:?}", is_visible(&array, 1, 3));

    // println!("Visible 2x1 {:?}", is_visible(&array, 2, 1));
    // println!("Visible 2x2 {:?}", is_visible(&array, 2, 2));

    // Part 1
    println!("Part 1");

    let mut count = 0_usize;

    for i in 0..array.num_rows() {
        for j in 0..array.num_columns() {
            if is_visible(&array, i, j) {
                count += 1;
            }
        }
    }

    println!("Answer part 1: {:?}", count);

    // Part 2
    println!("Part 2");

    println!("Trees for 1x2, {}", num_visible_trees(&array, 1, 2));
    println!("Trees for 3x2, {}", num_visible_trees(&array, 3, 2));

    let mut max_score = 0_usize;

    for i in 0..array.num_rows() {
        for j in 0..array.num_columns() {
            let new_score = num_visible_trees(&array, i, j);

            max_score = std::cmp::max(max_score, new_score);
        }
    }

    println!("Answer part 2: {:?}", max_score);
}
