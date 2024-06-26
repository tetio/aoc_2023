pub fn part1(input: &str) -> u32 {
    let map: Vec<Vec<char>> = input
        .split("\n")
        .map(|line| line.chars().into_iter().collect::<Vec<char>>())
        .collect();
    0
}

fn tile_at(map: &Vec<Vec<char>>, x: usize, y: usize) -> char {
    if x > map.len() || y > map[0].len() {
        panic!("Coords out of bounds ({}, {})", x, y);
    }
    map[y][x]
}

pub fn part2(_input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scn_tile_at_01() {
        let map = vec![
            vec!['T', 'E', 'A', 'L'],
            vec!['Z', 'Ñ', 'Ç', '0'],
            vec!['2', '1', 'Q', 'W'],
            vec!['C', 'm', 'M', 'n'],
        ];
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
}
