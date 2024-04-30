use std::{cmp, collections::HashMap};

#[derive(Debug)]
struct Game {
    id: u32,
    turns: Vec<HashMap<String, u32>>
}

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;


pub fn part_01(s: &str) -> u32 {
    let res = s.split("\n")
        .map(|line| line2game(line))
        .filter(|game| is_valid_game(&game))
        .map(|game| game.id)
        .sum();
    res
}

pub fn part_02(s: &str) -> u32 {
    let res = s.split("\n")
        .map(|line| line2game(line))
        .map(|game| calculate_power(&game))
        .sum();
    res
}


fn line2game(line: &str) -> Game {
    let mut parts = line.split(':');
    let id = parts.next().expect("Game id part is missing").split(' ').last().unwrap().parse().unwrap();
    let game_turn_part = parts.next().expect("Game turn part is missing");
    let turns = get_turns(game_turn_part);
    Game {
        id,
        turns
    }
}

fn get_turns(game_turn_part: &str) -> Vec<HashMap<String, u32>> {
    game_turn_part.split(';').map(|s_turn| {
        get_turn(s_turn)
    }).collect()
}

fn get_turn(s: &str) -> HashMap<String, u32> {
    let mut turn: HashMap<String, u32> = HashMap::new();
    for s_cube in  s.split(',') {
        let mut cube = s_cube.trim().split(' ');
        let number_of_cubes = cube.next().expect("number_of_cubes not found").parse().expect("number_of_cubes should be a number");
        let color_of_cubes = cube.next().expect("color_of_cubes not found").to_string();
        turn.insert(color_of_cubes, number_of_cubes);
    }
    turn
}


fn is_valid_game(game: &Game) -> bool {
    let valid_turns = turns2rgb( &game.turns).into_iter()
    .filter(|rgb| rgb.0 <= MAX_RED && rgb.1 <= MAX_GREEN && rgb.2 <= MAX_BLUE).collect::<Vec<(u32, u32, u32)>>();
    valid_turns.len() ==  game.turns.len()
} 

fn calculate_power(game: &Game) -> u32 {
    let rgb = turns2rgb(&game.turns).into_iter()
    .reduce(|acc, e| (cmp::max(acc.0, e.0), cmp::max(acc.1, e.1), cmp::max(acc.2, e.2))).unwrap();
    rgb.0 * rgb.1 * rgb.2
}

fn turns2rgb(turns: &Vec<HashMap<String, u32>>) -> Vec<(u32, u32, u32)> {
    turns.into_iter().map(|turn| {
        let red = *turn.get("red").unwrap_or(&0);
        let green = *turn.get("green").unwrap_or(&0);
        let blue = *turn.get("blue").unwrap_or(&0);
        (red, green, blue)
    }).collect::<Vec<(u32, u32, u32)>>()
}

#[cfg(test)]
mod tests {
    use crate::*;
    const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn process_part_01() {
        let res =part_01(INPUT);
        assert_eq!(res, 8);
    }

    #[test]
    fn process_part_02() {
        let res = part_02(INPUT);
        assert_eq!(res, 2286);
    }

    #[test]
    fn process_line_001() {
        let game = line2game("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        assert_eq!(game.id, 1);
        assert!(is_valid_game(&game));
    }

    #[test]
    fn process_line_002_00() {
        let turns = get_turns("3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        assert_eq!(turns.len(), 3);
        let turn_1 = &turns[2];
        assert_eq!(turn_1["green"], 2);
    }

    #[test]
    #[should_panic]
    fn process_line_002_01() {
        let turns = get_turns("3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        assert_eq!(turns.len(), 3);
        let turn_1 = &turns[2];
        assert_eq!(turn_1["blue"], 2);
    }
}
