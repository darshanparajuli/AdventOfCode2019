use aoc_2019::read_input;

fn main() {
    let program = read_input()
        .iter()
        .map(|line| line.split(",").map(|a| a.parse::<i32>().unwrap()))
        .flatten()
        .collect::<Vec<_>>();

    part1(program.clone());
    part2(program);
}

struct Mode {
    v: u32,
}

impl Mode {
    fn from(value: u32) -> Self {
        Self { v: value / 100 }
    }
    fn is_next_pos(&mut self) -> bool {
        let value = self.v % 10;
        self.v /= 10;
        value == 0
    }
}

fn part1(mut program: Vec<i32>) {
    let input = 1;

    let mut pc = 0;
    loop {
        let v = program[pc];
        let opcode = (v % 10) + (((v / 10) % 10) * 10);

        match opcode {
            1 => {
                // Add
                let mut mode = Mode::from(v as u32);
                let mut a = program[pc + 1];
                if mode.is_next_pos() {
                    a = program[a as usize];
                }

                let mut b = program[pc + 2];
                if mode.is_next_pos() {
                    b = program[b as usize];
                }

                let dest = program[pc + 3];
                program[dest as usize] = a + b;

                pc += 4;
            }
            2 => {
                // Mul
                let mut mode = Mode::from(v as u32);
                let mut a = program[pc + 1];
                if mode.is_next_pos() {
                    a = program[a as usize];
                }

                let mut b = program[pc + 2];
                if mode.is_next_pos() {
                    b = program[b as usize];
                }

                let dest = program[pc + 3];
                program[dest as usize] = a * b;

                pc += 4;
            }
            3 => {
                // Read from input
                let dest = program[pc + 1];
                program[dest as usize] = input;
                pc += 2;
            }
            4 => {
                // Write to output
                let src = program[pc + 1];
                let value = program[src as usize];
                println!("part 1: {}", value);
                pc += 2;
            }
            99 => {
                // halt
                break;
            }
            _ => unreachable!(),
        }
    }
}

fn part2(mut program: Vec<i32>) {
    let input = 5;

    let mut pc = 0;
    loop {
        let v = program[pc];
        let opcode = (v % 10) + (((v / 10) % 10) * 10);

        match opcode {
            1 => {
                // Add
                let mut mode = Mode::from(v as u32);
                let mut a = program[pc + 1];
                if mode.is_next_pos() {
                    a = program[a as usize];
                }

                let mut b = program[pc + 2];
                if mode.is_next_pos() {
                    b = program[b as usize];
                }

                let dest = program[pc + 3];
                program[dest as usize] = a + b;

                pc += 4;
            }
            2 => {
                // Mul
                let mut mode = Mode::from(v as u32);
                let mut a = program[pc + 1];
                if mode.is_next_pos() {
                    a = program[a as usize];
                }

                let mut b = program[pc + 2];
                if mode.is_next_pos() {
                    b = program[b as usize];
                }

                let dest = program[pc + 3];
                program[dest as usize] = a * b;

                pc += 4;
            }
            3 => {
                // Read from input
                let dest = program[pc + 1];
                program[dest as usize] = input;
                pc += 2;
            }
            4 => {
                // Write to output
                let src = program[pc + 1];
                let value = program[src as usize];
                println!("part 2: {}", value);
                pc += 2;
            }
            5 => {
                // jump-if-true
                let mut mode = Mode::from(v as u32);
                let mut a = program[pc + 1];
                if mode.is_next_pos() {
                    a = program[a as usize];
                }

                if a != 0 {
                    let mut b = program[pc + 2];
                    if mode.is_next_pos() {
                        b = program[b as usize];
                    }

                    pc = b as usize;
                } else {
                    pc += 3;
                }
            }
            6 => {
                // jump-if-false
                let mut mode = Mode::from(v as u32);
                let mut a = program[pc + 1];
                if mode.is_next_pos() {
                    a = program[a as usize];
                }

                if a == 0 {
                    let mut b = program[pc + 2];
                    if mode.is_next_pos() {
                        b = program[b as usize];
                    }

                    pc = b as usize;
                } else {
                    pc += 3;
                }
            }
            7 => {
                // less than
                let mut mode = Mode::from(v as u32);
                let mut a = program[pc + 1];
                if mode.is_next_pos() {
                    a = program[a as usize];
                }

                let mut b = program[pc + 2];
                if mode.is_next_pos() {
                    b = program[b as usize];
                }

                let dest = program[pc + 3];
                if a < b {
                    program[dest as usize] = 1;
                } else {
                    program[dest as usize] = 0;
                }

                pc += 4;
            }
            8 => {
                // equal to
                let mut mode = Mode::from(v as u32);
                let mut a = program[pc + 1];
                if mode.is_next_pos() {
                    a = program[a as usize];
                }

                let mut b = program[pc + 2];
                if mode.is_next_pos() {
                    b = program[b as usize];
                }

                let dest = program[pc + 3];
                if a == b {
                    program[dest as usize] = 1;
                } else {
                    program[dest as usize] = 0;
                }

                pc += 4;
            }
            99 => {
                // halt
                break;
            }
            _ => unreachable!(),
        }
    }
}
