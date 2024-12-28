use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

fn build_graph(rules: &Vec<Vec<i32>>, update: &Vec<i32>) -> HashMap<i32, usize> {
    let mut parents_by_child: HashMap<i32, HashSet<i32>> = HashMap::new();
    let mut children_by_parent: HashMap<i32, HashSet<i32>> = HashMap::new();
    let update_set: HashSet<i32> = HashSet::from_iter(update.clone());
    for rule in rules {
        let parent: i32 = rule[0];
        let child: i32 = rule[1];
        if !update_set.contains(&parent) || !update_set.contains(&child) {
            continue;
        }
        parents_by_child.entry(parent).or_insert(HashSet::new());
        parents_by_child.entry(child).or_insert(HashSet::new()).insert(parent);
        children_by_parent.entry(parent).or_insert(HashSet::new()).insert(child);
        children_by_parent.entry(child).or_insert(HashSet::new());
    }
    let mut topologically_sorted = Vec::new();
    let mut q = VecDeque::from_iter(parents_by_child.iter().filter(|(p, cs)| cs.is_empty()).map(|(p, cs)| *p));
    let mut counter = 0;
    while !q.is_empty() {
        let id = q.pop_front().unwrap();
        topologically_sorted.push((id, counter));
        counter += 1;
        let children = children_by_parent.get(&id).unwrap();
        for c in children {
            let ps = parents_by_child.get_mut(&c).unwrap();
            ps.remove(&id);
            if ps.is_empty() {
                q.push_back(*c);
            }
        }
    }
    let map: HashMap<i32, usize> = topologically_sorted.into_iter().collect();
    map
}

fn part1(rules: &Vec<Vec<i32>>, updates: &Vec<Vec<i32>>) -> i32 {
    let mut res = 0;
    'outer: for update in updates {
        let graph = build_graph(rules, update);
        let mut prev_ord = None;
        for page in update {
            let ord = *graph.get(&page).expect(&format!("Not in graph: {}", page));
            if ord < prev_ord.unwrap_or(0) {
                continue 'outer;
            }
            prev_ord = Some(ord);
        }
        res += update[update.len() / 2];
    }
    res
}

fn main() {
    let contents = fs::read_to_string("inputs/day5.txt").unwrap();
    let rules: Vec<_> = contents.lines().take_while(|x| !x.is_empty())
        .map(|x| x.split("|").map(|xx| xx.parse::<i32>().unwrap()).collect::<Vec<_>>())
        .collect();
    let updates: Vec<_> = contents.lines().skip(rules.len() + 1)
        .map(|x| x.split(",").map(|xx| xx.parse::<i32>().unwrap()).collect::<Vec<_>>())
        .collect();

    // part 1
    println!("{}", part1(&rules, &updates));
}