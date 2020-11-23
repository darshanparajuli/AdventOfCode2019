use aoc_2019::read_input;
use std::collections::VecDeque;

fn main() {
    let input = read_input()
        .iter()
        .map(|s| s.split(",").map(|a| a.parse::<i64>().unwrap()))
        .flatten()
        .collect::<Vec<_>>();

    part1(&input);
    part2(&input);
}

fn part1(input: &[i64]) {
    let mut c = IntCodeComputer::new(input.to_vec());
    c.push_input(1);

    loop {
        match c.run() {
            Some(a) => println!("{}", a),
            None => break,
        }
    }
}

fn part2(input: &[i64]) {
    let mut c = IntCodeComputer::new(input.to_vec());
    c.push_input(2);

    loop {
        match c.run() {
            Some(a) => println!("{}", a),
            None => break,
        }
    }
}

struct IntCodeComputer {
    program: Vec<i64>,
    pc: usize,
    relative_base: i64,
    halted: bool,
    last_output: Option<i64>,
    input: VecDeque<i64>,
}

#[derive(Debug)]
enum ModeType {
    Pos,
    Imm,
    Rel,
}

struct Mode {
    v: u64,
}

impl Mode {
    fn from(value: u64) -> Self {
        Self { v: value / 100 }
    }

    fn next(&mut self) -> ModeType {
        let value = self.v % 10;
        self.v /= 10;

        match value {
            0 => ModeType::Pos,
            1 => ModeType::Imm,
            2 => ModeType::Rel,
            _ => unreachable!(),
        }
    }
}

impl IntCodeComputer {
    fn new(mut program: Vec<i64>) -> Self {
        program.resize(1024 * 1024, 0);
        Self {
            program,
            pc: 0,
            relative_base: 0,
            halted: false,
            last_output: None,
            input: VecDeque::new(),
        }
    }

