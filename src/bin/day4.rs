use std::collections::{HashMap, HashSet};
use std::fs;

struct State {
    next_expected_char: char,
    found: i32,
}

fn parse(input: &String) -> Vec<Vec<char>> {
    let lines = input.lines();
    lines.map(|x| x.chars().collect::<Vec<_>>()).collect::<Vec<_>>()
}

fn find_in_line(input: &Vec<char>) -> i32 {
    let mut count = 0;
    let next_chars = HashMap::from([
        ('X', 'M'),
        ('M', 'A'),
        ('A', 'S')
    ]);
    let mut next: Option<char> = None;
    for i in 0..input.len() {
        let ch = input[i];
        let mut found_char = false;
        if next.is_some() {
            if next.unwrap() == ch {
                found_char = true;
            }
        }
        if (found_char) {
            if ch == 'S' {
                next = None;
                count += 1;
            } else {
                next = Some(next_chars[&ch]);
            }
        } else {
            if ch == 'X' {
                next = Some(next_chars[&ch])
            } else {
                next = None
            }
        }
    }
    count
}

fn find_in_line_and_rev(input: &Vec<char>) -> i32 {
    let mut res = find_in_line(input);
    let reversed_line = input.iter().map(|x| *x).rev().collect::<Vec<_>>();
    res += find_in_line(&reversed_line);
    res
}

fn part1(input: &Vec<Vec<char>>) -> i32 {
    let mut res = 0;
    // horizontal lines
    for i in 0..input.len() {
        let line = &input[i];
        res += find_in_line_and_rev(line);
    }
    // vertical lines
    for i in 0..input[0].len() {
        let mut vertical = Vec::new();
        for j in 0..input.len() {
            vertical.push(input[j][i]);
        }
        res += find_in_line_and_rev(&vertical);
    }
    // diagonal lines left to right
    // starting from the top
    for i in 0..input[0].len() {
        let mut col = i;
        let mut diag = Vec::new();
        let mut row = 0;
        while col < input[0].len() && row < input.len() {
            diag.push(input[row][col]);
            row += 1;
            col += 1;
        }
        res += find_in_line_and_rev(&diag);
    }
    // starting from the left
    for i in 1..input.len() {
        let mut row = i;
        let mut col = 0;
        let mut diag = Vec::new();
        while col < input[0].len() && row < input.len() {
            diag.push(input[row][col]);
            row += 1;
            col += 1;
        }
        res += find_in_line_and_rev(&diag);
    }
    // diagonal lines right to left
    // starting from the top
    for i in 0..input[0].len() {
        let mut col = i;
        let mut diag = Vec::new();
        let mut row = 0;
        while col >= 0 && row < input.len() {
            diag.push(input[row][col]);
            row += 1;
            if col == 0 {
                break;
            } else {
                col -= 1;
            }
        }
        res += find_in_line_and_rev(&diag);
    }
    // starting from the right
    for i in 1..input.len() {
        let mut row = i;
        let mut col = input[0].len() - 1;
        let mut diag = Vec::new();
        while col >= 0 && row < input.len() {
            diag.push(input[row][col]);
            row += 1;
            if (col == 0) {
                break;
            } else {
                col -= 1;
            }
        }
        res += find_in_line_and_rev(&diag);
    }
    res
}

fn part2(input: &Vec<Vec<char>>) -> i32 {
    let mut res = 0;
    let mut set = HashSet::new();
    let mut set_ms = HashSet::from(['M', 'S']);
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            if input.len() - i > 2 && input[0].len() - j > 2 {
                let top_left = input[i][j];
                let top_right = input[i][j + 2];
                let middle = input[i + 1][j + 1];
                let bottom_left = input[i + 2][j];
                let bottom_right = input[i + 2][j + 2];
                if (middle == 'A') {
                    set.clear();
                    set.insert(top_left);
                    set.insert(bottom_right);
                    if set == set_ms {
                        set.clear();
                        set.insert(top_right);
                        set.insert(bottom_left);
                        if set == set_ms {
                            res += 1;
                        }
                    }
                }
            }
        }
    }
    res
}

fn main() {
    let contents = fs::read_to_string("inputs/day4.txt").unwrap();
    let parsed = parse(&contents);

    // part 1
    // println!("{}", part1(&parsed));

    // part 2
    println!("{}", part2(&parsed));
}