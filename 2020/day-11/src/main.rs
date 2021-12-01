use std::fs;

fn main() {
    // part1();
    part2();
}

fn part1() {
    let test_input = r"L.LL.LL.LL
    LLLLLLL.LL
    L.L.L..L..
    LLLL.LL.LL
    L.LL.LL.LL
    L.LLLLL.LL
    ..L.L.....
    LLLLLLLLLL
    L.LLLLLL.L
    L.LLLLL.LL";

    let test_parsed = parse(test_input);
    println!("Parsed test input: {:?}", test_parsed);

    println!(
        "Result for test input: {:?}",
        count_occupied_seats(&test_parsed)
    );

    let input = fs::read_to_string("input.txt").expect("Dead file");
    let parsed = parse(&input);
    println!("Parsed test input: {:?}", parsed);

    println!("Result for test input: {:?}", count_occupied_seats(&parsed));
}

fn count_occupied_seats(field: &Vec<Vec<char>>) -> u32 {
    let mut old_field = field.clone();

    loop {
        let new_field = step(&old_field);

        println!("New field: {:?}", new_field);

        if old_field == new_field {
            let row_counts = new_field
                .iter()
                .map(|l| l.iter().filter(|&c| *c == '#').count());

            return row_counts.sum::<usize>() as u32;
        } else {
            old_field = new_field
        }
    }
}

fn parse(input: &str) -> Vec<Vec<char>> {
    let mut out_vec: Vec<Vec<char>> = vec![];

    for line in input.lines() {
        let mut in_vec: Vec<char> = vec![];

        for c in line.trim().chars() {
            in_vec.push(c)
        }

        out_vec.push(in_vec);
    }

    out_vec
}

fn step(field: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut new_field: Vec<Vec<char>> = vec![];

    for y in 0..field.len() {
        let mut new_field_row: Vec<char> = vec![];

        for x in 0..field[y].len() {
            let count = adjacency_count(field, x as i32, y as i32);

            if field[y][x] == 'L' && count == 0 {
                new_field_row.push('#');
            } else if field[y][x] == 'L' {
                new_field_row.push('L');
            } else if field[y][x] == '#' && count >= 4 {
                new_field_row.push('L');
            } else if field[y][x] == '#' {
                new_field_row.push('#');
            } else {
                new_field_row.push('.');
            }
        }
        new_field.push(new_field_row);
    }

    new_field
}

fn adjacency_count(field: &[Vec<char>], x: i32, y: i32) -> u32 {
    let mut count = 0;

    if x - 1 >= 0 && y - 1 >= 0 && field[(y - 1) as usize][(x - 1) as usize] == '#' {
        count += 1;
    }

    if y - 1 >= 0 && field[(y - 1) as usize][x as usize] == '#' {
        count += 1;
    }

    if x + 1 < field[0].len() as i32
        && y - 1 >= 0
        && field[(y - 1) as usize][(x + 1) as usize] == '#'
    {
        count += 1;
    }

    if x - 1 >= 0 && field[y as usize][(x - 1) as usize] == '#' {
        count += 1;
    }

    if x + 1 < field[0].len() as i32 && field[y as usize][(x + 1) as usize] == '#' {
        count += 1;
    }

    if x - 1 >= 0 && y + 1 < field.len() as i32 && field[(y + 1) as usize][(x - 1) as usize] == '#'
    {
        count += 1;
    }

    if y + 1 < field.len() as i32 && field[(y + 1) as usize][x as usize] == '#' {
        count += 1;
    }

    if x + 1 < field[0].len() as i32
        && y + 1 < field.len() as i32
        && field[(y + 1) as usize][(x + 1) as usize] == '#'
    {
        count += 1;
    }

    count
}

#[test]
fn test_adjacency_count() {
    // .L..
    // ..#.
    // .##.
    // .L..
    let mut field = vec![vec!['.'; 4]; 4];
    field[0][1] = 'L';
    field[1][2] = '#';
    field[2][1] = '#';
    field[2][2] = '#';
    field[3][1] = 'L';

    println!("Field: {:?}", field);

    assert_eq!(adjacency_count(&field, 2, 0), 1);
    assert_eq!(adjacency_count(&field, 3, 1), 2);
    assert_eq!(adjacency_count(&field, 1, 1), 3);
    assert_eq!(adjacency_count(&field, 2, 3), 2);
}

