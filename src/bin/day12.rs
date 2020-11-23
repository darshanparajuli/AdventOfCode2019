use aoc_2019::read_input;
use std::collections::HashSet;

fn main() {
    let input = read_input()
        .iter()
        .map(|s| {
            let index = s.find(',').unwrap();
            let x = s[s.find('=').unwrap() + 1..index].parse::<i32>().unwrap();

            let s = &s[index + 1..];
            let index = s.find(',').unwrap();
            let y = s[s.find('=').unwrap() + 1..index].parse::<i32>().unwrap();

            let s = &s[index + 1..];
            let z = s[s.find('=').unwrap() + 1..s.find('>').unwrap()]
                .parse::<i32>()
                .unwrap();

            Vec3::new(x, y, z)
        })
        .collect::<Vec<_>>();

    part1(&input);
    part2(&input);
}

fn part1(input: &[Vec3]) {
    let mut moons = input.iter().map(|p| Moon::new(*p)).collect::<Vec<Moon>>();
    let mut set = HashSet::new();

    for _ in 0..1000 {
        for i in 0..moons.len() {
            for j in 0..moons.len() {
                if i == j {
                    continue;
                }

                if set.contains(&(i, j)) {
                    continue;
                }

                let mut a = moons[i];
                let mut b = moons[j];

                let dx = (a.pos.x - b.pos.x).signum();
                let dy = (a.pos.y - b.pos.y).signum();
                let dz = (a.pos.z - b.pos.z).signum();

                a.vel.x -= dx;
                a.vel.y -= dy;
                a.vel.z -= dz;

                b.vel.x += dx;
                b.vel.y += dy;
                b.vel.z += dz;

                moons[i] = a;
                moons[j] = b;

                set.insert((i, j));
                set.insert((j, i));
            }
        }

        set.clear();

        for m in &mut moons {
            m.pos.x += m.vel.x;
            m.pos.y += m.vel.y;
            m.pos.z += m.vel.z;
        }
    }

    let total_energy: i32 = moons
        .iter()
        .map(|m| {
            (m.pos.x.abs() + m.pos.y.abs() + m.pos.z.abs())
                * (m.vel.x.abs() + m.vel.y.abs() + m.vel.z.abs())
        })
        .sum();

    println!("part 1: {}", total_energy);
}

fn part2(input: &[Vec3]) {
    let mut moons = input.iter().map(|p| Moon::new(*p)).collect::<Vec<Moon>>();
    let initial_moons = moons.clone();

    let mut set = HashSet::new();

    let mut pairs = vec![];
    for i in 0..moons.len() {
        for j in 0..moons.len() {
            if i == j {
                continue;
            }

            if set.contains(&(i, j)) {
                continue;
            }

            pairs.push((i, j));

            set.insert((i, j));
            set.insert((j, i));
        }
    }

    let mut xsteps = 0u64;

    loop {
        xsteps += 1;

        for (i, j) in &pairs {
            let mut a = moons[*i];
            let mut b = moons[*j];

            let dx = (a.pos.x - b.pos.x).signum();
            a.vel.x -= dx;
            b.vel.x += dx;

            moons[*i] = a;
            moons[*j] = b;
        }

        for m in &mut moons {
            m.pos.x += m.vel.x;
        }

        if moons.iter().zip(initial_moons.iter()).all(|(m, i)| m == i) {
            break;
        }
    }

    let mut ysteps = 0u64;
    let mut moons = initial_moons.clone();
    loop {
        ysteps += 1;

        for (i, j) in &pairs {
            let mut a = moons[*i];
            let mut b = moons[*j];

            let dy = (a.pos.y - b.pos.y).signum();
            a.vel.y -= dy;
            b.vel.y += dy;

            moons[*i] = a;
            moons[*j] = b;
        }

        for m in &mut moons {
            m.pos.y += m.vel.y;
        }

        if moons.iter().zip(initial_moons.iter()).all(|(m, i)| m == i) {
            break;
        }
    }

    let mut zsteps = 0u64;
    let mut moons = initial_moons.clone();
    loop {
        zsteps += 1;

        for (i, j) in &pairs {
            let mut a = moons[*i];
            let mut b = moons[*j];

            let dz = (a.pos.z - b.pos.z).signum();
            a.vel.z -= dz;
            b.vel.z += dz;

            moons[*i] = a;
            moons[*j] = b;
        }

        for m in &mut moons {
            m.pos.z += m.vel.z;
        }

        if moons.iter().zip(initial_moons.iter()).all(|(m, i)| m == i) {
            break;
        }
    }

    let steps = lcm(lcm(xsteps, ysteps), zsteps);
    println!("part 2: {}", steps);
}

fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}

fn gcd(a: u64, b: u64) -> u64 {
    if a == 0 {
        return b;
    }

    if b == 0 {
        return a;
    }

    if a > b {
        return gcd(a - b, b);
    } else if b > a {
        return gcd(a, b - a);
    } else {
        return a;
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Vec3 {
    x: i32,
    y: i32,
    z: i32,
}

impl Vec3 {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Moon {
    pos: Vec3,
    vel: Vec3,
}

impl Moon {
    fn new(pos: Vec3) -> Self {
        Self {
            pos,
            vel: Vec3::new(0, 0, 0),
        }
    }
}
