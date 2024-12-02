const INPUT: &str = include_str!("day2_input.txt");

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .split('\n')
        .map(|l| l.split(' ').map(|s| s.parse::<i32>().unwrap()).collect())
        .collect()
}

fn report_valid(report: &[i32]) -> bool {
    let report_increasing = report[0] < report[1];

    for i in 0..(report.len() - 1) {
        let increasing = report[i] < report[i + 1];
        if increasing != report_increasing {
            return false;
        }

        let distance = (report[i] - report[i + 1]).abs();
        if distance == 0 || distance > 3 {
            return false;
        }
    }

    true
}

fn solve_part1(input: &str) -> usize {
    let input = parse_input(input);

    input.iter().filter(|&report| report_valid(report)).count()
}

fn solve_part2(input: &str) -> u32 {
    let mut result = 0;
    let input = parse_input(input);

    for report in input {
        let report_len = report.len();

        if report_valid(&report) {
            result += 1;
            continue;
        }

        for i in 0..report_len {
            let filtered_report: Vec<_> = report
                .iter()
                .enumerate()
                .filter_map(|(idx, v)| if idx == i { None } else { Some(*v) })
                .collect();

            if report_valid(&filtered_report) {
                result += 1;
                break;
            }
        }
    }

    result
}

fn main() {
    println!("Part 1: {}", solve_part1(INPUT));
    println!("Part 2: {}", solve_part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day2() {
        const TEST_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

        assert_eq!(solve_part1(TEST_INPUT), 2);
        assert_eq!(solve_part1(INPUT), 314);

        assert_eq!(solve_part2(TEST_INPUT), 4);
        assert_eq!(solve_part2(INPUT), 373);
    }
}
