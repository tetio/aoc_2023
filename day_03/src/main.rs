use std::fs;

use day_03::{part_01, part_02};
fn main() {
    let input = fs::read_to_string("input.txt").expect("intput not found.");
    let res = part_01(input.as_str());
    println!("part_01 = {}", res);

    let res2 = part_02(input.as_str());
    println!("part_02 = {}", res2);
}
