use aoc_2019::read_input;
use std::cmp::max;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_input()?
        .iter()
        .map(|line| line.parse::<i32>())
        .collect::<Result<Vec<_>, _>>()?;
    part1(&input);
    part2(&input);
    Ok(())
}

fn required_fuel(a: i32) -> i32 {
    a / 3 - 2
}

fn part1(input: &[i32]) {
    let sum: i32 = input.iter().map(|e| required_fuel(*e)).sum();
    println!("part 1: {}", sum);
}

fn part2(input: &[i32]) {
    let sum: i32 = input
        .iter()
        .map(|e| {
            let mut fuel = *e;
            let mut total = 0;
            while fuel != 0 {
                fuel = max(required_fuel(fuel), 0);
                total += fuel;
            }
            total
        })
        .sum();
    println!("part 2: {}", sum);
}
