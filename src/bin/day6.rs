use aoc_2019::read_input;
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let input = read_input()
        .iter()
        .map(|line| {
            let mut a = line.split(")");
            (a.next().unwrap().to_string(), a.next().unwrap().to_string())
        })
        .collect::<Vec<_>>();

    part1(&input);
    part2(&input);
}

fn part1(input: &[(String, String)]) {
    let mut map = HashMap::new();
    for (a, b) in input {
        let v = map.entry(a).or_insert_with(|| vec![]);
        v.push(b);
    }

    let mut total_count = 0;

    let mut queue = VecDeque::new();

    for k in map.keys() {
        for e in &map[k] {
            queue.push_back(e);
        }

        while !queue.is_empty() {
            let k = queue.pop_front().unwrap();
            if let Some(e) = map.get(k) {
                for i in e {
                    queue.push_back(i);
                }
            }
            total_count += 1;
        }

        queue.clear();
    }

    println!("part 1: {}", total_count);
}

fn part2(input: &[(String, String)]) {
    let (santa_orbit, _) = input.iter().find(|a| a.1 == "SAN").unwrap();
    let (you_orbit, _) = input.iter().find(|a| a.1 == "YOU").unwrap();

    let mut parent = HashMap::new();
    for (a, b) in input {
        parent.insert(b, a);
    }

    let mut map = HashMap::new();
    for (a, b) in input {
        let v = map.entry(a).or_insert_with(|| HashSet::new());
        v.insert(b);

        if let Some(b) = parent.get(a) {
            v.insert(b);
        }
    }

    let mut visited = HashSet::new();

    let mut queue = VecDeque::new();
    let mut temp_elements = Vec::new();

    queue.push_back(you_orbit);
    visited.insert(you_orbit);

    let mut count = 0;

    'outer: loop {
        while !queue.is_empty() {
            let k = queue.pop_front().unwrap();
            if k == santa_orbit {
                break 'outer;
            }

            if let Some(e) = map.get(k) {
                for i in e {
                    if !visited.contains(i) {
                        temp_elements.push(i);
                        visited.insert(i);
                    }
                }
            }
        }

        count += 1;
        for i in &temp_elements {
            queue.push_back(i);
        }
        temp_elements.clear();
    }

    println!("part 2: {}", count);
}
