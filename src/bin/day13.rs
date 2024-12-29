const INPUT: &str = include_str!("day13_input.txt");

#[derive(Debug)]
struct Button {
    dx: i64,
    dy: i64,
}

#[derive(Debug)]
struct Machine {
    btn_a: Button,
    btn_b: Button,
    prize_x: i64,
    prize_y: i64,
}

fn parse_button(input: &str) -> Button {
    // Skip "Button X: "
    let input = &input[10..];
    let input: Vec<_> = input.split(", ").collect();

    let parse_axis = |s: &str| -> i64 { s[2..].parse().unwrap() };

    Button {
        dx: parse_axis(input[0]),
        dy: parse_axis(input[1]),
    }
}

fn parse_prize(input: &str) -> (i64, i64) {
    // Skip "Prize: "
    let input = &input[7..];
    let input: Vec<_> = input.split(", ").collect();

    let parse_axis = |s: &str| -> i64 { s[2..].parse().unwrap() };

    (parse_axis(input[0]), parse_axis(input[1]))
}

fn parse_inputs(input: &str) -> Vec<Machine> {
    input
        .split("\n\n")
        .map(|block| {
            let lines: Vec<_> = block.split('\n').collect();

            let btn_a = parse_button(lines[0]);
            let btn_b = parse_button(lines[1]);
            let (prize_x, prize_y) = parse_prize(lines[2]);

            Machine {
                btn_a,
                btn_b,
                prize_x,
                prize_y,
            }
        })
        .collect()
}

fn solve(machine: &Machine) -> Option<(i64, i64)> {
    let det = machine.btn_a.dx * machine.btn_b.dy - machine.btn_a.dy * machine.btn_b.dx;
    if det == 0 {
        return None;
    }

    let a = machine.btn_b.dy * machine.prize_x - machine.btn_b.dx * machine.prize_y;
    if a % det != 0 {
        return None;
    }

    let b = machine.btn_a.dx * machine.prize_y - machine.btn_a.dy * machine.prize_x;
    if b % det != 0 {
        return None;
    }

    Some((a / det, b / det))
}

fn solve_part1(input: &str) -> i64 {
    let machines = parse_inputs(input);
    let mut r = 0;

    for machine in machines {
        let Some((a, b)) = solve(&machine) else {
            continue;
        };

        if a > 100 || b > 100 {
            continue;
        }

        r += a * 3 + b;
    }

    r
}

fn solve_part2(input: &str) -> i64 {
    let machines = parse_inputs(input);
    let mut r = 0;

    for mut machine in machines {
        machine.prize_x += 10000000000000;
        machine.prize_y += 10000000000000;

        let Some((a, b)) = solve(&machine) else {
            continue;
        };

        r += a * 3 + b;
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
    fn day13() {
        const TEST_INPUT: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

        assert_eq!(solve_part1(TEST_INPUT), 480);
        assert_eq!(solve_part1(INPUT), 36250);
        assert_eq!(solve_part2(INPUT), 83232379451012);
    }
}
