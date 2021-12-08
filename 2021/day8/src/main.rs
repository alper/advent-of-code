use std::fs;
use std::collections::{HashMap, HashSet};
use itertools::{Itertools};

fn main() {
    let input = fs::read_to_string("input.txt").expect("File not readable");

    let output_digits: Vec<_> = input.lines().map(|l| l.split('|').nth(1).unwrap().trim()).collect();

    let simple_digits: Vec<usize> = output_digits
        .iter()
        .map(|ds|
            ds.split(' ')
                .filter(|d| matches!(d.len(), 2 | 3 | 4 | 7)).count()
                )
        .collect();

    println!("Answer 1: {:?}", simple_digits.iter().sum::<usize>());

    // Part 2



}

#[test]
fn test_get_wiring() {
    assert_eq!(get_wiring("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab"), ['d', 'e', 'a', 'f', 'g', 'b', 'c']);

}

fn get_wiring(groups: &str) -> [char; 7] {
    let mut h = parse_frequencies(groups);

    let four_vec = h.remove(&4).unwrap();
    let four = four_vec.first().unwrap();
    println!("Four: {:?}", four);

    let one_vec = h.remove(&2).unwrap();
    let one = one_vec.first().unwrap();
    println!("One: {:?}", one);

    let eight_vec = h.remove(&7).unwrap();
    let eight = eight_vec.first().unwrap();
    println!("Eight: {:?}", eight);

    let seven_vec = h.remove(&3).unwrap();
    let seven = seven_vec.first().unwrap();
    println!("Seven: {:?}", seven);

    // Get the six
    let mut six_segs_3 = h.get(&6).unwrap();
    let six_vec: Vec<_> = six_segs_3.iter().filter(|&s| s.intersection(&one).count() == 1).collect();
    let six = six_vec.first().unwrap().clone();

    println!("Six: {:?}", six);

    // Remove the six
    let six_segs_2: Vec<_> = six_segs_3.iter().filter(|&s| s.intersection(&one).count() != 1).collect();

    // Get the three
    let five_segs_3 = h.get(&5).unwrap();
    let three_vec: Vec<_> = five_segs_3.iter().filter(|&s| s.is_superset(one)).collect();
    let three = three_vec.first().unwrap().clone();

    println!("Three: {:?}", three);

    // Remove the three
    let five_segs_2: Vec<_> = five_segs_3.iter().filter(|&s| s != three).collect();

    println!("Five_2: {:?}", five_segs_2);

    // Get the five
    let five_vec: Vec<_> = five_segs_2.iter().filter(|&s| six.is_superset(s)).collect();
    let five = five_vec.first().unwrap().clone();

    println!("Five: {:?}", five);

    // Get the two
    let two_vec: Vec<_> = five_segs_2.iter().filter(|&s| s != five).collect();
    let two = two_vec.first().unwrap().clone();
    println!("Two: {:?}", two);

    // Get the nine
    let nine_vec: Vec<_> = six_segs_2.iter().filter(|&s| s.is_superset(four)).collect();
    let nine = nine_vec.first().unwrap().clone();

    println!("Nine: {:?}", nine);

    let zero_vec: Vec<_> = six_segs_2.iter().filter(|&s| s != nine).collect();
    let zero = zero_vec.first().unwrap().clone();

    println!("Zero: {:?}", zero);

    let d = eight.difference(zero).next().unwrap();
    let c = eight.difference(six).next().unwrap();
    let e = eight.difference(nine).next().unwrap();
    let b = nine.difference(three).next().unwrap();
    let a = seven.difference(one).next().unwrap();
    let g = zero.difference(seven).filter(|&w| w != b && w != e).next().unwrap();
    let f = one.iter().filter(|&w| w != c).next().unwrap();

    println!("{} {} {} {} {} {} {}", a, b, c, d, e, f, g);

    return [*a, *b, *c, *d, *e, *f, *g];
}

#[test]
fn test_parse_frequencies() {
    let h = parse_frequencies("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab");

    println!("{:?}", h);

    assert_eq!(h.get(&2).unwrap().len(), 1);
    assert_eq!(h.get(&4).unwrap().len(), 1);
}

fn parse_frequencies(s: &str) -> HashMap<usize, Vec<HashSet<char>>> {
    let parts: Vec<&str> = s.trim().split(' ').collect();

    let mut h = HashMap::new();

    for i in 2..8 {
        h.insert(i,
            parts
                .clone()
                .into_iter()
                .filter(|&p| p.len() == i)
                .map(|s| s.chars().collect::<HashSet<char>>())
                .collect()
            );

    }

    h
}