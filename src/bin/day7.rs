use aoc_2019::read_input;
use std::collections::VecDeque;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_input()?
        .iter()
        .map(|line| line.split(",").map(|a| a.parse::<i32>()))
        .flatten()
        .collect::<Result<Vec<_>, _>>()?;

    part1(input.clone());
    part2(input);

    Ok(())
}

fn is_valid(s: &[i32], tmp: [i32; 5]) -> bool {
    for i in &tmp {
        if !s.contains(i) {
            return false;
        }
    }

    true
}

fn part1(program: Vec<i32>) {
    let mut settings = vec![];
    for a in 0..=4 {
        for b in 0..=4 {
            for c in 0..=4 {
                for d in 0..=4 {
                    for e in 0..=4 {
                        let s = [a, b, c, d, e];
                        if is_valid(&s, [0, 1, 2, 3, 4]) {
                            settings.push(s);
                        }
                    }
                }
            }
        }
    }

    let mut max = 0;
    for s in settings {
        let mut c = IntCodeComputer::new(program.clone());
        c.push_input(s[0]);
        c.push_input(0);
        let o = c.run().unwrap();

        let mut c = IntCodeComputer::new(program.clone());
        c.push_input(s[1]);
        c.push_input(o);
        let o = c.run().unwrap();

        let mut c = IntCodeComputer::new(program.clone());
        c.push_input(s[2]);
        c.push_input(o);
        let o = c.run().unwrap();

        let mut c = IntCodeComputer::new(program.clone());
        c.push_input(s[3]);
        c.push_input(o);
        let o = c.run().unwrap();

        let mut c = IntCodeComputer::new(program.clone());
        c.push_input(s[4]);
        c.push_input(o);
        let o = c.run().unwrap();

        if o > max {
            max = o;
        }
    }

    println!("part 1: {}", max);
}

fn part2(program: Vec<i32>) {
    let mut settings = vec![];
    for a in 5..=9 {
        for b in 5..=9 {
            for c in 5..=9 {
                for d in 5..=9 {
                    for e in 5..=9 {
                        let s = [a, b, c, d, e];
                        if is_valid(&s, [5, 6, 7, 8, 9]) {
                            settings.push(s);
                        }
                    }
                }
            }
        }
    }

    let mut max = 0;
    for s in settings {
        let mut c1 = IntCodeComputer::new(program.clone());
        c1.push_input(s[0]);
        c1.push_input(0);

        let mut c2 = IntCodeComputer::new(program.clone());
        c2.push_input(s[1]);

        let mut c3 = IntCodeComputer::new(program.clone());
        c3.push_input(s[2]);

        let mut c4 = IntCodeComputer::new(program.clone());
        c4.push_input(s[3]);

        let mut c5 = IntCodeComputer::new(program.clone());
        c5.push_input(s[4]);

        while !c5.halted {
            let o = match c1.run() {
                Some(o) => o,
                None => break,
            };

            c2.push_input(o);
            let o = match c2.run() {
                Some(o) => o,
                None => break,
            };

            c3.push_input(o);
            let o = match c3.run() {
                Some(o) => o,
                None => break,
            };

            c4.push_input(o);
            let o = match c4.run() {
                Some(o) => o,
                None => break,
            };

            c5.push_input(o);
            let o = match c5.run() {
                Some(o) => o,
                None => break,
            };

            if o > max {
                max = o;
            }

            if c5.halted {
                break;
            }

            c1.push_input(o);
        }
    }

    println!("part 2: {}", max);
}

struct IntCodeComputer {
    program: Vec<i32>,
    pc: usize,
    halted: bool,
    last_output: Option<i32>,
    input: VecDeque<i32>,
}

impl IntCodeComputer {
    fn new(program: Vec<i32>) -> Self {
        Self {
            program,
            pc: 0,
            halted: false,
            last_output: None,
            input: VecDeque::new(),
        }
    }

