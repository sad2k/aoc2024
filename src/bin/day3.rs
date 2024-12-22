use std::fs;
use regex::Regex;

fn part1(input: &String) -> i64 {
    let regex = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let matches: Vec<_> = regex.find_iter(input).map(|m| m.as_str()).collect();
    matches.iter().map(|m| {
        let s = m.replace("mul(", "").replace(")", "");
        s.split(",").map(|x| x.parse::<i64>().unwrap()).product::<i64>()
    }).sum()
}

fn main() {
    let contents = fs::read_to_string("inputs/day3.txt").unwrap();

    // part 1
    println!("{}", part1(&contents));
}