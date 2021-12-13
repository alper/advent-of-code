use grid::Grid;
use itertools::Itertools;
use std::fs;

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
enum Axis {
    X,
    Y,
}

#[derive(Debug)]
struct Fold {
    axis: Axis,
    length: usize,
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("File not readable");

    let mut points: Vec<Point> = vec![];
    let mut folds: Vec<Fold> = vec![];

    for line in input.lines() {
        if line.contains(',') {
            let mut parts = line.trim().split(',');
            let x = parts.next().unwrap().parse::<usize>().unwrap();
            let y = parts.next().unwrap().parse::<usize>().unwrap();

            points.push(Point { x, y });
        }

        if line.contains('=') {
            let axis = if line.contains('x') { Axis::X } else { Axis::Y };
            let length = line
                .trim()
                .split('=')
                .last()
                .unwrap()
                .parse::<usize>()
                .unwrap();

            folds.push(Fold {
                axis: axis,
                length: length,
            })
        }
    }

    // Render to grid

    // let g = grid_points(&points);
    // render_grid(&g);

    let first_fold = &folds[0];

    fold_grid(&first_fold, &mut points);

    let g = grid_points(&points);
    render_grid(&g);

    let positive_points = g.iter().filter(|p| **p == '#').count();

    println!("Answer part 1: {}", positive_points);

    // Do the rest of the folds
    for fold in &folds[1..] {
        fold_grid(&fold, &mut points);
    }

    let g = grid_points(&points);
    render_grid(&g);
}

fn grid_points(points: &Vec<Point>) -> Grid<char> {
    let max_x = points.iter().map(|p| p.x).max().unwrap();
    let max_y = points.iter().map(|p| p.y).max().unwrap();

    let mut grid: Grid<char> = Grid::new(max_y + 1, max_x + 1);

    for point in grid.iter_mut() {
        *point = '.';
    }

    for point in points {
        // println!("Point: {:?}", point);
        let e = grid.get_mut(point.y, point.x).unwrap();
        *e = '#';
    }

    return grid;
}

fn fold_grid(f: &Fold, points: &mut Vec<Point>) {
    match f.axis {
        Axis::Y => {
            for point in points {
                if point.y > f.length {
                    println!("Input: {} {}", f.length, point.y);

                    let new_y = f.length - (point.y - f.length);

                    println!("New y: {}", new_y);
                    point.y = new_y;
                }
            }
        }
        Axis::X => {
            for point in points {
                if point.x > f.length {
                    let new_x = f.length - (point.x - f.length);
                    point.x = new_x;
                }
            }
        }
    }
}

fn render_grid(grid: &Grid<char>) {
    for r in 0..grid.rows() {
        println!("{:?}", grid.iter_row(r).collect::<Vec<&char>>());
    }
}
