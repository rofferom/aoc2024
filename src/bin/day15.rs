const INPUT: &str = include_str!("day15_input.txt");

mod part1 {
    #[derive(Clone, Copy, Debug, PartialEq)]
    enum Item {
        Empty,
        Wall,
        Fish,
    }

    #[derive(Debug)]
    struct Grid {
        items: Vec<Vec<Item>>,
        width: i32,
        height: i32,
    }

    impl Grid {
        fn print(&self, sub_x: i32, sub_y: i32) {
            for y in 0..self.height {
                for x in 0..self.width {
                    let c = if x == sub_x && y == sub_y {
                        '@'
                    } else {
                        match self.items[y as usize][x as usize] {
                            Item::Empty => '.',
                            Item::Fish => 'O',
                            Item::Wall => '#',
                        }
                    };

                    print!("{c}");
                }

                println!();
            }

            println!();
        }
    }

    #[derive(Debug)]
    enum Move {
        Up,
        Down,
        Left,
        Right,
    }

    fn parse_input(input: &str) -> (Grid, Vec<Move>, (i32, i32)) {
        // Parse grid
        let mut items = vec![];

        let mut lines = input.lines().enumerate();

        let (_, first) = lines.next().unwrap();
        let width = first.len();

        items.push(vec![Item::Wall; width]);

        let mut sub_x = None;
        let mut sub_y = None;

        for (y, l) in &mut lines {
            if l == first {
                items.push(vec![Item::Wall; width]);
                break;
            } else {
                items.push(
                    l.chars()
                        .enumerate()
                        .map(|(x, c)| match c {
                            '.' => Item::Empty,
                            '#' => Item::Wall,
                            'O' => Item::Fish,
                            '@' => {
                                sub_x = Some(x as i32);
                                sub_y = Some(y as i32);

                                Item::Empty
                            }
                            _ => panic!("Unexpected item {c}"),
                        })
                        .collect(),
                );
            }
        }

        let height = items.len();

        // Parse moves
        lines.next(); // Skip empty line

        let mut moves = vec![];

        for (_, l) in lines {
            l.chars().for_each(|c| {
                let c = match c {
                    '^' => Move::Up,
                    'v' => Move::Down,
                    '<' => Move::Left,
                    '>' => Move::Right,
                    _ => panic!("Unexpected move {c}"),
                };

                moves.push(c);
            });
        }

        (
            Grid {
                items,
                width: width as i32,
                height: height as i32,
            },
            moves,
            (sub_x.unwrap(), sub_y.unwrap()),
        )
    }

    pub(super) fn solve(input: &str) -> i32 {
        let (mut grid, moves, (mut sub_x, mut sub_y)) = parse_input(input);

        for m in moves {
            let (dx, dy) = match m {
                Move::Up => (0, -1),
                Move::Down => (0, 1),
                Move::Left => (-1, 0),
                Move::Right => (1, 0),
            };

            let next_x = sub_x + dx;
            let next_y = sub_y + dy;

            match grid.items[next_y as usize][next_x as usize] {
                Item::Wall => {
                    continue;
                }
                Item::Empty => {
                    sub_x = next_x;
                    sub_y = next_y;
                }
                Item::Fish => {
                    let (mut end_x, mut end_y) = (next_x, next_y);
                    while grid.items[end_y as usize][end_x as usize] == Item::Fish {
                        end_x += dx;
                        end_y += dy;
                    }

                    match grid.items[end_y as usize][end_x as usize] {
                        Item::Empty => {
                            grid.items[end_y as usize][end_x as usize] = Item::Fish;
                            grid.items[next_y as usize][next_x as usize] = Item::Empty;
                            (sub_x, sub_y) = (next_x, next_y);
                        }
                        Item::Wall => {}
                        Item::Fish => {
                            panic!()
                        }
                    }
                }
            }
        }

        let mut r = 0;
        for x in 0..grid.width {
            for y in 0..grid.height {
                if grid.items[y as usize][x as usize] != Item::Fish {
                    continue;
                }

                r += x + y * 100;
            }
        }

        r
    }
}

fn solve_part2(input: &str) -> i32 {
    0
}

fn main() {
    println!("Part 1: {}", part1::solve(INPUT));
    println!("Part 2: {}", solve_part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day15() {
        const TEST_INPUT: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

        const TEST_INPUT2: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

        assert_eq!(part1::solve(TEST_INPUT), 2028);
        assert_eq!(part1::solve(TEST_INPUT2), 10092);
        assert_eq!(part1::solve(INPUT), 1413675);

        const TEST_INPUT3: &str = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";

        //assert_eq!(solve_part2(TEST_INPUT3), 9021);
        //assert_eq!(solve_part2(INPUT), 7603);
    }
}
