use std::collections::HashMap;
use std::fs;
use std::iter::zip;
use std::str::Lines;
use regex::Regex;

fn part1(fst: &Vec<i32>, snd: &Vec<i32>) -> i32 {
    let mut mfst = fst.clone();
    let mut msnd = snd.clone();
    mfst.sort();
    msnd.sort();
    let zipped = zip(mfst, msnd);
    zipped.map(|(x, y)| (x - y).abs()).sum::<i32>()
}

fn part2(fst: &Vec<i32>, snd: &Vec<i32>) -> i32 {
    let freq = snd.iter().fold(HashMap::new(),
                               |mut map, val| {
                                   map.entry(val).and_modify(|f| *f += 1).or_insert(1);
                                   map
                               });
    fst.iter().map(|x| *x * *freq.get(x).unwrap_or(&0)).sum()
}

fn parse(lines: Lines<'_>) -> (Vec<i32>, Vec<i32>) {
    let mut fst = Vec::new();
    let mut snd = Vec::new();
    let sep = Regex::new(r"\s+").unwrap();
    for line in lines {
        let spl = sep.split(line).map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
        fst.push(spl[0]);
        snd.push(spl[1]);
    }
    (fst, snd)
}

fn main() {
    let contents = fs::read_to_string("inputs/day1.txt").unwrap();
    let parsed = parse(contents.lines());

    // part 1
    // println!("{}", part1(&parsed.0, &parsed.1));

    // part 2
    println!("{}", part2(&parsed.0, &parsed.1));
}