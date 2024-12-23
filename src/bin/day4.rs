use std::collections::HashMap;
use std::fs;

struct State {
    next_expected_char: char,
    found: i32
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

fn part1(input: &Vec<Vec<char>>) -> i32 {
    let mut res = 0;
    // horizontal lines
    for i in 0..input.len() {
        let line = &input[i];
        res += find_in_line(line);
    }
    res
}

fn main() {
    let contents = fs::read_to_string("inputs/day4.txt").unwrap();
    let parsed = parse(&contents);

    // part 1
    println!("{}", part1(&parsed));
}