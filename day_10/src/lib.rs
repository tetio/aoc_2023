
pub fn part1(input: &str) -> i32 {
    let map: Vec<Vec<char>> = input
        .split("\n")
        .map(|line| line.chars().into_iter().collect::<Vec<char>>())
        .collect();

    let start_pos = where_is_s(&map);
    let mut current_postion = start_pos;
    let mut res: Vec<(i32, i32)> = Vec::new();
    let mut first = true;
    while current_postion != start_pos || first {
        first = false;
        current_postion = next_move(&map, current_postion, &res);
        res.push(current_postion);
        println!("***  current_postion = {:?}", current_postion);
    }
    res.len() as i32 / 2
}

fn tile_at(map: &Vec<Vec<char>>, x:i32, y:i32) -> char {
    if x > map.len() as i32|| y > map[0].len() as i32{
        panic!("Coords out of bounds ({}, {})", x, y);
    }
    map[y as usize][x as usize]
}



pub fn part2(_input: &str) -> u32 {
    0
}

fn where_is_s(map: &Vec<Vec<char>>) -> (i32, i32) {
    let mut res : (i32, i32) = (0, 0);
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if tile_at(map, i as i32, j as i32) == 'S' {
                res = (i as i32, j as i32);
                break;
            }
        }
    }
    res
}

fn next_move(map: &Vec<Vec<char>>, current_postion: (i32, i32), res: &Vec<(i32, i32)>) -> (i32, i32) {
    let current_tile = tile_at(map, current_postion.0, current_postion.1);
    let destinations = get_connections(current_tile);
    let almost_valid_destinations: Vec<char> = destinations.into_iter()
        .filter(|destination| is_valid_move(map, *destination, current_postion)).collect();
    let valid_destinations: Vec<char> = almost_valid_destinations.into_iter()
        .filter(|d| {
            let direction = get_direction(*d);
            let next_position = (current_postion.0 as i32 + direction.0, current_postion.1 as i32 + direction.1);        
            let aux = (next_position.0, next_position.1);
            !res.contains(&aux)
        })
        .collect();

        let direction = get_direction(valid_destinations[0]);
        (current_postion.0 + direction.0, current_postion.1 + direction.1)
}

fn get_connections(tile: char) -> Vec<char> {
    let destinations = match tile {
        '|' => vec!['n', 's'],
        '-' => vec!['e', 'w'],
        'L' => vec!['e', 'n'],
        'J' => vec!['w', 'n'],
        '7' => vec!['w', 's'],
        'F' => vec!['e', 's'],
        'S' => vec!['n', 'w', 'e', 's'],
        _ => vec![],
    };
    destinations
}

fn is_valid_move(map: &Vec<Vec<char>>, destination: char, current_position: (i32, i32)) -> bool {
    let dir = get_direction(destination);
    if is_invalid_position(current_position, dir, map) {
        false
    } else {
        let connections_from_destination = get_connections(tile_at(map, (current_position.0 as i32 + dir.0),  (current_position.1 as i32 + dir.1)));
        let valid_connections: Vec<char> =  connections_from_destination.into_iter()
            .filter(|connection| reverse_connection(*connection ) == destination).collect();
        valid_connections.len() > 0
    }
}

fn is_invalid_position(current_position: (i32, i32), dir: (i32, i32), map: &Vec<Vec<char>>) -> bool {
    current_position.0 as i32 + dir.0 < 0 || 
        current_position.1 as i32 + dir.1 < 0 || 
        current_position.1 as i32 + dir.1 >= map.len() as i32 || 
        current_position.0 as i32 + dir.0 >= map[0].len() as i32
}

fn get_direction(direction: char) -> (i32, i32) {
    match direction {
        'n' => (0, -1),
        's' => (0, 1),
        'w' => (-1, 0),
        'e' => (1, 0),
        _ => panic!("Invalid direction {}", direction),
    }
}

// fn check_valid_connection(connection: char, connections_from_destination: Vec<char>) -> bool {
//     connections_from_destination.contains(&reverse_connection(connection))
// }

fn reverse_connection(connection: char) -> char {
    match connection {
        'w' => 'e',
        'e' => 'w',
        'n' => 's',
        's' => 'n',
        _ => panic!("Could not reverse connection {}", connection),

    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const MAP: &str = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";

    fn get_map() -> Vec<Vec<char>> {
        vec![
            vec!['T', 'E', 'A', 'L'],
            vec!['Z', 'Ñ', 'Ç', '0'],
            vec!['2', 'S', 'Q', 'W'],
            vec!['C', 'm', 'M', 'n'],
        ]
    } 

    fn get_map2() -> Vec<Vec<char>> {
        vec![
            vec!['-', 'L', '|', 'F', '7'],
            vec!['7', 'S', '-', '7', '|'],
            vec!['L', '|', '7', '|', '|'],
            vec!['-','L','-','J','|'],
            vec!['L','|','-','J','F'],
        ]
    }


    #[test]
    fn scn_tile_at_01() {
        let map = get_map();
        let mut tile = tile_at(&map, 0, 0);
        assert_eq!(tile, 'T');
        tile = tile_at(&map, 1, 0);
        assert_eq!(tile, 'E');
        tile = tile_at(&map, 2, 2);
        assert_eq!(tile, 'Q');
    }

    #[test]
    #[should_panic]
    fn scn_tile_at_02() {
        let map = vec![vec!['T', 'E'], vec!['Z', 'Ñ']];
        tile_at(&map, 5, 5);
    }

    #[test]
    fn scn_tile_at_03() {
        let map = get_map();
        let s = tile_at(&map, 1, 2);
        assert_eq!(s, 'S');
    }

    #[test]
    fn scn_tile_at_04() {
        let map = get_map();
        let s = where_is_s(&map);
        assert_eq!(s, (1, 2));
    }


    #[test]
    fn scn_part1_00() {
        let s = part1(MAP);
        assert_eq!(s, 4);
    }
}

