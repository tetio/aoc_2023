use std::fs;

use day_06::{part1, part2};

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt file not fund");
    let result1 = part1(input.as_str());
    println!("Part1 {}", result1);

    let result2 = part2(input.as_str());
    println!("Part2 {}", result2);
}
