use aoc_2019::read_input;

fn main() {
    let input = read_input()
        .iter()
        .map(|s| {
            s.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>();

    part1(&input);
    part2(&input);
}

fn part1(input: &[u32]) {
    let mut current: Vec<u32> = input.into();
    let mut next = Vec::with_capacity(current.len());

    for _ in 0..100 {
        for k in 0..current.len() {
            let mut sum = 0;
            let mut i = k;
            let mut a = 1i32;
            while i < current.len() {
                for j in i..std::cmp::min(i + k + 1, current.len()) {
                    sum += current[j] as i32 * a;
                }

                i += 2 * (k + 1);
                a = -a;
            }

            let last_digit = sum.abs() % 10;
            next.push(last_digit as u32);
        }

        current.clear();
        current.extend(&next);
        next.clear();
    }

    let result = digits_to_num(&current[..8]);
    println!("part 1: {:?}", result);
}

fn digits_to_num(digits: &[u32]) -> u32 {
    let mut result = 0;
    let mut m = 1;
    for i in (0..digits.len()).rev() {
        result += m * digits[i];
        m *= 10;
    }
    result
}

// Full disclosure: used some hints for this one.
fn part2(input: &[u32]) {
    let offset = digits_to_num(&input[..7]) as usize;

    let mut current = input.repeat(10_000);
    let mut next = vec![];

    for _ in 0..100 {
        next.push(*current.last().unwrap());

        let mut i = current.len() - 2;
        while i >= offset {
            let sum = *next.last().unwrap() + current[i];
            let last_digit = sum % 10;
            next.push(last_digit as u32);

            i -= 1;
        }

        current.drain(offset..);
        next.reverse();
        current.extend(&next);
        next.clear();
    }

    let result = digits_to_num(&current[offset..offset + 8]);
    println!("part 2: {:?}", result);
}