// Part 2 functions

fn adjacency_sight_count(field: &[Vec<char>], x: usize, y: usize) -> usize {
    let mut count = 0;

    // println!("Checking for {}x{}", x, y);

    // NW
    for (xx, yy) in (0..x).rev().zip((0..y).rev()) {
        // println!("Found1 {} at {}x{}", field[yy][xx], xx, yy);

        if field[yy][xx] == 'L' {
            break;
        } else if field[yy][xx] == '#' {
            count += 1;
            break;
        }
    }

    // N
    for yy in (0..y).rev() {
        // println!("Found2 {} at {}x{}", field[yy][x], x, yy);

        if field[yy][x] == 'L' {
            break;
        } else if field[yy][x] == '#' {
            count += 1;
            break;
        }
    }

    // NE
    for (xx, yy) in field[0].iter().enumerate().filter(|(i, _)| *i > x).map(|(i, _)| i).zip((0..y).rev()) {
        // println!("Found3 {} at {}x{}", field[yy][xx], xx, yy);

        if field[yy][xx] == 'L' {
            break;
        } else if field[yy][xx] == '#' {
            count += 1;
            break;
        }
    }

    // W
    for xx in (0..x).rev() {
        // println!("Found4 {} at {}x{}", field[y][xx], xx, y);

        if field[y][xx] == 'L' {
            break;
        } else if field[y][xx] == '#' {
            count += 1;
            break;
        }
    }

    // E
    for xx in field[0].iter().enumerate().filter(|(i, _)| *i > x).map(|(i, _)| i) {
        // println!("Found5 {} at {}x{}", field[y][xx], xx, y);

        if field[y][xx] == 'L' {
            break;
        } else if field[y][xx] == '#' {
            count += 1;
            break;
        }
    }

    // SW
    for (xx, yy) in (0..x).rev().zip(field.iter().enumerate().filter(|(i, _)| *i > y).map(|(i, _)| i)) {
        // println!("Found6 {} at {}x{}", field[yy][xx], xx, yy);

        if field[yy][xx] == 'L' {
            break;
        } else if field[yy][xx] == '#' {
            count += 1;
            break;
        }
    }

    // S
    for yy in field.iter().enumerate().filter(|(i, _)| *i > y).map(|(i, _)| i) {
        // println!("Found7 {} at {}x{}", field[yy][x], x, yy);

        if field[yy][x] == 'L' {
            break;
        } else if field[yy][x] == '#' {
            count += 1;
            break;
        }
    }

    // SE
    for (xx, yy) in field[0].iter().enumerate().filter(|(i, _)| *i > x).map(|(i, _)| i).zip(field.iter().enumerate().filter(|(i, _)| *i > y).map(|(i, _)| i )) {
        // println!("Found8 {} at {}x{}", field[yy][xx], xx, yy);

        if field[yy][xx] == 'L' {
            break;
        } else if field[yy][xx] == '#' {
            count += 1;
            break;
        }
    }

    count
}

#[test]
fn test_adjacency_sight_count() {
    // .L..
    // ..#.
    // .##.
    // .L..
    let mut field = vec![vec!['.'; 4]; 4];
    field[0][1] = 'L';
    field[1][2] = '#';
    field[2][1] = '#';
    field[2][2] = '#';
    field[3][1] = 'L';

    assert_eq!(adjacency_sight_count(&field, 0, 0), 1);
    assert_eq!(adjacency_sight_count(&field, 3, 0), 1);
    assert_eq!(adjacency_sight_count(&field, 1, 1), 3);
    assert_eq!(adjacency_sight_count(&field, 0, 2), 1);

    let field_test1 = parse(
        r".............
    .L.L.#.#.#.#.
    .............",
    );
    assert_eq!(adjacency_sight_count(&field_test1, 1, 1), 0);

    let field_test2 = parse(
        r".##.##.
    #.#.#.#
    ##...##
    ...L...
    ##...##
    #.#.#.#
    .##.##.",
    );
    assert_eq!(adjacency_sight_count(&field_test2, 3, 3), 0);
}

