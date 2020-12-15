use std::collections::HashMap;

fn main() {
    // println!("Test answer: {}", run_until_n(2020, &vec![0, 3, 6])); // Should be 406

    println!(
        "Answer part 1: {}",
        run_until_n(2020, &vec![0, 5, 4, 1, 10, 14, 7])
    ); // Should be 203

    println!(
        "Answer part 2: {}",
        run_until_n(30_000_000, &vec![0, 5, 4, 1, 10, 14, 7])
    );
}

fn run_until_n(n: u64, starting_sequence: &[u64]) -> u64 {
    let mut count = 1;

    let mut last_seen_cache: HashMap<u64, u64> = HashMap::new();
    let mut last_last_seen_cache: HashMap<u64, u64> = HashMap::new();
    let mut last = 0;

    for el in starting_sequence.iter() {
        // println!("{}th number: {}", count, el);

        last_seen_cache.insert(*el, count);
        last = *el;
        count += 1;
    }

    while count <= n {
        if last_seen_cache.contains_key(&last) && !last_last_seen_cache.contains_key(&last) {
            last = 0;

            last_last_seen_cache.insert(0, *last_seen_cache.get(&0).unwrap());
            last_seen_cache.insert(0, count);
        } else if last_seen_cache.contains_key(&last) && last_last_seen_cache.contains_key(&last) {
            let last_sighting = last_seen_cache.get(&last).unwrap();
            let pre_last_sighting = last_last_seen_cache.get(&last).unwrap();

            let new_last = last_sighting - pre_last_sighting;

            if last_seen_cache.contains_key(&new_last) {
                last_last_seen_cache.insert(new_last, *last_seen_cache.get(&new_last).unwrap());
            }
            last_seen_cache.insert(new_last, count);

            last = new_last;
        }

        count += 1;

        println!("Count: {}", count);
    }

    return last;
}

fn next_number(series: &[u64]) -> u64 {
    let last = series.last().unwrap();

    // println!("Last: {}", last);

    if !series[..series.len() - 2].contains(last) {
        return 0;
    } else {
        // let mut v = vec![];
        // v.extend_from_slice(&series[..series.len() - 1]);
        // v.reverse();

        // println!("Reversed head: {:?}", v);

        if false {
            let first_position = series[..series.len() - 1]
                .iter()
                .rev()
                .position(|&n| n == *last)
                .unwrap();
            // println!("First position: {}", first_position);

            return (first_position + 1) as u64;
        } else {
            for i in (0..series.len() - 1).rev() {
                if series[i] == *last {
                    return (series.len() - i - 1) as u64;
                }
            }
        }
    }
    0
}

#[test]
fn test_next_number() {
    assert_eq!(next_number(&vec![1, 2, 3, 0]), 0);
    assert_eq!(next_number(&vec![1, 2, 3, 2]), 2);
}
