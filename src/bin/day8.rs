use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("day8_input.txt");

struct Input {
    antennas: HashMap<char, Vec<(i32, i32)>>,
    height: i32,
    width: i32,
}

impl Input {
    fn new(input: &str) -> Self {
        let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
        let height = input.lines().count() as i32;
        let width = input.lines().next().unwrap().chars().count() as i32;

        for (y, l) in input.lines().enumerate() {
            for (x, c) in l.chars().enumerate() {
                if c == '.' {
                    continue;
                }

                let entry = antennas.entry(c).or_default();
                entry.push((x as i32, y as i32));
            }
        }

        Self {
            antennas,
            height,
            width,
        }
    }

    fn inside_grid(&self, x: i32, y: i32) -> bool {
        0 <= x && x < self.width && 0 <= y && y < self.height
    }
}

fn solve_part1(input: &str) -> usize {
    let input = Input::new(input);
    let mut antinodes = HashSet::new();

    for positions in input.antennas.values() {
        for i in 0..positions.len() {
            let (x, y) = positions[i];

            for (j, (other_x, other_y)) in positions.iter().enumerate() {
                if j == i {
                    continue;
                }

                let antinode_x = (other_x - x) + other_x;
                let antinode_y = (other_y - y) + other_y;

                if !input.inside_grid(antinode_x, antinode_y) {
                    continue;
                }

                antinodes.insert((antinode_x, antinode_y));
            }
        }
    }

    antinodes.len()
}

fn solve_part2(input: &str) -> usize {
    let input = Input::new(input);
    let mut antinodes = HashSet::new();

    for positions in input.antennas.values() {
        for i in 0..positions.len() {
            let (x, y) = positions[i];

            for (j, (other_x, other_y)) in positions.iter().enumerate() {
                if j == i {
                    continue;
                }

                let mut k = 0;

                loop {
                    let antinode_x = k * (other_x - x) + other_x;
                    let antinode_y = k * (other_y - y) + other_y;

                    if !input.inside_grid(antinode_x, antinode_y) {
                        break;
                    }

                    antinodes.insert((antinode_x, antinode_y));
                    k += 1;
                }
            }
        }
    }

    antinodes.len()
}

fn main() {
    println!("Part 1: {}", solve_part1(INPUT));
    println!("Part 2: {}", solve_part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day8() {
        const TEST_INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

        assert_eq!(solve_part1(TEST_INPUT), 14);
        assert_eq!(solve_part1(INPUT), 228);

        assert_eq!(solve_part2(TEST_INPUT), 34);
        assert_eq!(solve_part2(INPUT), 766);
    }
}
