#[derive(Debug, Default, PartialEq)]
struct Number {
    as_chars: Vec<char>,
    value: u32,
    coords: Vec<(u32,u32)>,
    surrounding_coords: Vec<(u32, u32)>,
}

pub fn part_01(input: &str) -> u32 {
    let lines: Vec<&str> = input.split("\n").collect();
    let mut line_num = 0;
    let mut numbers_aux: Vec<Vec<Number>>  = vec![];
    let mut parts_aux: Vec<Vec<(u32,u32)>> = vec![];
    
    for line in lines {
        let elements = get_elements(line_num, line);
        if !elements.0.is_empty() {
            numbers_aux.push(elements.0);
        }
        if !elements.1.is_empty() {
            parts_aux.push(elements.1);
        }        
        line_num += 1;
    } 
    let numbers = numbers_aux.into_iter().flatten().collect::<Vec<Number>>();
    let parts = parts_aux.into_iter().flatten().collect::<Vec<(u32,u32)>>();
    numbers.into_iter().filter(|n| is_it_a_part(&n, &parts)).map(|n| n.value).sum()
}





fn is_it_a_part(n: &Number, parts: &Vec<(u32, u32)>) -> bool {
    n.surrounding_coords.iter().filter(|coord| parts.contains(coord)).map(|_| 1).sum::<u32>() > 0 
}

fn get_elements(row: u32, line: &str) -> (Vec<Number>, Vec<(u32,u32)>) {
    let mut numbers: Vec<Number> = Vec::new();
    let mut parts: Vec<(u32,u32)> = Vec::new();
    let mut is_number_on = false;
    let mut number = Number::default();
    for (i, c) in line.chars().enumerate() {
        if c.is_digit(10) && is_number_on {
            number.as_chars.push(c);
            number.coords.push((i as u32, row));
        } else if c.is_digit(10) && !is_number_on {
            is_number_on = true;
            number = Number { as_chars: vec![c], value: 0, coords: vec![(i as u32, row)], surrounding_coords: vec![], };          
        } else if c != '.' {
            parts.push((i as u32, row));
        }
        if (!c.is_digit(10) && is_number_on) || (c.is_digit(10) && i == line.len() - 1) {
            is_number_on = false;
            number.value = number.as_chars.iter().collect::<String>().parse::<u32>().unwrap();
            number.surrounding_coords = number.coords.iter().map(|(x, y)| generate_surrounding_coords(*x, *y)).flatten().collect();
            numbers.push(number);
            number = Number::default(); 
        }
    }
    (numbers, parts)
}   


fn get_elements_02(row: u32, line: &str) -> (Vec<Number>, Vec<(u32,u32)>) {
    let mut numbers: Vec<Number> = Vec::new();
    let mut parts: Vec<(u32,u32)> = Vec::new();
    let mut is_number_on = false;
    let mut number = Number::default();
    for (i, c) in line.chars().enumerate() {
        if c.is_digit(10) && is_number_on {
            number.as_chars.push(c);
            number.coords.push((i as u32, row));
        } else if c.is_digit(10) && !is_number_on {
            is_number_on = true;
            number = Number { as_chars: vec![c], value: 0, coords: vec![(i as u32, row)], surrounding_coords: vec![], };          
        } else if c == '*' {
            parts.push((i as u32, row));
        }
        if (!c.is_digit(10) && is_number_on) || (c.is_digit(10) && i == line.len() - 1) {
            is_number_on = false;
            number.value = number.as_chars.iter().collect::<String>().parse::<u32>().unwrap();
            number.surrounding_coords = number.coords.iter().map(|(x, y)| generate_surrounding_coords(*x, *y)).flatten().collect();
            numbers.push(number);
            number = Number::default(); 
        }
    }
    (numbers, parts)
}


fn generate_surrounding_coords(x: u32, y: u32) -> Vec<(u32,u32)> {
    if x>0 && y >0 {
        vec![
            (x-1, y-1), (x-1, y), (x-1, y+1),
            (x, y-1), (x, y+1),
            (x+1, y-1), (x+1, y), (x+1, y+1),
        ]
    } else if x == 0 && y == 0{
        vec![
           (x, y+1),
             (x+1, y), (x+1, y+1),
        ]
    } else if x > 0 && y == 0 {
        vec![
            (x-1, y), (x-1, y+1),
            (x, y+1),
            (x+1, y), (x+1, y+1),
        ]
    } else if x == 0 && y > 0 {
        vec![
            (x, y-1), (x, y+1),
            (x+1, y-1), (x+1, y), (x+1, y+1),
        ]
    } else {
        vec![]
    }
}

