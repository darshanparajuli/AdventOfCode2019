use aoc_2019::read_input;
use std::collections::{BTreeSet, HashMap, VecDeque};

const WALL: char = '#';
const OPEN_SPACE: char = '.';
const START: char = '@';

fn main() {
    let input = read_input()
        .iter()
        .map(|s| s.chars().collect())
        .collect::<Vec<_>>();

    part1(&input);
}

/// Used hints. This one took some time :/
fn part1(input: &[Vec<char>]) {
    let start_pos = find_start_pos(input).unwrap();
    let collected_keys = BTreeSet::new();
    let positions = get_key_door_positions(input);
    let mut cache = HashMap::new();
    let steps = min_steps(start_pos, input, collected_keys, &positions, &mut cache);
    println!("part 1: {}", steps);
}

#[derive(Hash, Eq, PartialEq)]
struct CacheState(Pos, BTreeSet<char>);

fn min_steps(
    start_pos: Pos,
    input: &[Vec<char>],
    mut collected_keys: BTreeSet<char>,
    positions: &HashMap<char, Pos>,
    cache: &mut HashMap<CacheState, u32>,
) -> u32 {
    let state = CacheState(start_pos, collected_keys.clone());
    if cache.contains_key(&state) {
        return cache[&state];
    }

    let steps = get_reachable_keys(start_pos, input, &collected_keys)
        .iter()
        .map(|(key, n)| {
            collected_keys.insert(*key);
            let steps = min_steps(
                positions[key],
                input,
                collected_keys.clone(),
                positions,
                cache,
            );
            collected_keys.remove(key);

            n + steps
        })
        .min()
        .unwrap_or(0);

    cache.insert(state, steps);
    steps
}

fn find_start_pos(input: &[Vec<char>]) -> Option<Pos> {
    for y in 0..input.len() {
        let row = &input[y];
        for x in 0..row.len() {
            let c = row[x];
            if c == START {
                return Some(Pos::new(x as i32, y as i32));
            }
        }
    }
    None
}

fn get_key_door_positions(map: &[Vec<char>]) -> HashMap<char, Pos> {
    let mut result = HashMap::new();

    map.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, c)| match c {
            c @ 'a'..='z' | c @ 'A'..='Z' => {
                result.insert(*c, Pos::new(x as i32, y as i32));
            }
            _ => {}
        })
    });

    result
}

fn get_reachable_keys(
    start_pos: Pos,
    input: &[Vec<char>],
    collected_keys: &BTreeSet<char>,
) -> HashMap<char, u32> {
    let mut steps_map = HashMap::new();
    let mut dist_map = HashMap::new();

    let mut queue = VecDeque::new();
    dist_map.insert(start_pos, 0);
    queue.push_back(start_pos);

    while let Some(pos) = queue.pop_front() {
        let neighbors = [
            Pos::new(pos.x - 1, pos.y),
            Pos::new(pos.x + 1, pos.y),
            Pos::new(pos.x, pos.y - 1),
            Pos::new(pos.x, pos.y + 1),
        ];

        for n in &neighbors {
            if dist_map.contains_key(n) {
                continue;
            }

            if n.y < 0 || n.y >= input.len() as i32 {
                continue;
            }
            let row = &input[n.y as usize];
            if n.x < 0 || n.x >= row.len() as i32 {
                continue;
            }

            match row[n.x as usize] {
                WALL => {}
                c @ _ => {
                    dist_map.insert(*n, dist_map[&pos] + 1);

                    match c {
                        'a'..='z' => {
                            if collected_keys.contains(&c) {
                                queue.push_back(*n);
                            } else {
                                steps_map.insert(c, dist_map[n]);
                            }
                        }
                        'A'..='Z' => {
                            if collected_keys.contains(&c.to_ascii_lowercase()) {
                                queue.push_back(*n);
                            }
                        }
                        OPEN_SPACE | START => {
                            queue.push_back(*n);
                        }
                        _ => unreachable!(),
                    }
                }
            }
        }
    }

    steps_map
}

#[allow(dead_code)]
fn get_neighbor_pos(pos: Pos, map: &[Vec<char>]) -> Vec<Pos> {
    let mut v = vec![];

    let neighbors = [
        Pos::new(pos.x - 1, pos.y),
        Pos::new(pos.x + 1, pos.y),
        Pos::new(pos.x, pos.y - 1),
        Pos::new(pos.x, pos.y + 1),
    ];

    for n in &neighbors {
        if n.y < 0 || n.y >= map.len() as i32 {
            continue;
        }
        let row = &map[n.y as usize];
        if n.x < 0 || n.x >= row.len() as i32 {
            continue;
        }

        if row[n.x as usize] != WALL {
            v.push(*n)
        }
    }

    v
}

#[derive(Debug, Copy, Clone, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

impl Eq for Pos {}

impl PartialEq for Pos {
    fn eq(&self, other: &Pos) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Pos {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}
