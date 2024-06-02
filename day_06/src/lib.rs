struct Race {
    time: u64,
    distance: u64,
}

pub fn part1(input: &str) -> u64 {
    let races = init_races(input);
    let solution: Vec<u64> = races
        .into_iter()
        .map(|race| race2results(&race).len() as u64)
        .collect();
    solution.iter().product()
}

pub fn part2(input: &str) -> u64 {
    let races = init2_races(input);
    let solution: Vec<u64> = races
        .into_iter()
        .map(|race| race2results(&race).len() as u64)
        .collect();
    solution.iter().product()
}

fn race2results(race: &Race) -> Vec<u64> {
    (0..race.time)
        .map(|t| distance_traveled(race.time - t, t))
        .filter(|distance| distance > &race.distance)
        .collect::<Vec<u64>>()
}

fn init_races(input: &str) -> Vec<Race> {
    let mut races_info: Vec<Race> = vec![];
    let lines: Vec<&str> = input.split("\n").collect();
    let times = line2numbers(lines[0]);
    let distances = line2numbers(lines[1]);
    for race in 0..times.len() {
        let race_info = Race {
            time: times[race],
            distance: distances[race],
        };
        races_info.push(race_info);
    }
    races_info
}

fn init2_races(input: &str) -> Vec<Race> {
    let mut races_info: Vec<Race> = vec![];
    let lines: Vec<&str> = input.split("\n").collect();
    let time = line2numbers2(lines[0]);
    let distance = line2numbers2(lines[1]);
    let race_info = Race {
        time,
        distance,
    };
    races_info.push(race_info);
    races_info
}

fn line2numbers(line: &str) -> Vec<u64> {
    line.split(":").collect::<Vec<&str>>()[1]
        .split(" ")
        .into_iter()
        .filter(|s| s.trim().len() > 0)
        .map(|s| s.parse::<u64>().expect("bad number in spec"))
        .collect()
}

fn line2numbers2(line: &str) -> u64 {
    line.split(":").collect::<Vec<&str>>()[1]
        .replace(" ", "")
        .parse::<u64>()
        .expect("Not a valid namber in this line")
}

fn distance_traveled(time: u64, velocity: u64) -> u64 {
    velocity * time
}

#[cfg(test)]
mod tests {
    use crate::*;

    const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_line2numbers_001() {
        let res = line2numbers("Time:      7  15   30");
        assert_eq!(3, res.len());
        assert_eq!(7, res[0]);
        assert_eq!(15, res[1]);
        assert_eq!(30, res[2]);
    }

    #[test]
    fn test_init_races_001() {
        let res = init_races(INPUT);
        assert_eq!(3, res.len());
        assert_eq!(63, res[0].distance * res[0].time);
        assert_eq!(600, res[1].distance * res[1].time);
        assert_eq!(6000, res[2].distance * res[2].time);
    }

    #[test]
    fn test_01_001() {
        let res = part1(INPUT);
        assert_eq!(288, res, "wrong result");
    }

    #[test]
    fn test_02_001() {
        let res = part2(INPUT);
        assert_eq!(71503, res, "wrong result");
    }
}
