
use std::collections::HashMap;
use lazy_static::lazy_static;

//const NUMBERS0_9: [&'static str; 10] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

lazy_static! {
    static ref NUMBERS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        //m.insert("zero", "0");
        m.insert("nine", "9");
        m.insert("eight", "8");
        m.insert("seven", "7");
        m.insert("six", "6");
        m.insert("five", "5");
        m.insert("four", "4");
        m.insert("three", "3");
        m.insert("two", "2");
        m.insert("one", "1");

        m
    };
}

pub fn sum_all_calibration_values(s: &str) -> u32 {
    s.split("\n")
        .map(|row| {
            let first = find_first_number(row);
            let second = find_second_number(row);
            format!("{}{}", first, second)
        })
        .map(|s_number| s_number.parse::<u32>().unwrap())
        .sum()
}



pub fn part_02_01(s: &str) -> u32 {
    let res =  s.split("\n").map(|s| {
    let mut all_done = false;
    let mut ss = s;
    let mut line = "".to_string();
        while !all_done {
            let next_number = first_number_in_letters(ss);
            if next_number.0 == usize::MAX {
                //line.push_str(s);
                all_done = true;
                line.push_str(ss);
            } else {
                let number = NUMBERS.get(next_number.1).unwrap();
                line.push_str(&ss[..next_number.0]);
                line.push_str(*number);
                ss = &ss[next_number.0+next_number.1.len()..];
            }
        }
        line
    }).collect::<Vec<_>>().join("\n");
    println!("---------");
    println!("{}", res);
    println!("---------");
    sum_all_calibration_values(res.as_str())  
}


pub fn part_02(s: &str) -> u32{
    let res =  s.split("\n").map(|s| {
        let first = first_number_in_letters(s);
        let last = last_number_in_letters(s);
        // if first.0 == usize::MAX {
        //     s.to_string()
        // } else if first.0+first.1.len()-1 < last.0 {
        //     found_two_numbers(first, last, s)
        // } else {
        //     found_one_number(first, s)
        // }
        if first.0 == usize::MAX {
            s.to_string()
        } else {
            found_two_numbers(first, last, s)
        }
    }).collect::<Vec<_>>().join("\n");
    println!("---------");
    println!("{}", res);
    println!("---------");
    sum_all_calibration_values(res.as_str())   
}

fn found_one_number(first: (usize, &str), s: &str) -> String {
    let mut aux_res = "".to_string();
    let a = first.0+first.1.len();
    let ab = &s[a..];
    let first_number = NUMBERS.get(first.1).unwrap();
    aux_res.push_str(&s[..first.0]);
    aux_res.push_str(*first_number);
    aux_res.push_str(ab);
    aux_res
}

fn found_two_numbers(first: (usize, &str), last: (usize, &str), s: &str) -> String {
    let mut aux_res = "".to_string();
    let a = first.0+(first.1.len());
    let b = last.0;
    //let ab = &s[a..b];

    let first_number = NUMBERS.get(first.1).unwrap();
    let last_number = NUMBERS.get(last.1).unwrap();
    aux_res.push_str(&s[..first.0]);
    aux_res.push_str(*first_number);
    //aux_res.push_str(ab);
    aux_res.push_str(*last_number);
    let bb = b+(last.1.len());
    aux_res.push_str(&s[bb..]);
    aux_res
}



pub fn first_number_in_letters(s: &str) -> (usize, &str) {
    let mut min_pos = usize::MAX;
    let mut number = "";
    for key in NUMBERS.keys() {
        let pos = s.find(key);
        if pos.is_some() && pos.unwrap() < min_pos {
            min_pos = pos.unwrap();
            number = key;
        }
    }
    (min_pos, number)
}


pub fn last_number_in_letters(s: &str) -> (usize, &str) {
    let mut max_pos = 0;
    let mut number = "";
    for key in NUMBERS.keys() {
        let pos = s.rfind(key);
        if pos.is_some() && pos.unwrap() >= max_pos {
            max_pos = pos.unwrap();
            number = key;
        }
    }
    (max_pos, number)
}

fn find_first_number(s: &str) -> &str {
    let pos = s.find(char::is_numeric).unwrap();
    s.get(pos..pos+1).unwrap()
}

fn find_second_number(s: &str) -> &str {
    let pos = s.rfind(char::is_numeric).unwrap();
    s.get(pos..pos+1).unwrap()
}


/* 
fn find_numbers_in_letters(s: &str) -> Vec<(usize, &str)> {
    let mut res = NUMBERS.clone().into_keys()
        .map(|num| {
            match s.rfind(num) {
                Some(pos) => (pos, num),
                _ => (usize::MAX, "")
            }
        })
        .filter(|(a, _b)| *a < usize::MAX)
        .collect::<Vec<(usize, &str)>>();
    res.sort();
    res
}

*/



#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet";

    const INPUT2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn test_sum_all_calibration_values() {
        let res = sum_all_calibration_values(INPUT);   
        assert_eq!(142, res);    
    } 

    
    #[test]
    fn test_sum_all_calibration_values_001_000() {
        let res = sum_all_calibration_values("drsgdrrgscqmsggrgq1fsqjhtkkrltt");   
        assert_eq!(11, res);    
    } 

    #[test]
    fn test_sum_all_calibration_values_001() {
        let res = sum_all_calibration_values("219");   
        assert_eq!(29, res);    
    } 

    #[test]
    fn test_part_two_001() {
        let res = part_02("two1nine");   
        assert_eq!(29, res);    
    } 

    #[test]
    fn test_part_two_002() {
        let res = part_02("xfive1nine");   
        assert_eq!(59, res);   
    } 
    #[test]
    fn test_part_two_003() {
        let res = part_02("1two1nine1"); 
        assert_eq!(11, res);   
    }
    #[test]
    fn test_part_two_004() {
        let res = part_02("xtwone3four"); 
        assert_eq!(24, res);   
    }
    #[test]
    fn test_part_two_005() {
        let res = part_02("zoneight234"); 
        assert_eq!(14, res);   
    }
    #[test]
    fn test_part_two_006() {
        let res = part_02("xtwoneight2"); 
        assert_eq!(22, res);   
    }
    #[test]
    fn test_part_two_007() {
        let res = part_02("xoneight2"); 
        assert_eq!(12, res);   
    }
    #[test]
    fn test_part_two_008() {
        let res = part_02("xdsdsdsoneeightsdsds"); 
        assert_eq!(18, res);   
    }
    #[test]
    fn test_part_two_009() {
        let res = part_02("xdsdsdsoneeeightsdsds"); 
        assert_eq!(18, res);   
    }
    #[test]
    fn test_part_two_010() {
        let res = part_02("7sixthreerzmpbffcx"); 
        assert_eq!(73, res);   
    }
    #[test]
    fn test_part_two_011() {
        let res = part_02("asdfgsevenkdjkdjdd"); 
        assert_eq!(77, res);   
    }

    #[test]
    fn test_part_two_012() {
        let res = part_02("5two6xxkzdrbfsix"); 
        assert_eq!(56, res);   
    }

    #[test]
    fn test_part_two_013() {
        let res = part_02("5sevenine"); 
        assert_eq!(59, res);   
    }    


    #[test]
    fn test_part_two_014_00() {
        let res = part_02_01("fiveight"); 
        assert_eq!(55, res);   
    } 

    #[test]
    fn test_part_two_014_01() {
        let res = part_02("fiveight"); 
        assert_eq!(58, res);   
    }

    #[test]
    fn test_part_two_999() {
        let res = part_02(INPUT2); 
        assert_eq!(281, res);   
    }
}