pub fn part_02(input: &str) -> u32 {
    let lines: Vec<&str> = input.split("\n").collect();
    let mut line_num = 0;
    let mut numbers_aux: Vec<Vec<Number>>  = vec![];
    let mut parts_aux: Vec<Vec<(u32,u32)>> = vec![];
    
    for line in lines {
        let elements = get_elements_02(line_num, line);
        if !elements.0.is_empty() {
            numbers_aux.push(elements.0);
        }
        if !elements.1.is_empty() {
            parts_aux.push(elements.1);
        }        
        line_num += 1;
    } 
    let numbers = numbers_aux.into_iter().flatten().collect::<Vec<Number>>();
    let parts = parts_aux.into_iter().flatten().collect::<Vec<(u32,u32)>>();
    //numbers.into_iter().filter(|n| is_it_a_part(&n, &parts)).map(|n| n.value).sum()
    parts.into_iter().map(|c| numbers_around_this_point(c, &numbers))
    .map(|part_numbers| part_numbers.0*part_numbers.1)
    .sum()
}

fn numbers_around_this_point(c:(u32, u32), numbers: &Vec<Number>) -> (u32, u32) {
    let coords = generate_surrounding_coords(c.0, c.1);
    let valid_numbers = find_valid_numbers(numbers, &coords);
    
    if valid_numbers.len() == 2 {
        (valid_numbers[0], valid_numbers[1])
    } else {
        (0 ,0)
    }
}

fn find_valid_numbers(numbers: &Vec<Number>, coords: &Vec<(u32, u32)>) -> Vec<u32> {
    let mut aux_nums:Vec<&Number> = vec![];
    coords.into_iter().map(|x| {
        let mut res = vec![];
        for num in numbers {
            if valid_coord(&num.coords, *x) && !aux_nums.contains(&num) {
                res.push(num.value);
                aux_nums.push(num);
                break;
            }
        }
        res
    }).flatten().collect::<Vec<u32>>()   
}

fn valid_coord(coords: &Vec<(u32, u32)>, coord: (u32, u32)) -> bool {
    let mut res = false;
    for c in coords {
        if c.0 == coord.0 && c.1 == coord.1 {
            res = true;
        }
    }
    res
}



#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str= "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_get_elements_001() {
        let input = "....................";
        let result = get_elements(0,input);
        assert!(result.0.is_empty());
        assert!(result.1.is_empty());
    }
    #[test]
    fn test_get_elements_002() {
        let input = "#..........#.......?";
        let result = get_elements(0,input);
        assert!(result.0.is_empty());
        assert_eq!(result.1.len(), 3);
    }
    #[test]
    fn test_get_elements_003() {
        let input = "%&%&%&%&%&%&%&%&%&%&";
        let result = get_elements(0,input);
        assert!(result.0.is_empty());
        assert_eq!(result.1.len(), 20);
    }    
    #[test]
    fn test_get_elements_010() {
        let input = "......456...........";
        let result = get_elements(0,input);
        assert_eq!(result.0[0].value, 456);
        assert!(result.1.is_empty());
    }
    #[test]
    fn test_get_elements_011() {
        let input = "...................1";
        let result = get_elements(0,input);
        assert_eq!(result.0[0].value, 1);
        assert!(result.1.is_empty());
    }
    #[test]
    fn test_get_elements_012() {
        let input = "1.23..456..2.55555.1";
        let result = get_elements(0,input);
        assert_eq!(result.0.len(), 6);
        assert_eq!(result.0[0].value, 1);
        assert_eq!(result.0[3].value, 2);
        assert_eq!(result.0[4].value, 55555);
        assert_eq!(result.0[5].value, 1);
        assert!(result.1.is_empty());
    }

    #[test]
    fn test_part_01_001() {
        let res = part_01(INPUT);
        assert_eq!(res, 4361);
    }

    #[test]
    fn test_part_02_001() {
        let res = part_02(INPUT);
        assert_eq!(res, 467835);
    }
}


