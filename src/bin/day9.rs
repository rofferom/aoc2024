const INPUT: &str = include_str!("day9_input.txt");

fn checksum(input: &[Option<usize>]) -> usize {
    input
        .iter()
        .enumerate()
        .filter_map(|(i, v)| (*v).map(|v| i * v))
        .sum()
}

fn solve_part1(raw_input: &str) -> usize {
    let mut input = vec![];

    // Parse input
    for (idx, c) in raw_input
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .enumerate()
    {
        if idx % 2 == 0 {
            let id = idx / 2;
            // File block
            for _ in 0..c {
                input.push(Some(id));
            }
        } else {
            // Free space block
            for _ in 0..c {
                input.push(None);
            }
        }
    }

    // Helpers
    let next_free = |l: &mut Vec<Option<usize>>, start_idx: usize| {
        (start_idx..l.len()).find(|&i| l[i].is_none())
    };

    let last_used = |l: &mut Vec<Option<usize>>, start_idx: usize| {
        (0..=start_idx).rev().find(|&i| l[i].is_some())
    };

    // Swap blocks
    let l_len = input.len();
    let mut next_free_idx = next_free(&mut input, 0).unwrap();
    let mut last_used_idx = last_used(&mut input, l_len - 1).unwrap();

    while next_free_idx < last_used_idx {
        input[next_free_idx] = input[last_used_idx];
        input[last_used_idx] = None;

        next_free_idx = next_free(&mut input, next_free_idx).unwrap();
        last_used_idx = last_used(&mut input, last_used_idx).unwrap();
    }

    // Compute checksum
    checksum(&input)
}

#[derive(Debug)]
enum Block {
    File { id: i32, size: usize },
    Empty { size: usize },
}

fn solve_part2(raw_input: &str) -> usize {
    let mut input = vec![];
    let mut next_id_to_move = -1;

    // Parse input and track the highest block id. This is the first to be moved
    for (idx, size) in raw_input
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .enumerate()
    {
        if idx % 2 == 0 {
            let id = (idx as i32) / 2;

            input.push(Block::File {
                id,
                size: size as _,
            });

            next_id_to_move = id;
        } else if size > 0 {
            input.push(Block::Empty { size: size as _ });
        }
    }

    // Try to move all blocks
    while next_id_to_move >= 0 {
        // Find next block to move
        let (idx_to_move, id_to_move, size_to_move) = input
            .iter()
            .enumerate()
            .find_map(|(idx, entry)| match entry {
                Block::File { id, size } => {
                    if *id == next_id_to_move {
                        Some((idx, *id, *size))
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .unwrap();

        // Find empty space, from right to left
        let mut free_block = None;

        for i in (0..idx_to_move).rev() {
            let Block::Empty { size } = input[i] else {
                continue;
            };

            if size >= size_to_move {
                free_block = Some((i, size));
            }
        }

        // Move block to available place
        if let Some((free_idx, free_size)) = free_block {
            let remaining_free_size = free_size - size_to_move;

            input[free_idx] = Block::File {
                id: id_to_move,
                size: size_to_move,
            };

            input[idx_to_move] = Block::Empty { size: size_to_move };

            if remaining_free_size > 0 {
                input.insert(
                    free_idx + 1,
                    Block::Empty {
                        size: remaining_free_size,
                    },
                );
            }
        }

        // Merge possible consecutive empty blocks
        loop {
            let mut merged = false;

            for i in 0..(input.len() - 1) {
                let (Block::Empty { size: size_i }, Block::Empty { size: size_i1 }) =
                    (&input[i], &input[i + 1])
                else {
                    continue;
                };

                input[i] = Block::Empty {
                    size: size_i + size_i1,
                };

                input.remove(i + 1);

                merged = true;
                break;
            }

            if !merged {
                break;
            }
        }

        next_id_to_move -= 1;
    }

    // Dirty : rework the Vec to have the same layout than in part 1
    let mut input_like_part1 = vec![];

    for block in input {
        match block {
            Block::File { id, size } => {
                for _ in 0..size {
                    input_like_part1.push(Some(id as usize));
                }
            }
            Block::Empty { size } => {
                for _ in 0..size {
                    input_like_part1.push(None);
                }
            }
        }
    }

    checksum(&input_like_part1)
}

fn main() {
    println!("Part 1: {}", solve_part1(INPUT));
    println!("Part 2: {}", solve_part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day9() {
        const TEST_INPUT: &str = "2333133121414131402";

        assert_eq!(solve_part1(TEST_INPUT), 1928);
        assert_eq!(solve_part1(INPUT), 6241633730082);

        assert_eq!(solve_part2(TEST_INPUT), 2858);
        assert_eq!(solve_part2(INPUT), 6265268809555);
    }
}
