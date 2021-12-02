#!/usr/local/bin/fish

set -l day $argv[1]
set -l proj_name "day$day"

cargo new $proj_name

cd $proj_name

touch input.txt

echo 'use std::fs;
use itertools::{Itertools};

fn main() {
    let input = fs::read_to_string("input.txt").expect("File not readable");

    println!("Input: {:?}", input);
}' > src/main.rs

cargo add itertools

