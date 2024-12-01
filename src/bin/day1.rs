use std::collections::HashMap;

const INPUT: &str = include_str!("day1_input.txt");

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    input
        .split('\n')
        .map(|l| {
            let mut s = l.split("   ").map(|s| s.parse::<i32>().unwrap());
            (s.next().unwrap(), s.next().unwrap())
        })
        .unzip()
}

fn solve_part1(input: &str) -> i32 {
    let (mut left, mut right) = parse_input(input);
    left.sort();
    right.sort();
    assert_eq!(left.len(), right.len());

    left.iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum()
}

fn solve_part2(input: &str) -> i32 {
    let (left, right) = parse_input(input);

    let mut c = HashMap::new();

    for v in right {
        let entry = c.entry(v).or_insert(0);
        *entry += 1;
    }

    let mut similarity = 0;

    for v in left {
        similarity += v * c.get(&v).unwrap_or(&0);
    }

    similarity
}

fn main() {
    println!("Part 1: {}", solve_part1(INPUT));
    println!("Part 2: {}", solve_part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1() {
        const TEST_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

        assert_eq!(solve_part1(TEST_INPUT), 11);
        assert_eq!(solve_part1(INPUT), 1258579);

        assert_eq!(solve_part2(TEST_INPUT), 31);
        assert_eq!(solve_part2(INPUT), 23981443);
    }
}
