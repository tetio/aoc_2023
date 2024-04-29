use std::fs;

use day_01::{part_01, part_02};

fn main() {
    part01();
    part02(); 
}

fn part01() {
    let data = fs::read_to_string("input.txt").expect("imput.txt was not found");
    let res = part_01(data.as_str());
    println!("Part One  is {res}");
}

fn part02() {
    let data = fs::read_to_string("input.txt").expect("imput.txt was not found");
    let res = part_02(data.as_str());
    println!("Part two is {res}");
}