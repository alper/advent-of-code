fn main() {
    run_until_2020(&vec![0, 5, 4, 1, 10, 14, 7]);
}

fn run_until_2020(starting_sequence: &[u64]) -> u64 {
    let mut count = 1;

    let mut v = vec![];
    v.extend_from_slice(starting_sequence);

    for el in v.iter() {
        println!("{}th number: {}", count, el);

        count += 1;
    }

    while count <= 2020 {
        let next_number = next_number(&v);

        println!("{}th number: {}", count, next_number);

        v.push(next_number);
        count += 1;
    }

    return *v.last().unwrap();
}

fn next_number(series: &[u64]) -> u64 {
    let last = series.last().unwrap();

    // let first_position = series.iter().position(|&x| x == *last).unwrap();

    // println!("Last: {}", last);

    if !series[..series.len() - 2].contains(last) {
        return 0;
    } else {
        let mut v = vec![];
        v.extend_from_slice(&series[..series.len() - 1]);
        v.reverse();

        // println!("Reversed head: {:?}", v);

        let first_position = v.iter().position(|&n| n == *last).unwrap();
        // println!("First position: {}", first_position);

        return (first_position + 1) as u64;
    }
}

#[test]
fn test_next_number() {
    assert_eq!(next_number(&vec![1, 2, 3, 0]), 0);
    assert_eq!(next_number(&vec![1, 2, 3, 2]), 2);
}
