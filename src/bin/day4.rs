const INPUT: &str = include_str!("day4_input.txt");

struct Input {
    input: Vec<Vec<char>>,
    width: i32,
    height: i32,
}

impl Input {
    fn new(s: &str) -> Self {
        let input: Vec<Vec<char>> = s.lines().map(|l| l.chars().collect()).collect();
        let width: i32 = input[0].len() as i32;
        let height = input.len() as i32;

        Self {
            input,
            width,
            height,
        }
    }

    fn get_string(&self, len: usize, start_x: i32, start_y: i32, dx: i32, dy: i32) -> Vec<char> {
        let mut x = start_x;
        let mut y = start_y;
        let mut s = vec!['.'; len];

        for i in s.iter_mut() {
            if x < 0 || x >= self.width || y < 0 || y >= self.height {
                break;
            }

            *i = self.input[y as usize][x as usize];

            x += dx;
            y += dy;
        }

        s
    }
}

fn solve_part1(input: &str) -> u64 {
    const XMAS: &[char] = &['X', 'M', 'A', 'S'];
    const XMAS_LEN: usize = XMAS.len();

    let input = Input::new(input);

    let directions = &[
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, 1),
        (-1, 1),
        (1, -1),
        (-1, -1),
    ];

    let mut c = 0;

    for start_y in 0..input.height {
        for start_x in 0..input.width {
            for (dx, dy) in directions {
                let s = input.get_string(XMAS_LEN, start_x, start_y, *dx, *dy);
                if s == XMAS {
                    c += 1;
                }
            }
        }
    }

    c
}

fn solve_part2(input: &str) -> u64 {
    const MAS: &[char] = &['M', 'A', 'S'];
    const SAM: &[char] = &['S', 'A', 'M'];
    const MAS_LEN: usize = MAS.len();

    let input = Input::new(input);

    let mut c = 0;

    for start_y in 0..input.height {
        for start_x in 0..input.width {
            let s = input.get_string(MAS_LEN, start_x, start_y, 1, 1);
            if s != MAS && s != SAM {
                continue;
            }

            let s = input.get_string(MAS_LEN, start_x, start_y + 2, 1, -1);
            if s != MAS && s != SAM {
                continue;
            }

            c += 1;
        }
    }

    c
}

fn main() {
    println!("Part 1: {}", solve_part1(INPUT));
    println!("Part 2: {}", solve_part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day4() {
        const TEST_INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        assert_eq!(solve_part1(TEST_INPUT), 18);
        assert_eq!(solve_part1(INPUT), 2562);

        assert_eq!(solve_part2(TEST_INPUT), 9);
        assert_eq!(solve_part2(INPUT), 1902);
    }
}
