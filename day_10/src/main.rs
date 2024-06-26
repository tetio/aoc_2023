use std::fs;

use day_10::{part1, part2};

fn main() {
    let input = fs::read_to_string("input.txt").expect("(!) input.txt file not found.");
    let res1 = part1(&input);
    println!("Part1={}", res1);
    let res2 = part2(&input);
    println!("Part2={}", res2);
}
