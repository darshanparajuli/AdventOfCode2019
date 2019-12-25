use aoc_2019::read_input;
use std::collections::HashMap;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_input()?
        .iter()
        .map(|s| {
            let mut v = vec![];

            let mut s = &s[..];
            while let Some(i) = s.find(',') {
                let index = s.find(' ').unwrap();
                let amount = &s[..index].parse::<i64>().unwrap();
                let name = s[index + 1..i].to_owned();

                v.push(Chemical::new(name, *amount));

                s = &s[i + 2..];
            }

            let next_index = s.find("=>").unwrap();
            {
                let index = s.find(' ').unwrap();
                let amount = &s[..index].parse::<i64>().unwrap();

                let name = s[index + 1..next_index - 1].to_owned();
                v.push(Chemical::new(name, *amount));
            }

            s = &s[next_index + 3..];

            let index = s.find(' ').unwrap();
            let amount = &s[..index].parse::<i64>().unwrap();
            let name = s[index + 1..].to_owned();

            Reaction::new(v, Chemical::new(name, *amount))
        })
        .collect::<Vec<_>>();

    part1(&input);
    part2(&input);

    Ok(())
}

fn part1(input: &[Reaction]) {
    let mut reaction_map = HashMap::new();
    let mut ore_map = HashMap::new();

    for r in input {
        reaction_map.insert(&r.output.name, r);
    }

    input.iter().filter(|r| r.has_ore_input()).for_each(|r| {
        ore_map.insert(&r.output.name, r);
    });

    let mut count = HashMap::new();
    let mut remaining = HashMap::new();

    let fuel = reaction_map[&"FUEL".to_string()];

    part1_helper(
        &fuel.output.name,
        &reaction_map,
        fuel.output.amount,
        &mut count,
        &mut remaining,
    );

    let mut total = 0;
    for (k, v) in count {
        let r = ore_map[k];
        let run_count = r.required_runs(v);
        total += run_count.count * r.ore_amount();
    }

    println!("part 1: {}", total);
}

fn part2(input: &[Reaction]) {
    let mut reaction_map = HashMap::new();
    let mut ore_map = HashMap::new();

    for r in input {
        reaction_map.insert(&r.output.name, r);
    }

    input.iter().filter(|r| r.has_ore_input()).for_each(|r| {
        ore_map.insert(&r.output.name, r);
    });

    let mut count = HashMap::new();
    let mut remaining = HashMap::new();

    let fuel = reaction_map[&"FUEL".to_string()];

    let total_ores = 1_000_000_000_000i64;

    let mut low = 0;
    let mut high = total_ores;

    let total: i64;

    loop {
        if low >= high {
            total = high;
            break;
        }

        let mid = (low + high) / 2;
        part1_helper(
            &fuel.output.name,
            &reaction_map,
            mid,
            &mut count,
            &mut remaining,
        );

        let mut t = 0;
        for (k, v) in &count {
            let r = ore_map[k];
            let run_count = r.required_runs(*v - *remaining.entry(k).or_insert(0));
            t += run_count.count * r.ore_amount();
        }

        remaining.clear();
        count.clear();

        if t > total_ores {
            high = mid - 1;
        } else if t < total_ores {
            low = mid + 1;
        }
    }

    println!("part 2: {}", total);
}

fn part1_helper<'a>(
    output: &'a String,
    reaction_map: &HashMap<&'a String, &'a Reaction>,
    output_acc: i64,
    count: &mut HashMap<&'a String, i64>,
    remaining: &mut HashMap<&'a String, i64>,
) {
    let r = reaction_map[output];
    if r.has_ore_input() {
        *count.entry(output).or_insert(0) += output_acc;
    } else {
        let run_count = r.required_runs(output_acc - *remaining.entry(output).or_insert(0));
        remaining.insert(output, run_count.excess);

        for i in &r.input {
            part1_helper(
                &i.name,
                reaction_map,
                i.amount * run_count.count,
                count,
                remaining,
            );
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Chemical {
    name: String,
    amount: i64,
}

impl Chemical {
    fn new(name: String, amount: i64) -> Self {
        Self { name, amount }
    }

    fn is_ore(&self) -> bool {
        self.name == "ORE"
    }
}

#[derive(Debug)]
struct Reaction {
    input: Vec<Chemical>,
    output: Chemical,
}

impl Reaction {
    fn new(input: Vec<Chemical>, output: Chemical) -> Self {
        Self { input, output }
    }

    fn has_ore_input(&self) -> bool {
        self.input.len() == 1 && self.input.first().unwrap().is_ore()
    }

    fn ore_amount(&self) -> i64 {
        self.input.first().unwrap().amount
    }

    fn required_runs(&self, output_amt: i64) -> RunCount {
        let c = ((output_amt as f64) / (self.output.amount as f64)).ceil() as i64;
        let a = c * self.output.amount - output_amt;
        RunCount {
            count: c,
            excess: a,
        }
    }
}

#[derive(Debug)]
struct RunCount {
    count: i64,
    excess: i64,
}
