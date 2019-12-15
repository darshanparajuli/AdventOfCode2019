use std::collections::VecDeque;

pub struct IntCodeComputer {
    program: Vec<i64>,
    pc: usize,
    relative_base: i64,
    input: Input,
}

pub struct Input {
    buffer: VecDeque<i64>,
}

impl Input {
    fn new() -> Self {
        Self {
            buffer: VecDeque::new(),
        }
    }

    pub fn read(&mut self) -> Option<i64> {
        self.buffer.pop_front()
    }

    pub fn write(&mut self, v: i64) {
        self.buffer.push_back(v);
    }
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

pub enum Ret<'a> {
    In(&'a mut Input),
    Out(i64),
    Halt,
}

impl IntCodeComputer {
    pub fn new(mut program: Vec<i64>) -> Self {
        program.resize(1024 * 1024, 0);
        Self {
            program,
            pc: 0,
            relative_base: 0,
            input: Input::new(),
        }
    }

    pub fn set_initial_input(&mut self, initial_input: &[i64]) {
        for i in initial_input {
            self.input.write(*i);
        }
    }

    pub fn run(&mut self) -> Ret {
        loop {
            let instr = self.program[self.pc];
            let opcode = (instr % 10) + (((instr / 10) % 10) * 10);

            match opcode {
                1 => {
                    // Add
                    let mut mode = Mode::from(instr as u64);
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
                    let mut mode = Mode::from(instr as u64);
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
                    match self.input.read() {
                        Some(input) => {
                            let mut mode = Mode::from(instr as u64);
                            let dest = match mode.next() {
                                ModeType::Pos => self.program[self.pc + 1] as usize,
                                ModeType::Rel => {
                                    (self.program[self.pc + 1] + self.relative_base) as usize
                                }
                                ModeType::Imm => unreachable!(),
                            };
                            self.program[dest as usize] = input;
                            self.pc += 2;
                        }
                        None => {
                            return Ret::In(&mut self.input);
                        }
                    }
                }
                4 => {
                    // Write to output
                    let mut mode = Mode::from(instr as u64);
                    let value = match mode.next() {
                        ModeType::Pos => self.program[self.program[self.pc + 1] as usize],
                        ModeType::Imm => self.program[self.pc + 1],
                        ModeType::Rel => {
                            self.program[(self.program[self.pc + 1] + self.relative_base) as usize]
                        }
                    };

                    self.pc += 2;
                    return Ret::Out(value);
                }
                5 => {
                    // jump-if-true
                    let mut mode = Mode::from(instr as u64);
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
                    let mut mode = Mode::from(instr as u64);
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
                    let mut mode = Mode::from(instr as u64);
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
                    let mut mode = Mode::from(instr as u64);
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
                    let mut mode = Mode::from(instr as u64);
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
                    break;
                }
                _ => unreachable!(),
            }
        }

        Ret::Halt
    }

    pub fn set_value_at_address(&mut self, addr: usize, value: i64) {
        self.program[addr] = value;
    }
}
