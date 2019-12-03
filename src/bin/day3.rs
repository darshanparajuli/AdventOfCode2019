use aoc_2019::parse_input;
use std::cmp;
use std::error::Error;

#[derive(Debug)]
enum Path {
    R(u32),
    U(u32),
    L(u32),
    D(u32),
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = parse_input(|line| {
        line.split(",")
            .map(|e| match &e[0..1] {
                "R" => Path::R(e[1..].parse::<u32>().unwrap()),
                "U" => Path::U(e[1..].parse::<u32>().unwrap()),
                "L" => Path::L(e[1..].parse::<u32>().unwrap()),
                "D" => Path::D(e[1..].parse::<u32>().unwrap()),
                _ => unreachable!(),
            })
            .collect::<Vec<_>>()
    })?;
    assert!(input.len() == 2);

    part1(&input[0], &input[1]);
    part2(&input[0], &input[1]);

    Ok(())
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Line {
    p1: Point,
    p2: Point,
}

impl Line {
    fn intersects(&self, line: &Line) -> Option<Point> {
        if self.p1.x == self.p2.x && line.p1.y == line.p2.y {
            let x1 = cmp::min(line.p1.x, line.p2.x);
            let x2 = cmp::max(line.p1.x, line.p2.x);

            if x1 <= self.p1.x && x2 >= self.p1.x {
                let y1 = cmp::min(self.p1.y, self.p2.y);
                let y2 = cmp::max(self.p1.y, self.p2.y);
                if y1 <= line.p1.y && y2 >= line.p1.y {
                    return Some(Point {
                        x: self.p1.x,
                        y: line.p1.y,
                    });
                }
            }
        }

        if self.p1.y == self.p2.y && line.p1.x == line.p2.x {
            let y1 = cmp::min(line.p1.y, line.p2.y);
            let y2 = cmp::max(line.p1.y, line.p2.y);

            if y1 <= self.p1.y && y2 >= self.p1.y {
                let x1 = cmp::min(self.p1.x, self.p2.x);
                let x2 = cmp::max(self.p1.x, self.p2.x);
                if x1 <= line.p1.x && x2 >= line.p1.x {
                    return Some(Point {
                        x: line.p1.x,
                        y: self.p1.y,
                    });
                }
            }
        }

        None
    }

    fn len(&self) -> i32 {
        if self.p1.x == self.p2.x {
            (self.p1.y - self.p2.y).abs()
        } else {
            (self.p1.x - self.p2.x).abs()
        }
    }
}

fn lines_from_paths(paths: &[Path]) -> Vec<Line> {
    let mut points = vec![];
    let mut x = 0i32;
    let mut y = 0i32;

    for p in paths {
        match p {
            Path::R(v) => x += *v as i32,
            Path::U(v) => y += *v as i32,
            Path::L(v) => x -= *v as i32,
            Path::D(v) => y -= *v as i32,
        }
        points.push(Point { x, y });
    }

    points
        .windows(2)
        .map(|chunk| {
            let p1 = chunk[0];
            let p2 = chunk[1];
            Line { p1, p2 }
        })
        .collect::<Vec<_>>()
}

fn part1(paths1: &[Path], paths2: &[Path]) {
    let lines1 = lines_from_paths(paths1);
    let lines2 = lines_from_paths(paths2);

    let mut intersections = vec![];

    for l1 in &lines1 {
        for l2 in &lines2 {
            if let Some(intersection) = l1.intersects(l2) {
                intersections.push(intersection);
            }
        }
    }

    let min = intersections
        .iter()
        .map(|p| p.x.abs() + p.y.abs())
        .min()
        .unwrap();
    println!("part 1: {:?}", min);
}

fn part2(paths1: &[Path], paths2: &[Path]) {
    let lines1 = lines_from_paths(paths1);
    let lines2 = lines_from_paths(paths2);

    let mut dist = vec![];

    for l1 in &lines1 {
        for l2 in &lines2 {
            if let Some(p) = l1.intersects(l2) {
                let mut s1 = 0;
                for (i, l) in lines1.iter().enumerate() {
                    if l == l1 {
                        if l.p1.x == l.p2.x {
                            s1 += (p.y - l.p1.y).abs();
                        } else {
                            s1 += (p.x - l.p1.x).abs();
                        }
                        break;
                    } else {
                        if i == 0 {
                            s1 += (l.p1.x + l.p1.y).abs();
                        }
                        s1 += l.len();
                    }
                }

                let mut s2 = 0;
                for (i, l) in lines2.iter().enumerate() {
                    if l == l2 {
                        if l.p1.x == l.p2.x {
                            s2 += (p.y - l.p1.y).abs();
                        } else {
                            s2 += (p.x - l.p1.x).abs();
                        }
                        break;
                    } else {
                        if i == 0 {
                            s2 += (l.p1.x + l.p1.y).abs();
                        }
                        s2 += l.len();
                    }
                }

                dist.push((s1, s2));
            }
        }
    }

    let min = dist.iter().map(|(a, b)| a + b).min().unwrap();
    println!("part 2: {}", min);
}
