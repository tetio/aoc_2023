mod exercises;
use std::fs;
use crate::exercises::part1;

fn main() {
    let input = fs::read_to_string("input.txt").expect("*** File not found!");
    let res1 = part1(input.as_str());
    println!("Part1 = {}", res1);
}
