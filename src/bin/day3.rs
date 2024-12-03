const INPUT: &str = include_str!("day3_input.txt");

#[derive(Debug)]
struct Input<'a> {
    s: &'a str,
    begin: usize,
}

#[derive(Debug)]
struct Condition {
    enabled: bool,
    ending: Option<usize>,
}

fn compute_matches(input: &str, matches: &[usize]) -> u64 {
    let mut r = 0;

    for i in 0..matches.len() {
        let s = if i < matches.len() - 1 {
            let begin = matches[i];
            let end = matches[i + 1];
            &input[begin..end]
        } else {
            let begin = matches[i];
            &input[begin..]
        };

        let Some(closing) = s.find(')') else {
            continue;
        };

        // Skip "mul("
        let s = &s[4..closing];

        let Some(comma) = s.find(',') else {
            continue;
        };

        let Ok(left) = s[0..comma].parse::<u64>() else {
            continue;
        };

        let Ok(right) = s[comma + 1..].parse::<u64>() else {
            continue;
        };

        r += left * right;
    }

    r
}

fn solve_part1(input: &str) -> u64 {
    let mut next_inputs = Some(Input { s: input, begin: 0 });
    let mut matches = vec![];

    while let Some(input) = next_inputs.take() {
        let Some(m) = input.s.find("mul(") else {
            break;
        };

        let m_begin = input.begin + m;
        next_inputs = Some(Input {
            s: &input.s[m + 1..],
            begin: m_begin + 1,
        });

        matches.push(m_begin);
    }

    compute_matches(input, &matches)
}

fn solve_part2(input: &str) -> u64 {
    let mut next_inputs = Some(Input { s: input, begin: 0 });
    let mut matches = vec![];

    let mut condition = Condition {
        enabled: true,
        ending: input.find("don't()"),
    };

    while let Some(current_input) = next_inputs.take() {
        let Some(m) = current_input.s.find("mul(") else {
            break;
        };

        let m_begin = current_input.begin + m;
        next_inputs = Some(Input {
            s: &current_input.s[m + 1..],
            begin: m_begin + 1,
        });

        match &condition {
            Condition {
                enabled: true,
                ending: Some(ending),
            } => {
                if m_begin < *ending {
                    matches.push(m_begin);
                } else {
                    let starting = *ending + 1;

                    condition = Condition {
                        enabled: false,
                        ending: input[starting..]
                            .find("do()")
                            .map(|ending| starting + ending),
                    };
                }
            }
            Condition {
                enabled: true,
                ending: None,
            } => {
                matches.push(m_begin);
            }
            Condition {
                enabled: false,
                ending: Some(ending),
            } => {
                if m_begin > *ending {
                    let starting = *ending + 1;

                    condition = Condition {
                        enabled: true,
                        ending: input[starting..]
                            .find("don't()")
                            .map(|ending| starting + ending),
                    };

                    matches.push(m_begin);
                }
            }
            Condition {
                enabled: false,
                ending: None,
            } => {}
        }
    }

    compute_matches(input, &matches)
}

fn main() {
    println!("Part 1: {}", solve_part1(INPUT));
    println!("Part 2: {}", solve_part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day3() {
        const TEST_INPUT1: &str =
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        assert_eq!(solve_part1(TEST_INPUT1), 161);
        assert_eq!(solve_part1(INPUT), 189600467);

        const TEST_INPUT2: &str =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        assert_eq!(solve_part2(TEST_INPUT2), 48);
        assert_eq!(solve_part2(INPUT), 107069718);
    }
}
