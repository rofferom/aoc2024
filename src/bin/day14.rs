use std::collections::HashMap;

const INPUT: &str = include_str!("day14_input.txt");
const INPUT_WIDTH: i32 = 101;
const INPUT_HEIGHT: i32 = 103;

#[derive(Debug)]
struct Robot {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

fn parse(input: &str) -> Vec<Robot> {
    let split_entry = |s: &str| -> (i32, i32) {
        let mut it = s[2..].split(',');

        (
            it.next().unwrap().parse().unwrap(),
            it.next().unwrap().parse().unwrap(),
        )
    };

    input
        .split('\n')
        .map(|l| {
            let mut it = l.split(' ');
            let (x, y) = split_entry(it.next().unwrap());
            let (vx, vy) = split_entry(it.next().unwrap());

            Robot { x, y, vx, vy }
        })
        .collect()
}

fn modulo(v: i32, m: i32) -> i32 {
    let v = v % m;
    if v >= 0 {
        v
    } else {
        v + m
    }
}

fn solve_part1(input: &str, width: i32, height: i32) -> u64 {
    const ITERATIONS: i32 = 100;
    let mut robots = parse(input);

    // Move everything
    for r in &mut robots {
        r.x = modulo(r.x + r.vx * ITERATIONS, width);
        r.y = modulo(r.y + r.vy * ITERATIONS, height);
    }

    // Count quadrants
    let mid_x = width / 2;
    let mid_y = height / 2;

    let mut quadrants = [0; 4];

    for r in robots {
        if r.x < mid_x && r.y < mid_y {
            quadrants[0] += 1;
        } else if r.x < mid_x && r.y > mid_y {
            quadrants[1] += 1;
        } else if r.x > mid_x && r.y < mid_y {
            quadrants[2] += 1;
        } else if r.x > mid_x && r.y > mid_y {
            quadrants[3] += 1;
        }
        // Ignore robots that are in the middle
    }

    quadrants.iter().product()
}

fn solve_part2(input: &str, width: i32, height: i32) -> u64 {
    let mut robots = parse(input);
    let mut i = 0;

    loop {
        for r in &mut robots {
            r.x = modulo(r.x + r.vx, width);
            r.y = modulo(r.y + r.vy, height);
        }

        let mut positions = HashMap::new();
        for r in &robots {
            let entry = positions.entry((r.x, r.y)).or_insert(0);
            *entry += 1;
        }

        i += 1;

        let c = positions.values().filter(|&&v| v > 1).count();
        if c == 0 {
            for y in 0..height {
                for x in 0..width {
                    if let Some(c) = positions.get(&(x, y)) {
                        print!("{c}");
                    } else {
                        print!(".");
                    }
                }

                println!();
            }

            return i;
        }
    }
}

fn main() {
    println!("Part 1: {}", solve_part1(INPUT, INPUT_WIDTH, INPUT_HEIGHT));
    println!("Part 2: {}", solve_part2(INPUT, INPUT_WIDTH, INPUT_HEIGHT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day14() {
        const TEST_INPUT: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

        const TEST_INPUT_WIDTH: i32 = 11;
        const TEST_INPUT_HEIGHT: i32 = 7;

        assert_eq!(
            solve_part1(TEST_INPUT, TEST_INPUT_WIDTH, TEST_INPUT_HEIGHT),
            12
        );
        assert_eq!(solve_part1(INPUT, INPUT_WIDTH, INPUT_HEIGHT), 224438715);
        assert_eq!(solve_part2(INPUT, INPUT_WIDTH, INPUT_HEIGHT), 7603);
    }
}
