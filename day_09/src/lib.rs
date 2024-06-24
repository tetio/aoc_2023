//use itertools::Itertools;

pub fn part1(input: &str) -> i32 {
    input.split("\n").map(|line| process_sensor(line)).sum()
}

pub fn part2(_input: &str) -> i32 {
    0
}

fn process_sensor(line: &str) -> i32 {
    let data: Vec<i32> = line
        .split(" ")
        .map(|n| n.parse::<i32>().expect("(!) A number was expected!"))
        .collect();
    let history = calculate_differences(data);
    let last_values: Vec<i32> = history.into_iter().rev().map(|x| x[x.len() - 1]).collect();
    predict_value(last_values)
}

fn calculate_differences(data: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = vec![];
    result.push(data.clone());
    let mut current = &data;
    let mut diff: Vec<i32>;
    while !are_all_zeroes(current) {
        diff = make_differences(current);
        result.push(diff.clone());
        current = &diff;
    }
    result
}

fn are_all_zeroes(data: &Vec<i32>) -> bool {
    data.into_iter().filter(|d| **d != 0).count() == 0
}

fn predict_value(values: Vec<i32>) -> i32 {
    let mut value = 0;
    for i in 1..values.len() {
        value = values[i] + value;
    }
    value
}

fn make_differences(data: &Vec<i32>) -> Vec<i32> {
    let mut res = vec![];
    if data.len() > 0 {
        for i in 0..data.len() - 1 {
            res.push(data[i + 1] - data[i]);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scn_make_diferences_01() {
        let v = vec![-1, -2];
        let res = make_differences(&v);
        assert_eq!(res, vec![-1])
    }
    #[test]
    fn scn_make_diferences_02() {
        let v = vec![-1, -2, -3];
        let res = make_differences(&v);
        assert_eq!(res, vec![-1, -1])
    }
    #[test]
    fn scn_make_diferences_03() {
        let v = vec![1, 3, 9, 15];
        let res = make_differences(&v);
        assert_eq!(res, vec![2, 6, 6])
    }
    #[test]
    fn scn_make_diferences_04() {
        let v = vec![1, -3, 9, -15];
        let res = make_differences(&v);
        assert_eq!(res, vec![-4, 12, -24])
    }

    #[test]
    fn scn_process_sensor_01() {
        let v = "1 3 6 10 15 21";
        let res = process_sensor(v);
        assert_eq!(res, 28)
    }

    #[test]
    fn scn_process_sensor_02() {
        let v = "10 13 16 21 30 45";
        let res = process_sensor(v);
        assert_eq!(res, 68)
    }
}
