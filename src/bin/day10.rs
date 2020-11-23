use aoc_2019::read_input;
use std::collections::HashSet;

fn main() {
    let input = read_input()
        .iter()
        .map(|s| {
            let mut row = vec![];
            s.chars().for_each(|c| match c {
                '.' => row.push(0),
                '#' => row.push(1),
                _ => unreachable!(),
            });
            row
        })
        .collect::<Vec<_>>();

    let (x, y) = part1(&input);
    part2(&input, x, y);
}

fn asteroid_hit(map: &[Vec<u32>], sx: i32, sy: i32, dx: i32, dy: i32) -> Option<(i32, i32)> {
    let mut x = sx + dx;
    let mut y = sy + dy;
    while y >= 0 && x >= 0 && y < map.len() as i32 && x < map[y as usize].len() as i32 {
        if map[y as usize][x as usize] == 1 {
            return Some((x, y));
        }

        x += dx;
        y += dy;
    }

    None
}

fn gcd(a: i32, b: i32) -> i32 {
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

fn part1(map: &[Vec<u32>]) -> (usize, usize) {
    let asteroids = {
        let mut result = vec![];
        for y in 0..map.len() {
            let row = &map[y];
            for x in 0..row.len() {
                if row[x] == 1 {
                    result.push((x, y));
                }
            }
        }
        result
    };

    let mut hit_count = vec![];

    for (sx, sy) in &asteroids {
        let mut count = 0;

        for (x, y) in asteroids.iter().filter(|(x, y)| x != sx || y != sy) {
            let mut dx = *x as i32 - *sx as i32;
            let mut dy = *y as i32 - *sy as i32;

            let gcd = gcd(dx.abs(), dy.abs());
            dx /= gcd;
            dy /= gcd;

            match asteroid_hit(map, *sx as i32, *sy as i32, dx, dy) {
                Some((hx, hy)) => {
                    if hx == *x as i32 && hy == *y as i32 {
                        count += 1;
                    }
                }
                None => unreachable!(),
            }
        }

        hit_count.push(((sx, sy), count));
    }

    let ((x, y), max_count) = hit_count.iter().max_by(|(_, a), (_, b)| a.cmp(&b)).unwrap();
    println!("part 1: {}", max_count);

    (**x, **y)
}

fn part2(map: &[Vec<u32>], sx: usize, sy: usize) {
    let mut coords = vec![];

    let asteroids = {
        let mut result = vec![];
        for y in 0..map.len() {
            let row = &map[y];
            for x in 0..row.len() {
                if row[x] == 1 {
                    result.push((x, y));
                }
            }
        }
        result
    };

    let mut tright = asteroids
        .iter()
        .filter(|(x, y)| x >= &sx && y < &sy)
        .collect::<HashSet<_>>();

    let mut bright = asteroids
        .iter()
        .filter(|(x, y)| x >= &sx && y >= &sy)
        .collect::<HashSet<_>>();

    let mut bleft = asteroids
        .iter()
        .filter(|(x, y)| x < &sx && y >= &sy)
        .collect::<HashSet<_>>();

    let mut tleft = asteroids
        .iter()
        .filter(|(x, y)| x < &sx && y < &sy)
        .collect::<HashSet<_>>();

    let mut temp = vec![];

    while coords.len() < 200 {
        // tright
        {
            for (x, y) in tright.iter().filter(|(x, y)| *x != sx || *y != sy) {
                let mut dx = *x as i32 - sx as i32;
                let mut dy = *y as i32 - sy as i32;

                let gcd = gcd(dx.abs(), dy.abs());
                dx /= gcd;
                dy /= gcd;

                match asteroid_hit(map, sx as i32, sy as i32, dx, dy) {
                    Some((hx, hy)) => {
                        if hx == *x as i32 && hy == *y as i32 {
                            let angle = (dy as f32).atan2(dx as f32);
                            temp.push(((*x, *y), angle));
                        }
                    }
                    None => unreachable!(),
                }
            }

            temp.sort_by(|(_, a1), (_, a2)| a1.partial_cmp(a2).unwrap());
            temp.iter().for_each(|((x, y), _)| coords.push((*x, *y)));

            for ((x, y), _) in &temp {
                tright.remove(&(*x, *y));
            }

            temp.clear();
        }

        // bright
        {
            for (x, y) in bright.iter().filter(|(x, y)| *x != sx || *y != sy) {
                let mut dx = *x as i32 - sx as i32;
                let mut dy = *y as i32 - sy as i32;

                let gcd = gcd(dx.abs(), dy.abs());
                dx /= gcd;
                dy /= gcd;

                match asteroid_hit(map, sx as i32, sy as i32, dx, dy) {
                    Some((hx, hy)) => {
                        if hx == *x as i32 && hy == *y as i32 {
                            let angle = (dy as f32).atan2(dx as f32);
                            temp.push(((*x, *y), angle));
                        }
                    }
                    None => unreachable!(),
                }
            }

            temp.sort_by(|(_, a1), (_, a2)| a1.partial_cmp(a2).unwrap());
            temp.iter().for_each(|((x, y), _)| coords.push((*x, *y)));

            for ((x, y), _) in &temp {
                bright.remove(&(*x, *y));
            }

            temp.clear();
        }

        // bleft
        {
            for (x, y) in bleft.iter().filter(|(x, y)| *x != sx || *y != sy) {
                let mut dx = *x as i32 - sx as i32;
                let mut dy = *y as i32 - sy as i32;

                let gcd = gcd(dx.abs(), dy.abs());
                dx /= gcd;
                dy /= gcd;

                match asteroid_hit(map, sx as i32, sy as i32, dx, dy) {
                    Some((hx, hy)) => {
                        if hx == *x as i32 && hy == *y as i32 {
                            let angle = (dy as f32).atan2(dx as f32);
                            temp.push(((*x, *y), angle));
                        }
                    }
                    None => unreachable!(),
                }
            }

            temp.sort_by(|(_, a1), (_, a2)| a1.partial_cmp(a2).unwrap());
            temp.iter().for_each(|((x, y), _)| coords.push((*x, *y)));

            for ((x, y), _) in &temp {
                bleft.remove(&(*x, *y));
            }

            temp.clear();
        }

        // tleft
        {
            for (x, y) in tleft.iter().filter(|(x, y)| *x != sx || *y != sy) {
                let mut dx = *x as i32 - sx as i32;
                let mut dy = *y as i32 - sy as i32;

                let gcd = gcd(dx.abs(), dy.abs());
                dx /= gcd;
                dy /= gcd;

                match asteroid_hit(map, sx as i32, sy as i32, dx, dy) {
                    Some((hx, hy)) => {
                        if hx == *x as i32 && hy == *y as i32 {
                            let angle = (dy as f32).atan2(dx as f32);
                            temp.push(((*x, *y), angle));
                        }
                    }
                    None => unreachable!(),
                }
            }

            temp.sort_by(|(_, a1), (_, a2)| a1.partial_cmp(a2).unwrap());
            temp.iter().for_each(|((x, y), _)| coords.push((*x, *y)));

            for ((x, y), _) in &temp {
                tleft.remove(&(*x, *y));
            }

            temp.clear();
        }
    }

    // println!("{:?}", coords);

    let (x, y) = coords[199];
    println!("part 2: {}", x * 100 + y);
}
