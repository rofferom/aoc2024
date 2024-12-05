use std::collections::HashMap;

const INPUT: &str = include_str!("day5_input.txt");

#[derive(Debug)]
struct Input {
    rules: HashMap<i32, Vec<i32>>,
    updates: Vec<Vec<i32>>,
}

impl Input {
    fn new(input: &str) -> Self {
        let input: Vec<_> = input.split("\n\n").collect();

        let raw_rules: Vec<(i32, i32)> = input[0]
            .split('\n')
            .map(|l| {
                let mut l = l.split('|');
                let before = l.next().unwrap().parse().unwrap();
                let after = l.next().unwrap().parse().unwrap();

                (before, after)
            })
            .collect();

        let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
        for (a, b) in raw_rules {
            let entry = rules.entry(a).or_default();
            entry.push(b);
        }

        let updates: Vec<Vec<i32>> = input[1]
            .split('\n')
            .map(|l| l.split(',').map(|s| s.parse().unwrap()).collect())
            .collect();

        Self { rules, updates }
    }
}

fn solve_part1(input: &str) -> i32 {
    let input = Input::new(input);
    let mut r = 0;

    for update in &input.updates {
        let mut valid = true;
        for i in 0..update.len() {
            // Check if there is a rule for the current item
            let Some(rule_entry) = input.rules.get(&update[i]) else {
                continue;
            };

            // Get the items before the current one
            let slice = &update[0..i];
            if slice.is_empty() {
                continue;
            }

            // Check for misplaced items
            if rule_entry.iter().any(|item| slice.contains(item)) {
                valid = false;
                break;
            }
        }

        if valid {
            r += update[update.len() / 2];
        }
    }

    r
}

fn solve_part2(input: &str) -> i32 {
    let input = Input::new(input);
    let mut r = 0;

    for mut update in input.updates {
        let mut valid = true;

        loop {
            // The current update has to be processed as long as there are
            // misplaced items
            let mut swapped = false;

            for i in 1..update.len() {
                let current: i32 = update[i];
                // Check if there is a rule for the current item
                let Some(current_rule) = input.rules.get(&current) else {
                    continue;
                };

                // Get the items before the current one
                let mut slice = vec![0; i];
                slice.copy_from_slice(&update[0..i]);
                if slice.is_empty() {
                    continue;
                }

                // Look for any misplaced item in the preceding items.
                // A misplaced pair is swapped to have a valid order.
                for rule_item in current_rule {
                    if let Some(idx) = slice.iter().enumerate().find_map(|(idx, &slice_item)| {
                        if *rule_item == slice_item {
                            Some(idx)
                        } else {
                            None
                        }
                    }) {
                        let a = update[i];
                        let b = update[idx];

                        update[idx] = a;
                        update[i] = b;

                        valid = false;
                        swapped = true;

                        break;
                    }
                }
            }

            if !swapped {
                break;
            }
        }

        if valid {
            continue;
        }

        // At this point, valid updates have been ignored, and the remaining
        // invalid updates have been fixed
        r += update[update.len() / 2];
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
    fn day5() {
        const TEST_INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

        assert_eq!(solve_part1(TEST_INPUT), 143);
        assert_eq!(solve_part1(INPUT), 7074);

        assert_eq!(solve_part2(TEST_INPUT), 123);
        assert_eq!(solve_part2(INPUT), 4828);
    }
}
