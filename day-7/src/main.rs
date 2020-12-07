extern crate regex;
extern crate trees;

use regex::Regex;
use std::collections::HashSet;
use std::fs;
use trees::tr;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Dead file");

    let mut unique_colors: HashSet<&str> = HashSet::new();

    for line in input.split('\n') {
        let re = Regex::new(r"(.+?) bags contain(.+?)\.").unwrap();

        if let Some(caps) = re.captures(line) {
            if let (Some(container_bag), Some(contained_bags)) = (caps.get(1), caps.get(2)) {
                let re_contained_single = Regex::new(r"1 (.+?) bag").unwrap();
                let re_contained_several = Regex::new(r"(\d+) (.+?) bags").unwrap();

                println!(
                    "Bag: {:?} contains {:?}",
                    container_bag.as_str(),
                    contained_bags.as_str()
                );
            }
        }
    }
}
