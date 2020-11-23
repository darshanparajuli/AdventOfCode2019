use aoc_2019::intcode_computer::*;
use aoc_2019::read_input;

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

    let mut align_params = vec![];

    for y in 1..lines.len() - 1 {
        let line = &lines[y];
        for x in 1..line.len() - 1 {
            if line[x] == '#' {
                let left = line[x - 1];
                let right = line[x + 1];
                let top = lines[y - 1][x];
                let bottom = lines[y + 1][x];

                if left == '#' && right == '#' && top == '#' && bottom == '#' {
                    align_params.push(x * y);
                }
            }
        }
    }

    println!("part 1: {}", align_params.iter().sum::<usize>());

    // lines
    //     .iter()
    //     .for_each(|l| println!("{}", l.iter().collect::<String>()));
}

fn part2(input: &[i64]) {}