    fn run(&mut self) -> Option<i64> {
        loop {
            let v = self.program[self.pc];
            let opcode = (v % 10) + (((v / 10) % 10) * 10);

            match opcode {
                1 => {
                    // Add
                    let mut mode = Mode::from(v as u64);
                    let a = match mode.next() {
                        ModeType::Pos => self.program[self.program[self.pc + 1] as usize],
                        ModeType::Imm => self.program[self.pc + 1],
                        ModeType::Rel => {
                            self.program[(self.program[self.pc + 1] + self.relative_base) as usize]
                        }
                    };

                    let b = match mode.next() {
                        ModeType::Pos => self.program[self.program[self.pc + 2] as usize],
                        ModeType::Imm => self.program[self.pc + 2],
                        ModeType::Rel => {
                            self.program[(self.program[self.pc + 2] + self.relative_base) as usize]
                        }
                    };

                    let dest = match mode.next() {
                        ModeType::Pos => self.program[self.pc + 3] as usize,
                        ModeType::Rel => (self.program[self.pc + 3] + self.relative_base) as usize,
                        ModeType::Imm => unreachable!(),
                    };
                    self.program[dest as usize] = a + b;

                    self.pc += 4;
                }
                2 => {
                    // Mul
                    let mut mode = Mode::from(v as u64);
                    let a = match mode.next() {
                        ModeType::Pos => self.program[self.program[self.pc + 1] as usize],
                        ModeType::Imm => self.program[self.pc + 1],
                        ModeType::Rel => {
                            self.program[(self.program[self.pc + 1] + self.relative_base) as usize]
                        }
                    };

                    let b = match mode.next() {
                        ModeType::Pos => self.program[self.program[self.pc + 2] as usize],
                        ModeType::Imm => self.program[self.pc + 2],
                        ModeType::Rel => {
                            self.program[(self.program[self.pc + 2] + self.relative_base) as usize]
                        }
                    };

                    let dest = match mode.next() {
                        ModeType::Pos => self.program[self.pc + 3] as usize,
                        ModeType::Rel => (self.program[self.pc + 3] + self.relative_base) as usize,
                        ModeType::Imm => unreachable!(),
                    };
                    self.program[dest as usize] = a * b;

                    self.pc += 4;
                }
                3 => {
                    // Read from input
                    let mut mode = Mode::from(v as u64);
                    let dest = match mode.next() {
                        ModeType::Pos => self.program[self.pc + 1] as usize,
                        ModeType::Rel => (self.program[self.pc + 1] + self.relative_base) as usize,
                        ModeType::Imm => unreachable!(),
                    };
                    self.program[dest as usize] = self.input.pop_front().unwrap();
                    self.pc += 2;
                }
                4 => {
                    // Write to output
                    let mut mode = Mode::from(v as u64);
                    let value = match mode.next() {
                        ModeType::Pos => self.program[self.program[self.pc + 1] as usize],
                        ModeType::Imm => self.program[self.pc + 1],
                        ModeType::Rel => {
                            self.program[(self.program[self.pc + 1] + self.relative_base) as usize]
                        }
                    };

                    self.last_output = Some(value);
                    self.pc += 2;
                    return Some(value);
                }
                5 => {
                    // jump-if-true
                    let mut mode = Mode::from(v as u64);
                    let a = match mode.next() {
                        ModeType::Pos => self.program[self.program[self.pc + 1] as usize],
                        ModeType::Imm => self.program[self.pc + 1],
                        ModeType::Rel => {
                            self.program[(self.program[self.pc + 1] + self.relative_base) as usize]
                        }
                    };

                    if a != 0 {
                        let b = match mode.next() {
                            ModeType::Pos => self.program[self.program[self.pc + 2] as usize],
                            ModeType::Imm => self.program[self.pc + 2],
                            ModeType::Rel => {
                                self.program
                                    [(self.program[self.pc + 2] + self.relative_base) as usize]
                            }
                        };

                        self.pc = b as usize;
                    } else {
                        self.pc += 3;
                    }
                }
                6 => {
                    // jump-if-false
                    let mut mode = Mode::from(v as u64);
                    let a = match mode.next() {
                        ModeType::Pos => self.program[self.program[self.pc + 1] as usize],
                        ModeType::Imm => self.program[self.pc + 1],
                        ModeType::Rel => {
                            self.program[(self.program[self.pc + 1] + self.relative_base) as usize]
                        }
                    };

                    if a == 0 {
                        let b = match mode.next() {
                            ModeType::Pos => self.program[self.program[self.pc + 2] as usize],
                            ModeType::Imm => self.program[self.pc + 2],
                            ModeType::Rel => {
                                self.program
                                    [(self.program[self.pc + 2] + self.relative_base) as usize]
                            }
                        };

                        self.pc = b as usize;
                    } else {
                        self.pc += 3;
                    }
                }
                7 => {
                    // less than
                    let mut mode = Mode::from(v as u64);
                    let a = match mode.next() {
                        ModeType::Pos => self.program[self.program[self.pc + 1] as usize],
                        ModeType::Imm => self.program[self.pc + 1],
                        ModeType::Rel => {
                            self.program[(self.program[self.pc + 1] + self.relative_base) as usize]
                        }
                    };

                    let b = match mode.next() {
                        ModeType::Pos => self.program[self.program[self.pc + 2] as usize],
                        ModeType::Imm => self.program[self.pc + 2],
                        ModeType::Rel => {
                            self.program[(self.program[self.pc + 2] + self.relative_base) as usize]
                        }
                    };

                    let dest = match mode.next() {
                        ModeType::Pos => self.program[self.pc + 3] as usize,
                        ModeType::Rel => (self.program[self.pc + 3] + self.relative_base) as usize,
                        ModeType::Imm => unreachable!(),
                    };
                    if a < b {
                        self.program[dest as usize] = 1;
                    } else {
                        self.program[dest as usize] = 0;
                    }

                    self.pc += 4;
                }
                8 => {
                    // equal to
                    let mut mode = Mode::from(v as u64);
                    let a = match mode.next() {
                        ModeType::Pos => self.program[self.program[self.pc + 1] as usize],
                        ModeType::Imm => self.program[self.pc + 1],
                        ModeType::Rel => {
                            self.program[(self.program[self.pc + 1] + self.relative_base) as usize]
                        }
                    };

                    let b = match mode.next() {
                        ModeType::Pos => self.program[self.program[self.pc + 2] as usize],
                        ModeType::Imm => self.program[self.pc + 2],
                        ModeType::Rel => {
                            self.program[(self.program[self.pc + 2] + self.relative_base) as usize]
                        }
                    };

                    let dest = match mode.next() {
                        ModeType::Pos => self.program[self.pc + 3] as usize,
                        ModeType::Rel => (self.program[self.pc + 3] + self.relative_base) as usize,
                        ModeType::Imm => unreachable!(),
                    };
                    if a == b {
                        self.program[dest as usize] = 1;
                    } else {
                        self.program[dest as usize] = 0;
                    }

                    self.pc += 4;
                }
                9 => {
                    let mut mode = Mode::from(v as u64);
                    let a = match mode.next() {
                        ModeType::Pos => self.program[self.program[self.pc + 1] as usize],
                        ModeType::Imm => self.program[self.pc + 1],
                        ModeType::Rel => {
                            self.program[(self.program[self.pc + 1] + self.relative_base) as usize]
                        }
                    };

                    self.relative_base += a;
                    self.pc += 2;
                }
                99 => {
                    // halt
                    self.halted = true;
                    break;
                }
                _ => unreachable!(),
            }
        }

        None
    }

    fn push_input(&mut self, input: i64) {
        self.input.push_back(input);
    }
}
