use std::collections::HashSet;

const INPUT: &str = include_str!("day10_input.txt");

#[derive(Debug)]
struct Input {
    grid: Vec<Vec<u32>>,
    width: i32,
    height: i32,
}

fn parse_input(input: &str) -> Input {
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap_or(10)).collect())
        .collect();

    let height = grid.len() as i32;
    let width = grid[0].len() as i32;

    Input {
        grid,
        width,
        height,
    }
}

fn find_trailhead(
    input: &Input,
    found_paths: &mut Vec<Vec<(i32, i32)>>,
    current_path: Vec<(i32, i32)>,
    x: i32,
    y: i32,
    altitude: u32,
) {
    const DIRS: &[(i32, i32)] = &[(-1, 0), (1, 0), (0, -1), (0, 1)];

    for (dx, dy) in DIRS {
        let next_x = x + dx;
        let next_y = y + dy;

        if next_x < 0 || next_x >= input.width || next_y < 0 || next_y >= input.height {
            continue;
        }

        let v = input.grid[next_y as usize][next_x as usize];
        if v != altitude {
            continue;
        }

        let mut next_path = current_path.clone();
        next_path.push((next_x, next_y));

        if v == 9 {
            found_paths.push(next_path);
        } else {
            find_trailhead(input, found_paths, next_path, next_x, next_y, altitude + 1);
        }
    }
}

fn solve_part1(input: &str) -> usize {
    let input = parse_input(input);
    let mut r = 0;

    for y in 0..input.height {
        for x in 0..input.width {
            let v = input.grid[y as usize][x as usize];
            if v != 0 {
                continue;
            }

            let mut found_paths = vec![];
            find_trailhead(&input, &mut found_paths, vec![(x, y)], x, y, 1);

            let unique_nine: HashSet<_> = found_paths
                .into_iter()
                .map(|path| path[path.len() - 1])
                .collect();
            r += unique_nine.len();
        }
    }

    r
}

fn solve_part2(input: &str) -> usize {
    let input = parse_input(input);
    let mut r = 0;

    for y in 0..input.height {
        for x in 0..input.width {
            let v = input.grid[y as usize][x as usize];
            if v != 0 {
                continue;
            }

            let mut found = vec![];
            find_trailhead(&input, &mut found, vec![(x, y)], x, y, 1);

            r += found.len();
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
    fn day10() {
        const TEST_INPUT1_1: &str = "10..9..
2...8..
3...7..
4567654
...8..3
...9..2
.....01";
        assert_eq!(solve_part1(TEST_INPUT1_1), 3);

        const TEST_INPUT1_2: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        assert_eq!(solve_part1(TEST_INPUT1_2), 36);
        assert_eq!(solve_part1(INPUT), 548);

        assert_eq!(solve_part2(TEST_INPUT1_2), 81);
        assert_eq!(solve_part2(INPUT), 1252);
    }
}
