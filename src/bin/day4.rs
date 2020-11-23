fn main() {
    let input = (147981, 691423);

    let (min, max) = input;
    part1(min, max);
    part2(min, max);
}

fn has_two_same_adjacent_digits(mut n: u32) -> bool {
    for _ in 0..6 {
        let i = n % 10;
        n /= 10;
        let j = n % 10;

        if i == j {
            return true;
        }
    }

    false
}

fn is_increasing_from_left_to_right(mut n: u32) -> bool {
    let mut i = n % 10;
    n /= 10;

    for _ in 0..5 {
        let j = n % 10;
        if j > i {
            return false;
        }

        i = j;
        n /= 10;
    }
    true
}

fn part1(min: u32, max: u32) {
    let mut count = 0;

    for i in min..=max {
        if has_two_same_adjacent_digits(i) && is_increasing_from_left_to_right(i) {
            count += 1;
        }
    }

    println!("part 1: {}", count);
}

fn has_two_same_adjacent_digits2(mut n: u32) -> bool {
    let mut k = 1;
    let mut v = vec![];

    for _ in 0..6 {
        let a = n % 10;
        n /= 10;
        let b = n % 10;

        if a == b {
            k += 1;
        } else {
            if k > 1 {
                v.push(k);
            }

            k = 1;
        }
    }

    v.contains(&2)
}

fn part2(min: u32, max: u32) {
    let mut count = 0;

    for i in min..=max {
        if has_two_same_adjacent_digits2(i) && is_increasing_from_left_to_right(i) {
            count += 1;
        }
    }

    println!("part 2: {}", count);
}