    fn run(&mut self) -> Option<i32> {
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

        loop {
            let v = self.program[self.pc];
            let opcode = (v % 10) + (((v / 10) % 10) * 10);

            match opcode {
                1 => {
                    // Add
                    let mut mode = Mode::from(v as u32);
                    let mut a = self.program[self.pc + 1];
                    if mode.is_next_pos() {
                        a = self.program[a as usize];
                    }

                    let mut b = self.program[self.pc + 2];
                    if mode.is_next_pos() {
                        b = self.program[b as usize];
                    }

                    let dest = self.program[self.pc + 3];
                    self.program[dest as usize] = a + b;

                    self.pc += 4;
                }
                2 => {
                    // Mul
                    let mut mode = Mode::from(v as u32);
                    let mut a = self.program[self.pc + 1];
                    if mode.is_next_pos() {
                        a = self.program[a as usize];
                    }

                    let mut b = self.program[self.pc + 2];
                    if mode.is_next_pos() {
                        b = self.program[b as usize];
                    }

                    let dest = self.program[self.pc + 3];
                    self.program[dest as usize] = a * b;

                    self.pc += 4;
                }
                3 => {
                    // Read from input
                    let dest = self.program[self.pc + 1];
                    self.program[dest as usize] = self.input.pop_front().unwrap();
                    self.pc += 2;
                }
                4 => {
                    // Write to output
                    let src = self.program[self.pc + 1];
                    let value = self.program[src as usize];
                    self.last_output = Some(value);
                    self.pc += 2;
                    return Some(value);
                }
                5 => {
                    // jump-if-true
                    let mut mode = Mode::from(v as u32);
                    let mut a = self.program[self.pc + 1];
                    if mode.is_next_pos() {
                        a = self.program[a as usize];
                    }

                    if a != 0 {
                        let mut b = self.program[self.pc + 2];
                        if mode.is_next_pos() {
                            b = self.program[b as usize];
                        }

                        self.pc = b as usize;
                    } else {
                        self.pc += 3;
                    }
                }
                6 => {
                    // jump-if-false
                    let mut mode = Mode::from(v as u32);
                    let mut a = self.program[self.pc + 1];
                    if mode.is_next_pos() {
                        a = self.program[a as usize];
                    }

                    if a == 0 {
                        let mut b = self.program[self.pc + 2];
                        if mode.is_next_pos() {
                            b = self.program[b as usize];
                        }

                        self.pc = b as usize;
                    } else {
                        self.pc += 3;
                    }
                }
                7 => {
                    // less than
                    let mut mode = Mode::from(v as u32);
                    let mut a = self.program[self.pc + 1];
                    if mode.is_next_pos() {
                        a = self.program[a as usize];
                    }

                    let mut b = self.program[self.pc + 2];
                    if mode.is_next_pos() {
                        b = self.program[b as usize];
                    }

                    let dest = self.program[self.pc + 3];
                    if a < b {
                        self.program[dest as usize] = 1;
                    } else {
                        self.program[dest as usize] = 0;
                    }

                    self.pc += 4;
                }
                8 => {
                    // equal to
                    let mut mode = Mode::from(v as u32);
                    let mut a = self.program[self.pc + 1];
                    if mode.is_next_pos() {
                        a = self.program[a as usize];
                    }

                    let mut b = self.program[self.pc + 2];
                    if mode.is_next_pos() {
                        b = self.program[b as usize];
                    }

                    let dest = self.program[self.pc + 3];
                    if a == b {
                        self.program[dest as usize] = 1;
                    } else {
                        self.program[dest as usize] = 0;
                    }

                    self.pc += 4;
                }
                99 => {
                    // halt
                    self.halted = true;
                    break;
                }
                _ => unreachable!(),
            }
        }

        self.last_output
    }

    fn push_input(&mut self, input: i32) {
        self.input.push_back(input);
    }
}
