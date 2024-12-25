use std::collections::HashMap;

const INPUT: &str = include_str!("day11_input.txt");

fn solve(raw_input: &str, iterations: usize) -> u64 {
    let mut input: HashMap<u64, u64> = raw_input
        .split(' ')
        .map(|s| (s.parse().unwrap(), 1))
        .collect();

    for _ in 0..iterations {
        let mut next_input = HashMap::new();

        let mut add_value = |k: u64, v: u64| {
            let entry = next_input.entry(k).or_insert(0);
            *entry += v;
        };

        for (k, v) in input.into_iter() {
            if k == 0 {
                add_value(1, v);
            } else {
                let digits = k.ilog10() + 1;
                if digits % 2 == 0 {
                    let p = 10u64.pow(digits / 2);

                    add_value(k / p, v);
                    add_value(k % p, v);
                } else {
                    add_value(k * 2024, v);
                }
            }
        }

        input = next_input;
    }

    input.values().copied().sum()
}

fn solve_part1(input: &str) -> u64 {
    solve(input, 25)
}

fn solve_part2(input: &str) -> u64 {
    solve(input, 75)
}

fn main() {
    println!("Part 1: {}", solve_part1(INPUT));
    println!("Part 2: {}", solve_part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day11() {
        const TEST_INPUT: &str = "125 17";

        assert_eq!(solve_part1(TEST_INPUT), 55312);
        assert_eq!(solve_part1(INPUT), 194782);
        assert_eq!(solve_part2(INPUT), 233007586663131);
    }
}
