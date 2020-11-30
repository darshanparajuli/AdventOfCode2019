use aoc_2019::read_input;
use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap, VecDeque};

const WALL: char = '#';
const OPEN_SPACE: char = '.';
const START: char = '@';

fn main() {
    let mut input = read_input()
        .iter()
        .map(|s| s.chars().collect())
        .collect::<Vec<_>>();

    part1(&input);

    update_input_for_part2(&mut input);
    part2(&input);
}

fn update_input_for_part2(input: &mut [Vec<char>]) {
    let start_pos = find_start_pos(&input);
    let start_pos = start_pos.first().unwrap();
    let patch: Vec<char> = "@#@###@#@".chars().collect();
    let mut i = 0;
    for y in start_pos.y - 1..=start_pos.y + 1 {
        for x in start_pos.x - 1..=start_pos.x + 1 {
            input[y as usize][x as usize] = patch[i];
            i += 1;
        }
    }
}

/// Used hints. This one took some time :/
fn part1(input: &[Vec<char>]) {
    let start_positions = find_start_pos(input);
    let collected_keys = BTreeSet::new();
    let positions = get_key_door_positions(input);
    let mut cache = HashMap::new();
    let steps = min_steps(
        start_positions.into_iter().collect(),
        input,
        collected_keys,
        &positions,
        &mut cache,
    );
    println!("part 1: {}", steps);
}

fn part2(input: &[Vec<char>]) {
    let start_positions = find_start_pos(input);
    let collected_keys = BTreeSet::new();
    let positions = get_key_door_positions(input);
    let mut cache = HashMap::new();
    let steps = min_steps(
        start_positions.into_iter().collect(),
        input,
        collected_keys,
        &positions,
        &mut cache,
    );
    println!("part 2: {}", steps);
}

#[derive(Hash, Eq, PartialEq)]
struct CacheState(BTreeSet<Pos>, BTreeSet<char>);

fn min_steps(
    start_pos: BTreeSet<Pos>,
    input: &[Vec<char>],
    mut collected_keys: BTreeSet<char>,
    positions: &HashMap<char, Pos>,
    cache: &mut HashMap<CacheState, u32>,
) -> u32 {
    let state = CacheState(start_pos.clone(), collected_keys.clone());
    if cache.contains_key(&state) {
        return cache[&state];
    }

    let steps = get_reachable_keys_for_multiple_start_positions(&start_pos, input, &collected_keys)
        .iter()
        .map(|(key, (n, src))| {
            collected_keys.insert(*key);
            let mut new_positions = BTreeSet::new();
            new_positions.insert(positions[key]);
            new_positions.extend(start_pos.iter());
            new_positions.remove(src);
            let steps = min_steps(
                new_positions,
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

fn find_start_pos(input: &[Vec<char>]) -> Vec<Pos> {
    let mut v = vec![];
    for y in 0..input.len() {
        let row = &input[y];
        for x in 0..row.len() {
            let c = row[x];
            if c == START {
                v.push(Pos::new(x as i32, y as i32));
            }
        }
    }
    v
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

fn get_reachable_keys_for_multiple_start_positions(
    pos: &BTreeSet<Pos>,
    input: &[Vec<char>],
    collected_keys: &BTreeSet<char>,
) -> HashMap<char, (u32, Pos)> {
    pos.iter()
        .map(|p| get_reachable_keys(*p, input, collected_keys))
        .map(|m| m.into_iter())
        .flatten()
        .collect()
}

fn get_reachable_keys(
    start_pos: Pos,
    input: &[Vec<char>],
    collected_keys: &BTreeSet<char>,
) -> HashMap<char, (u32, Pos)> {
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
                                steps_map.insert(c, (dist_map[n], start_pos));
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

#[derive(Debug, Copy, Clone, Hash, PartialOrd)]
struct Pos {
    x: i32,
    y: i32,
}

impl Ord for Pos {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.x == other.x {
            self.y.cmp(&other.y)
        } else {
            self.x.cmp(&other.x)
        }
    }
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
