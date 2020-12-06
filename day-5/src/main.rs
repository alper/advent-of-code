use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Dead file");

    let res = input.split('\n').map(|r| seat_id(r)).max();

    println!("Result: {:?}", res);

    let mut free_seats = [true; 1024];

    println!("Number of total seats: {:?}", free_seats.len()) ;

    for pass in input.split('\n') {
        let sid = seat_id(pass);

        free_seats[sid as usize] = false;
    }

    for i in 1..1023 {
        if !free_seats[i-1] && free_seats[i] && !free_seats[i+1] {
            println!("My seat: {:?}", i);
        }
    }
}

fn seat_id(row_seq: &str) -> u32 {
    row(&row_seq[..7]) * 8 + col(&row_seq[7..])
}

#[test]
fn test_seat_id() {
    assert_eq!(seat_id("FBFBBFFRLR"), 357);
    assert_eq!(seat_id("BFFFBBFRRR"), 567);
    assert_eq!(seat_id("FFFBBBFRRR"), 119);
    assert_eq!(seat_id("BBFFBBFRLL"), 820);
}

fn row(seq: &str) -> u32 {
    let mut seats: Vec<u32> = (0..(2u32.pow(seq.len() as u32))).collect();

    for char in seq.chars() {
        let half = seats.len() / 2;

        if char == 'F' {
            if seats.len() == 2 {
                return seats[0];
            } else {
                seats.drain(half..);
            }
        } else {
            if seats.len() == 2 {
                return seats[1];
            } else {
                seats.drain(0..half);
            }
        }
    }

    0 // Default return value for the compiler
}

#[test]
fn test_row() {
    // 0 1 2 3
    assert_eq!(row("FF"), 0);
    assert_eq!(row("FB"), 1);
    assert_eq!(row("BB"), 3);
    assert_eq!(row("BF"), 2);

    assert_eq!(row("FBFBBFF"), 44);
}

fn col(seq: &str) -> u32 {
    // Doing a direct lookup table
    match seq {
        "LLL" => 0,
        "LLR" => 1,
        "LRL" => 2,
        "LRR" => 3,
        "RLL" => 4,
        "RLR" => 5,
        "RRL" => 6,
        "RRR" => 7,
        _ => unreachable!()
    }
}

#[test]
fn test_col() {
    assert_eq!(col("RLR"), 5);
}