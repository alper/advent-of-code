#!/usr/local/bin/fish

set -l day $argv[1]
set -l proj_name "day$day"

if test -d $proj_name
    exit 1
end

cargo new $proj_name

cd $proj_name

touch input.txt
touch test_input.txt

echo 'use std::fs;
use itertools::{Itertools};

fn main() {
    let input = fs::read_to_string("test_input.txt").expect("File not readable");

    println!("Input: {:?}", input);
}' >src/main.rs

cargo add itertools
