use aoc_2019::intcode_computer::*;
use aoc_2019::read_input;
use std::collections::HashMap;
use std::convert::From;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_input()?
        .iter()
        .map(|a| a.split(",").collect::<Vec<_>>())
        .flatten()
        .map(|a| a.parse::<i64>())
        .collect::<Result<Vec<_>, _>>()?;

    part1(&input);
    part2(&input);

    Ok(())
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Color {
    Black,
    White,
}

impl From<i64> for Color {
    fn from(v: i64) -> Self {
        match v {
            0 => Color::Black,
            1 => Color::White,
            _ => unreachable!(),
        }
    }
}

impl From<Color> for i64 {
    fn from(color: Color) -> Self {
        match color {
            Color::Black => 0,
            Color::White => 1,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum TurnDir {
    Left90,
    Right90,
}

impl From<i64> for TurnDir {
    fn from(v: i64) -> Self {
        match v {
            0 => TurnDir::Left90,
            1 => TurnDir::Right90,
            _ => unreachable!(),
        }
    }
}

impl From<TurnDir> for i64 {
    fn from(turn_dir: TurnDir) -> Self {
        match turn_dir {
            TurnDir::Left90 => 0,
            TurnDir::Right90 => 1,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn for_turn(&self, turn_dir: TurnDir) -> Dir {
        match turn_dir {
            TurnDir::Left90 => match *self {
                Dir::Up => Dir::Left,
                Dir::Down => Dir::Right,
                Dir::Left => Dir::Down,
                Dir::Right => Dir::Up,
            },
            TurnDir::Right90 => match *self {
                Dir::Up => Dir::Right,
                Dir::Down => Dir::Left,
                Dir::Left => Dir::Up,
                Dir::Right => Dir::Down,
            },
        }
    }
}

fn part1(program: &[i64]) {
    let mut buffer: Vec<i64> = vec![];

    let mut rx = 0i64;
    let mut ry = 0i64;
    let mut dir = Dir::Up;

    let mut map: HashMap<(i64, i64), Color> = HashMap::new();

    let mut computer = IntCodeComputer::new(program.to_vec());
    computer.set_initial_input(&[Color::Black.into()]);

    loop {
        match computer.run() {
            Ret::In(input) => match map.get(&(rx, ry)).cloned() {
                Some(c) => {
                    let c = c.into();
                    input.write(c);
                }
                None => {
                    input.write(Color::Black.into());
                }
            },
            Ret::Out(o) => {
                buffer.push(o);

                if buffer.len() == 2 {
                    let color_out = buffer[0].into();
                    let turn_dir = buffer[1].into();
                    dir = dir.for_turn(turn_dir);

                    map.insert((rx, ry), color_out);

                    match dir {
                        Dir::Up => {
                            ry += 1;
                        }
                        Dir::Down => {
                            ry -= 1;
                        }
                        Dir::Left => {
                            rx -= 1;
                        }
                        Dir::Right => {
                            rx += 1;
                        }
                    }

                    buffer.clear();
                }
            }
            Ret::Halt => break,
        }
    }

    println!("part 1: {}", map.len());
}

fn part2(program: &[i64]) {
    let mut buffer: Vec<i64> = vec![];

    let mut rx = 0i64;
    let mut ry = 0i64;
    let mut dir = Dir::Up;

    let mut map: HashMap<(i64, i64), Color> = HashMap::new();
    map.insert((0, 0), Color::White);

    let mut computer = IntCodeComputer::new(program.to_vec());

    let mut points = vec![];
    points.push((0, 0));

    loop {
        match computer.run() {
            Ret::In(input) => match map.get(&(rx, ry)).cloned() {
                Some(c) => {
                    let c = c.into();
                    input.write(c);
                }
                None => {
                    input.write(Color::Black.into());
                }
            },
            Ret::Out(o) => {
                buffer.push(o);

                if buffer.len() == 2 {
                    let color_out = buffer[0].into();
                    let turn_dir = buffer[1].into();
                    dir = dir.for_turn(turn_dir);

                    map.insert((rx, ry), color_out);

                    if color_out == Color::White {
                        points.push((rx, ry));
                    }

                    match dir {
                        Dir::Up => {
                            ry += 1;
                        }
                        Dir::Down => {
                            ry -= 1;
                        }
                        Dir::Left => {
                            rx -= 1;
                        }
                        Dir::Right => {
                            rx += 1;
                        }
                    }

                    buffer.clear();
                }
            }
            Ret::Halt => break,
        }
    }

    let mut array = [[0; 60]; 20];

    for (x, y) in &points {
        let x = x + 10;
        let y = y + 10;
        array[y as usize][x as usize] = 1;
    }

    for y in (0..array.len()).rev() {
        for x in 0..array[y].len() {
            if array[y][x] == 1 {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
