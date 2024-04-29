use std::fs;

use day_02::part_01;

const DATA_FILE: &str = "input.txt";
fn main() {
    let data = fs::read_to_string(DATA_FILE).expect(format!("File {} not found", DATA_FILE).as_str());
    let part_01 = part_01(data.as_str());
    println!("Part_01 = {}", part_01);
}
