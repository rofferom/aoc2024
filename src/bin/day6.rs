use std::collections::HashSet;

const INPUT: &str = include_str!("day6_input.txt");

#[derive(Clone, Debug, PartialEq)]
enum Item {
    Empty,
    Obstacle,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Guard {
    x: i32,
    y: i32,
    dir: Direction,
}

#[derive(Clone, Debug)]
struct Grid {
    grid: Vec<Vec<Item>>,
    width: i32,
    height: i32,
}

impl Grid {
    fn new(grid: Vec<Vec<Item>>) -> Self {
        let width = grid[0].len() as i32;
        let height = grid.len() as i32;

        Self {
            grid,
            width,
            height,
        }
    }
}

fn parse_input(input: &str) -> (Grid, Guard) {
    let mut guard = None;

    let grid = input
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '.' => Item::Empty,
                    '#' => Item::Obstacle,
                    '^' => {
                        guard = Some(Guard {
                            x: x as i32,
                            y: y as i32,
                            dir: Direction::Up,
                        });

                        Item::Empty
                    }
                    'v' => {
                        guard = Some(Guard {
                            x: x as i32,
                            y: y as i32,
                            dir: Direction::Down,
                        });

                        Item::Empty
                    }
                    '<' => {
                        guard = Some(Guard {
                            x: x as i32,
                            y: y as i32,
                            dir: Direction::Left,
                        });

                        Item::Empty
                    }
                    '>' => {
                        guard = Some(Guard {
                            x: x as i32,
                            y: y as i32,
                            dir: Direction::Right,
                        });

                        Item::Empty
                    }
                    _ => {
                        panic!("Unexpected input");
                    }
                })
                .collect()
        })
        .collect();

    (Grid::new(grid), guard.unwrap())
}

fn solve_part1(input: &str) -> usize {
    let (grid, mut guard) = parse_input(input);
    let mut positions = HashSet::new();

    loop {
        let (next_x, next_y) = match guard.dir {
            Direction::Up => (guard.x, guard.y - 1),
            Direction::Down => (guard.x, guard.y + 1),
            Direction::Left => (guard.x - 1, guard.y),
            Direction::Right => (guard.x + 1, guard.y),
        };

        if next_x < 0 || next_x >= grid.width || next_y < 0 || next_y >= grid.height {
            break;
        } else if grid.grid[next_y as usize][next_x as usize] == Item::Obstacle {
            guard.dir = match guard.dir {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
            };
        } else {
            guard.x = next_x;
            guard.y = next_y;

            positions.insert((guard.x, guard.y));
        }
    }

    positions.len()
}

fn has_loop(grid: Grid, mut guard: Guard) -> bool {
    let mut positions = HashSet::new();

    loop {
        let (next_x, next_y) = match guard.dir {
            Direction::Up => (guard.x, guard.y - 1),
            Direction::Down => (guard.x, guard.y + 1),
            Direction::Left => (guard.x - 1, guard.y),
            Direction::Right => (guard.x + 1, guard.y),
        };

        if next_x < 0 || next_x >= grid.width || next_y < 0 || next_y >= grid.height {
            return false;
        }

        if grid.grid[next_y as usize][next_x as usize] == Item::Obstacle {
            guard.dir = match guard.dir {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
            };
        } else {
            guard.x = next_x;
            guard.y = next_y;
        }

        // Consider we are in a loop if the guard pass twice in the same location,
        // with the same direction
        if !positions.insert(guard.clone()) {
            return true;
        }
    }
}

fn solve_part2(input: &str) -> i32 {
    let (grid, guard) = parse_input(input);

    let mut r = 0;

    for y in 0..grid.height {
        for x in 0..grid.width {
            // Avoid to crush the guard with an obstacle
            if x == guard.x && y == guard.y {
                continue;
            }

            if grid.grid[y as usize][x as usize] == Item::Empty {
                let mut grid = grid.clone();
                grid.grid[y as usize][x as usize] = Item::Obstacle;

                if has_loop(grid, guard.clone()) {
                    r += 1;
                }
            }
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
    fn day6() {
        const TEST_INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

        assert_eq!(solve_part1(TEST_INPUT), 41);
        assert_eq!(solve_part1(INPUT), 4967);

        assert_eq!(solve_part2(TEST_INPUT), 6);
        assert_eq!(solve_part2(INPUT), 1789);
    }
}
