use std::fs;

fn main() {
    let sample_input = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ";

    let real_input = fs::read_to_string("input.txt").unwrap();

    let sample_input = real_input;

    let m = sample_input.split("\n").map(|l| l.split_whitespace().collect::<Vec<_>>()).collect::<Vec<_>>();

    println!("Matrix: {:?}", m);

    let mut transposed = (0..m[0].len()).map(|_| vec![]).collect::<Vec<_>>();

    for original_row in m {
        for (item, transposed_row) in original_row.into_iter().zip(&mut transposed) {
            transposed_row.push(item);
        }
    }

    println!("Transposed: {:?}", transposed);

    let res: usize = transposed.iter().map(|r| calculate_row(&r)).sum();

    println!("Total: {}", res);
}

fn calculate_row(row: &Vec<&str>) -> usize {
    match *row.iter().last().unwrap() {
        "*" => {
            row.iter().rev().skip(1).fold(1, |x, y| x * y.parse::<usize>().unwrap())
        },
        "+" => {
            row.iter().rev().skip(1).map(|s| s.parse::<usize>().unwrap()).sum()
        },
        _ => panic!()
    }
}
