use std::fs;

use day_05::{part_01, part_02};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Input.txt not found");
    let res_1 = part_01(input.as_str());
    println!("Part_01 is {}", res_1);
    let res_2 = part_02(input.as_str());
    println!("Part_02 is {}", res_2);

}
