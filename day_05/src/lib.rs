use chrono::{prelude::*, Duration};

const MAP_STATES: [&'static str; 9] = [
    "none",
    "seed",
    "seed-to-soil",
    "soil-to-fertilizer",
    "fertilizer-to-water",
    "water-to-light",
    "light-to-temperature",
    "temperature-to-humidity",
    "humidity-to-location"
];

#[derive(Debug, Default)]
struct Almanac {
    seeds: Vec<u32>,
    seed_to_soil: Vec<(u64, u64, u64)>,
    soil_to_fertilizer: Vec<(u64, u64, u64)>,
    fertilizer_to_water: Vec<(u64, u64, u64)>,
    water_to_light: Vec<(u64, u64, u64)>,
    light_to_temperature: Vec<(u64, u64, u64)>,
    temperature_to_humidity: Vec<(u64, u64, u64)>,
    humidity_to_location: Vec<(u64, u64, u64)>
}


pub fn part_01(input: &str) -> u32 {
    let almanac = build_almanac(input);
    // println!("almanac {:?}", almanac );
    let location = almanac.seeds.into_iter()
        .map(|seed| translate_map(&almanac.seed_to_soil, seed as u64))
        .map(|soil| translate_map(&almanac.soil_to_fertilizer, soil))
        .map(|fertilizer| translate_map(&almanac.fertilizer_to_water, fertilizer))
        .map(|water| translate_map(&almanac.water_to_light, water))
        .map(|light| translate_map(&almanac.light_to_temperature, light))
        .map(|temperature| translate_map(&almanac.temperature_to_humidity, temperature))
        .map(|humidity| translate_map(&almanac.humidity_to_location, humidity)).min();
    location.unwrap() as u32
}

fn build_almanac(input: &str) -> Almanac {
    let mut almanac: Almanac  = Almanac::default();
    let mut state = 0;

    input.split("\n").for_each(|line| {
        let is_end_of_section = line.is_empty();
        if !is_end_of_section && state > 0 {
            add_entry_to_corresponding_map(state, line, &mut almanac);
        } else if is_end_of_section && state == 1 {
            state += 1;
        } else {
            // check state
            match state {
                0 => (state, almanac.seeds) = get_seeds(line),
                _ =>  state += 1,
            }
        }
    });
    almanac
}

fn translate_map(maps: &Vec<(u64, u64, u64)>, pos: u64) -> u64 {
    let res = maps.into_iter().map(|map| translate(*map, pos)).filter(|t| t.is_some()).collect::<Vec<_>>();
    if res.len() > 0 {
        res[0].unwrap()
    } else {
        pos
    }
}
fn translate(map: (u64, u64, u64), pos: u64) -> Option<u64> {
    if pos < map.1 || pos  > map.1 + map.2 - 1 {
        None
    } else {
        Some(map.0 + (pos - map.1))
    }
}
fn get_seeds(line: &str) -> (u32, Vec<u32>) {
    (1, line.split(":").last().unwrap().trim().split(" ").map(|s| s.trim().parse::<u32>().expect("This shpukd be a seed")).collect())
}

fn add_entry_to_corresponding_map(state: u32, line: &str, almanac: &mut Almanac) {
    if line.ends_with(":") {
        return
    }
    match state {
        2 => almanac.seed_to_soil.push(string_to_source_destination_entry(line)),
        3 => almanac.soil_to_fertilizer.push(string_to_source_destination_entry(line)),
        4 => almanac.fertilizer_to_water.push(string_to_source_destination_entry(line)),
        5 => almanac.water_to_light.push(string_to_source_destination_entry(line)),
        6 => almanac.light_to_temperature.push(string_to_source_destination_entry(line)),
        7 => almanac.temperature_to_humidity.push(string_to_source_destination_entry(line)),
        8 => almanac.humidity_to_location.push(string_to_source_destination_entry(line)),
        _ => (),
    };
    
}

fn string_to_source_destination_entry(line: &str) -> (u64, u64, u64) {
    let res = line.split(" ").into_iter().map(|s| s.parse::<u64>().expect("source destination is not a number")).collect::<Vec<u64>>();
    (res[0], res[1], res[2])
}


pub fn part_02(input: &str) -> u32 {
    let mut almanac = build_almanac(input);
    let mut locations: Vec<u32> = vec![];
    //almanac.seeds = recalculate_seeds(almanac.seeds);
    for i in (0..almanac.seeds.len()-1).step_by(2) {
        println!("Loop #{} at {}", i, Local::now().to_string());
        let mut res: Vec<u32> = vec![];
        for j in 0..(almanac.seeds[i+1]-1) {
            res.push(almanac.seeds[i]+j);
        }

        let location = res.into_iter()
            .map(|seed| translate_map(&almanac.seed_to_soil, seed as u64))
            .map(|soil| translate_map(&almanac.soil_to_fertilizer, soil))
            .map(|fertilizer| translate_map(&almanac.fertilizer_to_water, fertilizer))
            .map(|water| translate_map(&almanac.water_to_light, water))
            .map(|light| translate_map(&almanac.light_to_temperature, light))
            .map(|temperature| translate_map(&almanac.temperature_to_humidity, temperature))
            .map(|humidity| translate_map(&almanac.humidity_to_location, humidity)).min().unwrap();
        locations.push(location as u32);
        
        let local: DateTime<Local> = Local::now();
        println!("{} - {}", local.to_string(), location)
        
    }
    locations.into_iter().min().unwrap()
}

fn recalculate_seeds(seeds: Vec<u32>) -> Vec<u32> {
    let mut res: Vec<u32> = vec![];
    for i in (0..seeds.len()-1).step_by(2) {
        for j in 0..(seeds[i+1]-1) {
            res.push(seeds[i]+j)
        }
    }

    res
}


#[cfg(test)]
mod tests {
    use crate::*;

    const INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";


    #[test]
    fn test_01_001() {
        let res = part_01(INPUT);
        assert_eq!(res, 35)
    }

    #[test]
    fn test_02_001() {
        let res = part_02(INPUT);
        assert_eq!(res, 46)
    }
}