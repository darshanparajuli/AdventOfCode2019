#![allow(dead_code)]
use aoc_2019::intcode_computer::*;
use aoc_2019::read_input;
use std::collections::VecDeque;

const ROBOT: char = '^';
const SCAFFOLD: char = '#';
const OPEN_SPACE: char = '.';

fn main() {
    let input = read_input()
        .iter()
        .map(|s| s.split(',').collect::<Vec<_>>())
        .flatten()
        .map(|c| c.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    part1(&input);
    part2(&input);
}

fn part1(input: &[i64]) {
    let lines = get_output_lines(input);

    let mut align_params = vec![];

    for y in 1..lines.len() - 1 {
        let line = &lines[y];
        for x in 1..line.len() - 1 {
            if line[x] == SCAFFOLD {
                let left = line[x - 1];
                let right = line[x + 1];
                let top = lines[y - 1][x];
                let bottom = lines[y + 1][x];

                if left == SCAFFOLD && right == SCAFFOLD && top == SCAFFOLD && bottom == SCAFFOLD {
                    align_params.push(x * y);
                }
            }
        }
    }

    println!("part 1: {}", align_params.iter().sum::<usize>());
}

fn part2(input: &[i64]) {
    println!("part 2:");
    let output = get_output_lines(input);
    let mut robot_input = get_robot_input(&output);

    let mut computer = IntCodeComputer::new(input.to_vec());
    computer.set_value_at_address(0, 2);

    loop {
        match computer.run() {
            Ret::In(input) => {
                robot_input.pop_front().unwrap().iter().for_each(|e| {
                    // println!("{}", *e as i64);
                    input.write(*e as i64)
                });
                input.write(10);
            }
            Ret::Out(o) => {
                // if o == 10 {
                //     println!();
                // } else {
                //     println!("{}", o);
                // }
                if o >= 0 && o <= u8::MAX as i64 {
                    print!("{} ", char::from(o as u8));
                } else {
                    print!("{}", o);
                }
            }
            Ret::Halt => break,
        }
    }
}

fn get_robot_input(map: &[Vec<char>]) -> VecDeque<Vec<char>> {
    let mut commands = vec![];

    let mut robot = get_robot(map).unwrap();

    loop {
        let mut cmd = String::new();
        robot.turn_left();
        if is_pos_valid(map, robot.next_pos()) {
            cmd.push_str("L");
        } else {
            robot.turn_right();

            robot.turn_right();

            if is_pos_valid(map, robot.next_pos()) {
                cmd.push_str("R");
            } else {
                // Done.
                break;
            }
        }

        let mut count = 0;
        while is_pos_valid(map, robot.next_pos()) {
            robot.move_forward();
            count += 1;
        }

        cmd.push_str(&count.to_string());
        commands.push(cmd);
    }

    // dbg!(commands);
    // print_map(map, None);

    // Done manually :(
    let mut result = VecDeque::new();
    // Main
    result.push_back("A,B,A,C,A,C,B,C,C,B".chars().collect());
    // A
    result.push_back("L,4,L,4,L,10,R,4".chars().collect());
    // B
    result.push_back("R,4,L,4,L,4,R,8,R,10".chars().collect());
    // C
    result.push_back("R,4,L,10,R,10".chars().collect());
    // Video feed
    result.push_back("N".chars().collect());
    result
}

fn get_robot(map: &[Vec<char>]) -> Option<Robot> {
    for y in 0..map.len() {
        let row = &map[y];
        for x in 0..row.len() {
            if row[x] == ROBOT {
                return Some(Robot {
                    pos: Position {
                        x: x as i32,
                        y: y as i32,
                    },
                    dir: match row[x] {
                        '>' => Dir::E,
                        '<' => Dir::W,
                        '^' => Dir::N,
                        'V' => Dir::S,
                        _ => panic!("Invalid robot: {}", row[x]),
                    },
                });
            }
        }
    }

    None
}

fn is_pos_valid(map: &[Vec<char>], pos: Position) -> bool {
    if pos.y >= 0 && (pos.y as usize) < map.len() {
        let row = &map[pos.y as usize];
        if pos.x >= 0 && (pos.x as usize) < row.len() {
            return row[pos.x as usize] == SCAFFOLD;
        }
    }

    false
}

fn print_map(map: &[Vec<char>], robot: Option<&Robot>) {
    for y in 0..map.len() {
        let row = &map[y];
        for x in 0..row.len() {
            if let Some(robot) = robot {
                if x == robot.pos.x as usize && y == robot.pos.y as usize {
                    print!("{} ", robot.get_face());
                } else {
                    print!("{} ", row[x]);
                }
            } else {
                print!("{} ", row[x]);
            }
        }
        println!();
    }
    println!();
}

#[derive(Debug)]
struct Robot {
    pos: Position,
    dir: Dir,
}

impl Robot {
    fn turn_left(&mut self) {
        self.dir = match self.dir {
            Dir::E => Dir::N,
            Dir::W => Dir::S,
            Dir::N => Dir::W,
            Dir::S => Dir::E,
        };
    }

    fn turn_right(&mut self) {
        self.dir = match self.dir {
            Dir::E => Dir::S,
            Dir::W => Dir::N,
            Dir::N => Dir::E,
            Dir::S => Dir::W,
        };
    }

    fn move_forward(&mut self) {
        match self.dir {
            Dir::E => self.pos.x += 1,
            Dir::W => self.pos.x -= 1,
            Dir::N => self.pos.y -= 1,
            Dir::S => self.pos.y += 1,
        };
    }

    fn next_pos(&self) -> Position {
        let mut pos = self.pos;
        match self.dir {
            Dir::E => pos.x += 1,
            Dir::W => pos.x -= 1,
            Dir::N => pos.y -= 1,
            Dir::S => pos.y += 1,
        };
        pos
    }

    fn get_face(&self) -> char {
        match self.dir {
            Dir::E => '>',
            Dir::W => '<',
            Dir::N => '^',
            Dir::S => 'V',
        }
    }
}

#[derive(Copy, Clone, Hash, Debug)]
struct Position {
    x: i32,
    y: i32,
}

impl Eq for Position {}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Debug)]
enum Dir {
    E,
    W,
    N,
    S,
}

fn get_output_lines(input: &[i64]) -> Vec<Vec<char>> {
    let mut computer = IntCodeComputer::new(input.to_vec());
    let mut buffer: Vec<i64> = vec![];
    let mut lines = vec![];

    loop {
        match computer.run() {
            Ret::In(_) => {}
            Ret::Out(o) => {
                // New line
                if o == 10 {
                    if !buffer.is_empty() {
                        let line = buffer
                            .iter()
                            .cloned()
                            .map(|d| char::from(d as u8))
                            .collect::<Vec<char>>();
                        lines.push(line);
                        buffer.clear()
                    }
                } else {
                    buffer.push(o);
                }
            }
            Ret::Halt => break,
        }
    }

    lines
}