fn step_sight(field: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut new_field: Vec<Vec<char>> = vec![];

    for y in 0..field.len() {
        let mut new_field_row: Vec<char> = vec![];

        for x in 0..field[y].len() {
            let count = adjacency_sight_count(field, x, y);

            if field[y][x] == 'L' && count == 0 {
                new_field_row.push('#');
            } else if field[y][x] == 'L' {
                new_field_row.push('L');
            } else if field[y][x] == '#' && count >= 5 {
                new_field_row.push('L');
            } else if field[y][x] == '#' {
                new_field_row.push('#');
            } else {
                new_field_row.push('.');
            }
        }
        new_field.push(new_field_row);
    }

    new_field
}

#[test]
fn test_step_sight() {
    let start = parse(r"L.LL.LL.LL
    LLLLLLL.LL
    L.L.L..L..
    LLLL.LL.LL
    L.LL.LL.LL
    L.LLLLL.LL
    ..L.L.....
    LLLLLLLLLL
    L.LLLLLL.L
    L.LLLLL.LL");

    assert_eq!(step_sight(&start), parse(r"#.##.##.##
    #######.##
    #.#.#..#..
    ####.##.##
    #.##.##.##
    #.#####.##
    ..#.#.....
    ##########
    #.######.#
    #.#####.##"));
    assert_eq!(step_sight(&step_sight(&start)), parse(r"#.LL.LL.L#
    #LLLLLL.LL
    L.L.L..L..
    LLLL.LL.LL
    L.LL.LL.LL
    L.LLLLL.LL
    ..L.L.....
    LLLLLLLLL#
    #.LLLLLL.L
    #.LLLLL.L#"));
    assert_eq!(step_sight(&step_sight(&step_sight(&start))), parse(r"#.L#.##.L#
    #L#####.LL
    L.#.#..#..
    ##L#.##.##
    #.##.#L.##
    #.#####.#L
    ..#.#.....
    LLL####LL#
    #.L#####.L
    #.L####.L#"));
    assert_eq!(step_sight(&step_sight(&step_sight(&step_sight(&start)))), parse(r"#.L#.L#.L#
    #LLLLLL.LL
    L.L.L..#..
    ##LL.LL.L#
    L.LL.LL.L#
    #.LLLLL.LL
    ..L.L.....
    LLLLLLLLL#
    #.LLLLL#.L
    #.L#LL#.L#"));
    assert_eq!(step_sight(&step_sight(&step_sight(&step_sight(&step_sight(&start))))), parse(r"#.L#.L#.L#
    #LLLLLL.LL
    L.L.L..#..
    ##L#.#L.L#
    L.L#.#L.L#
    #.L####.LL
    ..#.#.....
    LLL###LLL#
    #.LLLLL#.L
    #.L#LL#.L#"));
    assert_eq!(step_sight(&step_sight(&step_sight(&step_sight(&step_sight(&step_sight(&start)))))), parse(r"#.L#.L#.L#
    #LLLLLL.LL
    L.L.L..#..
    ##L#.#L.L#
    L.L#.LL.L#
    #.LLLL#.LL
    ..#.L.....
    LLL###LLL#
    #.LLLLL#.L
    #.L#LL#.L#"));
}

fn count_occupied_seats_sight(field: &Vec<Vec<char>>) -> u32 {
    let mut old_field = field.clone();

    loop {
        let new_field = step_sight(&old_field);

        println!("New field: {:?}", new_field);

        if old_field == new_field {
            let row_counts = new_field
                .iter()
                .map(|l| l.iter().filter(|&c| *c == '#').count());

            return row_counts.sum::<usize>() as u32;
        } else {
            old_field = new_field
        }
    }
}

fn part2() {
    let test_input = r"L.LL.LL.LL
    LLLLLLL.LL
    L.L.L..L..
    LLLL.LL.LL
    L.LL.LL.LL
    L.LLLLL.LL
    ..L.L.....
    LLLLLLLLLL
    L.LLLLLL.L
    L.LLLLL.LL";

    let test_parsed = parse(test_input);

    println!(
        "Result for test input part 2: {:?}",
        count_occupied_seats_sight(&test_parsed)
    );

    let input = fs::read_to_string("input.txt").expect("Dead file");
    let parsed = parse(&input);

    println!("Part 2 result for real input: {:?}", count_occupied_seats_sight(&parsed));
}
