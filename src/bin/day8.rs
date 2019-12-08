use aoc_2019::read_input;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_input()?
        .iter()
        .flat_map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<_>>();

    part1(&input);
    part2(&input);

    Ok(())
}

fn part1(input: &[u32]) {
    let w = 25;
    let h = 6;

    let mut layers = vec![];
    input.chunks(w * h).for_each(|e| layers.push(e));

    let mut num_zeroes = vec![];

    for layer in &layers {
        let count = layer
            .iter()
            .fold(0, |acc, x| if x == &0 { acc + 1 } else { acc });
        num_zeroes.push(count);
    }

    let (min_index, _) = num_zeroes
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| a.cmp(b))
        .unwrap();

    let min_zeros_layer = layers[min_index];

    let num_ones = min_zeros_layer
        .iter()
        .fold(0, |acc, x| if x == &1 { acc + 1 } else { acc });
    let num_twos = min_zeros_layer
        .iter()
        .fold(0, |acc, x| if x == &2 { acc + 1 } else { acc });

    println!("part 1: {}", num_ones * num_twos);
}

fn part2(input: &[u32]) {
    // 0 is black
    // 1 is white
    // 2 is transparent

    let w = 25;
    let h = 6;

    let mut layers = vec![];
    input.chunks(w * h).for_each(|e| layers.push(e));

    let mut result = vec![0; w * h];
    for i in 0..result.len() {
        let mut layeri = 0;
        let mut value = layers[layeri][i];
        while value == 2 {
            layeri += 1;
            value = layers[layeri][i];
        }

        result[i] = value;
    }

    println!("part 2:");
    for y in 0..h {
        for x in 0..w {
            print!("{}", result[y * w + x]);
        }
        println!();
    }
}
