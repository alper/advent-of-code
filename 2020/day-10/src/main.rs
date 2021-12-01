use std::collections::BTreeMap;
use std::fs;

fn main() {
    part1();
    part2();
}

fn part1() {
    let input = fs::read_to_string("input.txt").expect("Dead file");

    let diffs = determine_jolt_differences(&input);

    println!("Differences: {:?} and answer: {}", diffs, diffs.0 * diffs.1);
}

fn part2() {
    let input = fs::read_to_string("input.txt").expect("Dead file");

    println!("Sequences: {}", number_of_possible_sequences(&input));
}

fn number_of_possible_sequences(input: &str) -> u64 {
    let mut parsed: Vec<u64> = input
        .lines()
        .map(|l| l.trim().parse::<u64>().unwrap())
        .collect();

    parsed.sort();
    println!("Parsed: {:?}", parsed);

    sequences_counter_recurse(0, &parsed, &mut BTreeMap::new())
}

#[test]
fn test_number_of_possible_sequences() {
    assert_eq!(
        number_of_possible_sequences(
            r"16
        10
        15
        5
        1
        11
        7
        19
        6
        12
        4"
        ),
        8
    );
    assert_eq!(
        number_of_possible_sequences(
            r"28
            33
            18
            42
            31
            14
            46
            20
            48
            47
            24
            23
            49
            45
            19
            38
            39
            11
            1
            32
            25
            35
            8
            17
            7
            9
            4
            2
            34
            10
            3"
        ),
        19208
    );
}

fn sequences_counter_recurse(base: u64, tail: &[u64], cache: &mut BTreeMap<u64, u64>) -> u64 {
    if cache.contains_key(&base) {
        return *cache.get(&base).unwrap();
    }

    let mut possible_sequences = 1;

    println!("Passed {} and {:?}", base, tail);

    if tail.len() == 2 {
        let mut t0 = 0;
        if tail[0] - base <= 3 {
            t0 = sequences_counter_recurse(tail[0], &tail[1..], cache);
        }

        let mut t1 = 0;
        if tail[1] - base <= 3 {
            t1 = sequences_counter_recurse(tail[1], &tail[2..], cache);
        }

        possible_sequences = t0 + t1;

        cache.insert(base, possible_sequences);
    } else if tail.len() >= 3 {
        let mut t0 = 0;
        if tail[0] - base <= 3 {
            t0 = sequences_counter_recurse(tail[0], &tail[1..], cache);
        }

        let mut t1 = 0;
        if tail[1] - base <= 3 {
            t1 = sequences_counter_recurse(tail[1], &tail[2..], cache);
        }

        let mut t2 = 0;
        if tail[2] - base <= 3 {
            t2 = sequences_counter_recurse(tail[2], &tail[3..], cache);
        }

        possible_sequences = t0 + t1 + t2;

        cache.insert(base, possible_sequences);
    }

    possible_sequences
}

#[test]
fn test_sequences_counter_recurse() {
    assert_eq!(sequences_counter_recurse(0, &[1], &mut BTreeMap::new()), 1);

    assert_eq!(
        sequences_counter_recurse(0, &[1, 2], &mut BTreeMap::new()),
        2
    );
    assert_eq!(
        sequences_counter_recurse(0, &[1, 4], &mut BTreeMap::new()),
        1
    );

    assert_eq!(
        sequences_counter_recurse(0, &[1, 2, 3], &mut BTreeMap::new()),
        4
    );
    // 0 3
    // 0 1 3
    // 0 1 2 3
    // 0 2 3

    assert_eq!(
        sequences_counter_recurse(
            0,
            &[1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19],
            &mut BTreeMap::new()
        ),
        8
    );
}

fn determine_jolt_differences(input: &str) -> (u32, u32) {
    let mut parsed: Vec<u32> = input
        .lines()
        .map(|l| l.trim().parse::<u32>().unwrap())
        .collect();

    parsed.sort();

    println!("Parsed: {:?}", parsed);

    parsed.insert(0, 0);

    let mut differences: Vec<u32> = Vec::new();

    for pair in parsed.windows(2) {
        differences.push(pair[1] - pair[0]);
    }

    println!("Differences: {:?}", differences);

    (
        differences.iter().filter(|&d| *d == 1).count() as u32,
        (differences.iter().filter(|&d| *d == 3).count() + 1) as u32,
    )
}

#[test]
fn test_determine_jolt_differences() {
    assert_eq!(
        determine_jolt_differences(
            r"16
        10
        15
        5
        1
        11
        7
        19
        6
        12
        4"
        ),
        (7, 5)
    );
    assert_eq!(
        determine_jolt_differences(
            r"28
        33
        18
        42
        31
        14
        46
        20
        48
        47
        24
        23
        49
        45
        19
        38
        39
        11
        1
        32
        25
        35
        8
        17
        7
        9
        4
        2
        34
        10
        3"
        ),
        (22, 10)
    );
}
