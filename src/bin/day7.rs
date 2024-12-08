const INPUT: &str = include_str!("day7_input.txt");

fn compute_part1(results: &mut Vec<usize>, current: usize, remainings: &[usize]) {
    let add = current + remainings[0];
    let mul = current * remainings[0];

    let remainings = &remainings[1..];
    if remainings.is_empty() {
        results.push(add);
        results.push(mul);
    } else {
        compute_part1(results, add, remainings);
        compute_part1(results, mul, remainings);
    }
}

fn solve_part1(input: &str) -> usize {
    let mut r = 0;

    for l in input.lines() {
        let mut it = l.split(": ");
        let target: usize = it.next().unwrap().parse().unwrap();
        let values: Vec<usize> = it
            .next()
            .unwrap()
            .split(' ')
            .map(|s| s.parse().unwrap())
            .collect();

        let mut results = vec![];
        compute_part1(&mut results, values[0], &values[1..]);

        if results.contains(&target) {
            r += target;
        }
    }

    r
}

fn compute_part2(results: &mut Vec<usize>, current: usize, remainings: &[usize]) {
    let add = current + remainings[0];
    let mul = current * remainings[0];
    let concat = current * 10usize.pow(remainings[0].ilog10() + 1) + remainings[0];

    let remainings = &remainings[1..];
    if remainings.is_empty() {
        results.push(add);
        results.push(mul);
        results.push(concat);
    } else {
        compute_part2(results, add, remainings);
        compute_part2(results, mul, remainings);
        compute_part2(results, concat, remainings);
    }
}

fn solve_part2(input: &str) -> usize {
    let mut r = 0;

    for l in input.lines() {
        let mut it = l.split(": ");
        let target: usize = it.next().unwrap().parse().unwrap();
        let values: Vec<usize> = it
            .next()
            .unwrap()
            .split(' ')
            .map(|s| s.parse().unwrap())
            .collect();

        let mut results = vec![];
        compute_part2(&mut results, values[0], &values[1..]);

        if results.contains(&target) {
            r += target;
        }
    }

    r
}

fn main() {
    println!("Part 1: {}", solve_part1(INPUT));
    println!("Part 2: {}", solve_part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day7() {
        const TEST_INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

        assert_eq!(solve_part1(TEST_INPUT), 3749);
        assert_eq!(solve_part1(INPUT), 303876485655);

        assert_eq!(solve_part2(TEST_INPUT), 11387);
        assert_eq!(solve_part2(INPUT), 146111650210682);
    }
}
