use aoc_2019::read_input;

fn main() {
    let input = read_input()
        .iter()
        .flat_map(|line| {
            line.split(",")
                .map(|a| a.parse::<i32>())
                .collect::<Result<Vec<_>, _>>()
        })
        .flatten()
        .collect::<Vec<_>>();

    part1(input.clone());
    part2(input);
}

fn part1(mut input: Vec<i32>) {
    let mut pos = 0;

    input[1] = 12;
    input[2] = 2;

    while input[pos] != 99 {
        let opcode = input[pos];
        let a = input[input[pos + 1] as usize];
        let b = input[input[pos + 2] as usize];
        let dest = input[pos + 3] as usize;

        let result = match opcode {
            1 => (a + b),
            2 => a * b,
            _ => unreachable!(),
        };

        input[dest] = result;

        pos += 4;
    }

    println!("part 1: {}", input[0]);
}

fn part2(input: Vec<i32>) {
    for i in 0..=99 {
        for j in 0..=99 {
            let mut pos = 0;

            let mut input = input.clone();
            input[1] = i;
            input[2] = j;

            while input[pos] != 99 {
                let opcode = input[pos];
                let a = input[input[pos + 1] as usize];
                let b = input[input[pos + 2] as usize];
                let dest = input[pos + 3] as usize;

                let result = match opcode {
                    1 => (a + b),
                    2 => a * b,
                    _ => unreachable!(),
                };

                input[dest] = result;

                pos += 4;
            }

            if input[0] == 19690720 {
                println!("part 2: {}", 100 * i + j);
                return;
            }
        }
    }
}
