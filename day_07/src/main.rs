mod exercises;
use std::fs;
use crate::exercises::{part1, part2};

fn main() {
    let input = fs::read_to_string("input.txt").expect("*** File not found!");
    let res1 = part1(input.as_str());
    println!("Part1 = {}", res1);
    let res2 = part2(input.as_str());
    println!("Part2 = {}", res2);
}


// 249727675
// 249892896