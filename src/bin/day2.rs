use std::fs;
use std::str::Lines;

fn parse(lines: Lines<'_>) -> Vec<Vec<i32>> {
    lines.map(|line| line.split(" ").map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>()).collect::<Vec<_>>()
}

fn is_safe(report: &Vec<i32>) -> bool {
    let pairs = report.windows(2).collect::<Vec<_>>();
    let deltas = pairs.iter().map(|x| (x[1] - x[0]).abs()).collect::<Vec<_>>();
    let bad_deltas = deltas.iter().filter(|x| **x < 1 || **x > 3);
    if bad_deltas.count() > 0 {
        return false;
    }
    let signs = pairs.iter().map(|x| (x[1] - x[0]).signum()).collect::<Vec<_>>();
    if signs.iter().sum::<i32>().abs() != signs.len() as i32 {
        return false;
    }
    true
}

fn is_safe2(report: &Vec<i32>) -> bool {
    if is_safe(report) {
        return true;
    }

    let mut report2 = report.clone();
    for i in 0..report2.len() {
        let del = report2.remove(i);
        if is_safe(&report2) {
            return true;
        }
        report2.insert(i, del);
    }
    false
}

fn part1(input: &Vec<Vec<i32>>) -> u32 {
    input.iter().filter(|x| is_safe(*x)).count() as u32
}

fn part2(input: &Vec<Vec<i32>>) -> u32 {
    input.iter().filter(|x| is_safe2(*x)).count() as u32
}

fn main() {
    let contents = fs::read_to_string("inputs/day2.txt").unwrap();
    let parsed = parse(contents.lines());

    // part 1
    // println!("{}", part1(&parsed));

    // part 2
    println!("{}", part2(&parsed));
}