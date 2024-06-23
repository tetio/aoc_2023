use std::collections::HashMap;


pub fn part1(input: &str) -> u64 {
    let mut nodes: HashMap<String, (String, String)> = HashMap::new();
    let mut lines: Vec<&str> = input.split("\n").collect();

    let map: Vec<char> = lines[0].chars().collect();
    let mut look_ups = 0u64;
    let raw_nodes: Vec<&str> = lines.split_off(2);
    for raw_node in raw_nodes {
        create_entry2(raw_node.to_string(), &mut nodes);
    }
    let mut found = false;
    let mut map_index = 0;
    let mut current_node = "AAA".to_string();
    while !found {
        let node = &nodes[&current_node];
        current_node = match map[map_index] {
            'L' => node.0.clone(),
            'R' => node.1.clone(),
            _ => panic!("Invalid value in map")
        };
        if current_node == "ZZZ" {
            found = true;
        } else {
            map_index = (map_index + 1) % map.len();
        }
        look_ups += 1;
    }

    look_ups
}

pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

fn _create_entry(input: String) -> (String, String, String) {
    let line: Vec<_> = input.split("=").collect();
    let node_key = line[0].trim().to_owned();
    let s = line[1].replace("(", "").replace(")", "");
    let node_value: Vec<&str> = s.split(",").collect();
    let left_node = node_value[0].trim().to_owned();
    let right_node = node_value[1].trim().to_owned();
    (node_key, left_node, right_node) 
}

fn create_entry2<'a>(input: String, hm: &'a mut HashMap<String, (String, String)>) {
    let line: Vec<_> = input.split("=").collect();
    let node_key = line[0].trim().to_owned();
    let s = line[1].replace("(", "").replace(")", "");
    let node_value: Vec<&str> = s.split(",").collect();
    let left_node = node_value[0].trim().to_owned();
    let right_node = node_value[1].trim().to_owned();
    hm.insert(node_key, (left_node, right_node)); 
}

fn traverse(nodes: &HashMap<String, (String, String)>, map: &Vec<char>, current_node: String) -> usize {
    let mut look_ups: usize = 0;
    let mut found = false;
    let mut map_index = 0;
    let mut aux_node = current_node;
    while !found {
        let node = &nodes[&aux_node];
        aux_node = match map[map_index] {
            'L' => node.0.clone(),
            'R' => node.1.clone(),
            _ => panic!("Invalid value in map")
        };
        if aux_node.ends_with("Z") {
            found = true;
        } else {
            map_index = (map_index + 1) % map.len();
        }
        look_ups += 1;
    }  
    println!("Lookup={}", look_ups);
    look_ups  
}

pub fn part2(input: &str) -> u64 {
    let mut nodes: HashMap<String, (String, String)> = HashMap::new();
    let mut lines: Vec<&str> = input.split("\n").collect();

    let map: Vec<char> = lines[0].chars().collect();
    let raw_nodes: Vec<&str> = lines.split_off(2);
    for raw_node in raw_nodes {
        create_entry2(raw_node.to_string(), &mut nodes);
    }
    let init_nodes: Vec<String> = nodes.keys().into_iter().filter(|n| n.ends_with("A")).map(|m| m.clone()).collect(); 
    let mut lll: Vec<usize> = init_nodes.into_iter().map(|node| traverse(&nodes, &map, node)).collect();
    lll.push(1);
    let array = lll.split_last().unwrap().1;
    lcm(array) as u64
}


fn _all_end_nodes(nodes: &Vec<String>, num_nodes: usize) -> bool {
    nodes.into_iter().filter(|n| n.ends_with("Z")).count() == num_nodes
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::*;

    const INPUT: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"; 

    const INPUT2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    const INPUT3: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn create_entry_001() {
        let res = _create_entry("CCC = (ZZZ, GGG)".to_string());
        assert_eq!(res.0, "CCC");
        assert_eq!(res.1, "ZZZ");
        assert_eq!(res.2, "GGG");
    }

    #[test]
    fn create_entry2_001() {
        let mut nodes: HashMap<String, (String, String)> = HashMap::new();
        create_entry2("CCC = (ZZZ, GGG)".to_string(), &mut nodes);
        assert_eq!(nodes["CCC"].0, "ZZZ");
        assert_eq!(nodes["CCC"].1, "GGG");
    }

    #[test]
    fn part1_001() {
        let res = part1(INPUT);
        assert_eq!(res, 2);
    }

    #[test]
    fn part1_002() {
        let res = part1(INPUT2);
        assert_eq!(res, 6);
    }


    #[test]
    fn part2_001() {
        let res = part2(INPUT3);
        assert_eq!(res, 6);
    }

}


