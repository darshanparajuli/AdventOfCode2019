use aoc_2019::parse_input;
use std::cmp::max;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = parse_input(|line| line.parse::<i32>().unwrap())?;
    part1(&input);
    part2(&input);
    Ok(())
}

fn part1(input: &[i32]) {
    let sum: i32 = input.iter().map(|e| e / 3 - 2).sum();
    println!("part 1: {}", sum);
}

fn part2(input: &[i32]) {
    let sum: i32 = input
        .iter()
        .map(|e| {
            let mut fuel = *e;
            let mut total = 0;
            while fuel != 0 {
                fuel = max(fuel / 3 - 2, 0);
                total += fuel;
            }
            total
        })
        .sum();
    println!("part 2: {}", sum);
}
