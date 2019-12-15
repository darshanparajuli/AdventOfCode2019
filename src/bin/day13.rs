use aoc_2019::intcode_computer::*;
use aoc_2019::read_input;
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

fn part1(input: &[i64]) {
    let mut computer = IntCodeComputer::new(input.to_vec());
    let mut buffer: Vec<i64> = vec![];
    let mut count = 0;
    loop {
        match computer.run() {
            Ret::In(_) => {}
            Ret::Out(o) => {
                buffer.push(o);

                if buffer.len() == 3 {
                    let tile: Tile = buffer[2].into();
                    if tile == Tile::Block {
                        count += 1;
                    }

                    buffer.clear();
                }
            }
            Ret::Halt => break,
        }
    }

    println!("part 1: {}", count);
}

fn part2(input: &[i64]) {
    let mut computer = IntCodeComputer::new(input.to_vec());
    computer.set_value_at_address(0, 2);

    let mut buffer: Vec<i64> = vec![];

    let mut board = [[Tile::Empty; 100]; 100];
    let mut bx = 0;
    let mut px = 0;
    let mut score = 0;

    loop {
        match computer.run() {
            Ret::In(input) => {
                if bx < px {
                    input.write(-1);
                } else if bx > px {
                    input.write(1);
                } else {
                    input.write(0);
                }
            }
            Ret::Out(o) => {
                buffer.push(o);

                if buffer.len() == 3 {
                    let x = buffer[0];
                    let y = buffer[1];
                    let value = buffer[2];

                    if x == -1 && y == 0 {
                        score = value;
                    } else {
                        let tile: Tile = value.into();
                        board[y as usize][x as usize] = tile;

                        match tile {
                            Tile::Ball => bx = x,
                            Tile::HorizontalPaddle => px = x,
                            _ => {}
                        }
                    }

                    buffer.clear();
                }
            }
            Ret::Halt => break,
        }
    }

    println!("part 2: {}", score);
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Tile {
    Empty,
    Wall,
    Block,
    HorizontalPaddle,
    Ball,
}

impl From<i64> for Tile {
    fn from(value: i64) -> Self {
        match value {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::HorizontalPaddle,
            4 => Tile::Ball,
            x => panic!("invalid tile id: {}", x),
        }
    }
}
