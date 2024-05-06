use std::fs;

use day_04::{part_01, part_02};
fn main() {
    let input = fs::read_to_string("input.txt").expect("No input file found");
    let res = part_01(&input); 
    println!("Res is {res}");

    let res2 = part_02(&input); 
    println!("Res2 is {res2}");
}
