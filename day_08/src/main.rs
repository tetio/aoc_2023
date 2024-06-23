mod exercises;

use std::fs;
use exercises::{part1, part2};

fn main() {
    let path = "input.txt";
    let input = fs::read_to_string(path).expect(format!("No file found::{}", path).as_str());
    let res1 = part1(&input);
    println!("Part1={}", res1);
    let res2 = part2(&input);
    println!("Part2={}", res2);
}
