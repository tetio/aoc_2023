use std::collections::HashMap;

pub fn part_01(input: &str) -> u32 {
    input.split("\n").map(|line| calculate_winning_numbers(line)).sum()
}

fn calculate_winning_numbers(line: &str) -> u32 {
    let s_numbers = line.split("|").collect::<Vec<&str>>();
    let winning_numbers = s_numbers[0].split(":").collect::<Vec<&str>>()[1]
        .split(" ").map(|s| parse_number(s.trim())).filter(|x| *x > 0).collect::<Vec<u32>>();
    let card_numbers = s_numbers[1].split(" ").map(|s| parse_number(s.trim())).filter(|x| *x > 0).collect::<Vec<u32>>();
    let correct_numbers = card_numbers.into_iter().filter(|n| winning_numbers.contains(n)).collect::<Vec<u32>>();
    final_score(correct_numbers.len() as u32)
}

fn parse_number(s: &str) -> u32 {
    match s.trim().parse::<u32>() {
        Ok(n) => n,
        _ => 0
    }
}

fn final_score(len: u32) -> u32 {
    if len == 0 {
        0
    } else {
        let two: u32 = 2;
        two.pow(len-1)
    }
}


pub fn part_02(input: &str) -> u32 {
    let mut hm: HashMap<u32, (u32, u32)> = HashMap::new();
    input.split("\n").for_each(|line| {
        let s_numbers = line.split("|").collect::<Vec<&str>>();
        let winning_numbers = s_numbers[0].split(":").collect::<Vec<&str>>()[1]
            .split(" ").map(|s| parse_number(s.trim())).filter(|x| *x > 0).collect::<Vec<u32>>();
        let card_numbers = s_numbers[1].split(" ").map(|s| parse_number(s.trim())).filter(|x| *x > 0).collect::<Vec<u32>>();
        let card = s_numbers[0].split(":").collect::<Vec<&str>>()[0].split("Card").collect::<Vec<&str>>()[1].trim().parse::<u32>().expect("key not a number");
        let correct_numbers = card_numbers.into_iter().filter(|n| winning_numbers.contains(n)).collect::<Vec<u32>>().len() as u32;
        hm.insert(card, (correct_numbers, 1));
    });
    let max_card = hm.len() as u32;
    //let mut res: HashMap<u32, (u32, u32)> = hm.clone();
    for k in 1 .. max_card {
        let max_key = k + hm[&k].0;
        // println!("Clau {}, {} vegades", k, res[&k].1);
        for _ in 0 .. hm[&k].1 {
            for new_key in k+1..=max_key {
                if new_key <= max_card {
                    let n = hm[&new_key];
                    // println!("Actualitzo clau {} amb el nou valor {}", new_key, n.1+1);
                    hm.insert(new_key, (n.0, n.1+1));            }
            }
        }
    };
    hm.values().map(|x| x.1).sum()
}

#[cfg(test)]
mod tests {
    use crate::*;

    const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_part_01_001() {
        let res = part_01(INPUT);
        assert_eq!(res, 13)
    }

    #[test]
    fn test_part_02_001() {
        let res = part_02(INPUT);
        assert_eq!(res, 30)
    }


}