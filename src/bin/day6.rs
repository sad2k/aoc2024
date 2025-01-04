use std::collections::HashSet;
use std::fs;

fn parse(input: &String) -> (Vec<Vec<bool>>, (usize, usize)) {
    let lines = input.lines();
    let mut res = Vec::new();
    let mut start = (0, 0);
    for (row, line) in lines.enumerate() {
        let r: Vec<_> = line.chars().enumerate().map(|(col, ch)| match ch {
            '#' => true,
            '^' => {
                start = (row, col);
                false
            }
            _ => false
        }).collect();
        res.push(r);
    }
    (res, start)
}

fn part1(map: &Vec<Vec<bool>>, start: (usize, usize)) -> usize {
    let mut dir = (-1, 0);
    let mut coord = (start.0 as i32, start.1 as i32);
    let mut visited = HashSet::new();
    loop {
        visited.insert(coord);
        let new_coord = (coord.0 + dir.0, coord.1 + dir.1);
        if new_coord.0 < 0 || new_coord.1 < 0 || new_coord.0 >= map.len() as i32 || new_coord.1 >= map[0].len() as i32 {
            break;
        }
        if map[new_coord.0 as usize][new_coord.1 as usize] {
            // obstacle, need to change the direction
            dir = match dir {
                (-1, 0) => (0, 1),
                (0, 1) => (1, 0),
                (1, 0) => (0, -1),
                (0, -1) => (-1, 0),
                _ => panic!("unsupported direction: {dir:?}")
            }
        } else {
            coord = new_coord
        }
    }
    visited.len()
}

fn part2(map: &Vec<Vec<bool>>, start: (usize, usize)) -> usize {
    // brute forcing...
    let mut res = 0;
    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if !map[row][col] && start != (row, col) {
                let mut map_copy = map.clone();
                map_copy[row][col] = true;
                if is_loop(&map_copy, start) {
                    res += 1;
                }
            }
        }
    }
    res
}

fn is_loop(map: &Vec<Vec<bool>>, start: (usize, usize)) -> bool {
    let mut dir = (-1, 0);
    let mut coord = (start.0 as i32, start.1 as i32);
    let mut visited = HashSet::new();
    loop {
        if visited.contains(&(coord, dir)) {
            return true;
        }
        visited.insert((coord, dir));
        let new_coord = (coord.0 + dir.0, coord.1 + dir.1);
        if new_coord.0 < 0 || new_coord.1 < 0 || new_coord.0 >= map.len() as i32 || new_coord.1 >= map[0].len() as i32 {
            break;
        }
        if map[new_coord.0 as usize][new_coord.1 as usize] {
            // obstacle, need to change the direction
            dir = match dir {
                (-1, 0) => (0, 1),
                (0, 1) => (1, 0),
                (1, 0) => (0, -1),
                (0, -1) => (-1, 0),
                _ => panic!("unsupported direction: {dir:?}")
            }
        } else {
            coord = new_coord
        }
    }
    false
}

fn main() {
    let contents = fs::read_to_string("inputs/day6.txt").unwrap();
    let (map, start) = parse(&contents);

    // part 1
    // println!("{}", part1(&map, start));

    // part 2
    println!("{}", part2(&map, start));
}