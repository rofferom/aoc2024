const INPUT: &str = include_str!("day12_input.txt");

#[derive(Clone, Debug)]
struct Region {
    c: char,
    items: Vec<(i32, i32)>,
}

#[derive(Debug)]
struct Item {
    c: char,
    region: Option<usize>,
}

#[derive(Debug)]
struct Input {
    grid: Vec<Vec<Item>>,
    width: i32,
    height: i32,
}

fn parse_input(input: &str) -> Input {
    let grid: Vec<Vec<Item>> = input
        .lines()
        .map(|l| l.chars().map(|c| Item { c, region: None }).collect())
        .collect();

    let width = grid[0].len() as i32;
    let height = grid.len() as i32;

    Input {
        grid,
        width,
        height,
    }
}

fn solve_part1(input: &str) -> usize {
    let mut input = parse_input(input);
    let mut regions: Vec<Region> = vec![];
    let mut region_next_id = 0;

    const DIRS: &[(i32, i32)] = &[(-1, 0), (1, 0), (0, -1), (0, 1)];

    for y in 0..input.height {
        for x in 0..input.width {
            let c = input.grid[y as usize][x as usize].c;

            for (dx, dy) in DIRS {
                let visit_x = x + dx;
                if visit_x < 0 || visit_x >= input.width {
                    continue;
                }

                let visit_y = y + dy;
                if visit_y < 0 || visit_y >= input.height {
                    continue;
                }

                let visit_c = input.grid[visit_y as usize][visit_x as usize].c;
                if c == visit_c {
                    let Some(id) = input.grid[visit_y as usize][visit_x as usize].region else {
                        continue;
                    };

                    //println!("{x}x{y} - Visit {visit_x}x{visit_y}. Existing region {id}");
                    input.grid[y as usize][x as usize].region = Some(id);
                    regions[id].items.push((x, y));
                    break;
                }
            }

            if input.grid[y as usize][x as usize].region.is_none() {
                let id = region_next_id;
                region_next_id += 1;

                //println!("{x}x{y}. New region {id}");
                input.grid[y as usize][x as usize].region = Some(id);

                let region = Region {
                    c,
                    items: vec![(x, y)],
                };
                regions.push(region);
            }
        }
    }

    // Merge regions
    loop {
        let mut merged_count = 0;
        let mut merged_regions = vec![regions[0].clone()];

        for i in 1..regions.len() {
            let mut merged = false;
            for j in 0..merged_regions.len() {
                if regions[i].c != merged_regions[j].c {
                    continue;
                }

                let mut merge = false;
                'outer: for (x, y) in &regions[i].items {
                    for (dx, dy) in DIRS {
                        if merged_regions[j].items.contains(&(x + dx, y + dy)) {
                            merge = true;
                            break 'outer;
                        }
                    }
                }

                if merge {
                    merged_regions[j].items.append(&mut regions[i].items);
                    merged_count += 1;
                    merged = true;
                }
            }

            if !merged {
                merged_regions.push(regions[i].clone());
            }
        }

        regions = merged_regions;
        if merged_count == 0 {
            break;
        }
    }

    // Update grid
    for (id, region) in regions.iter().enumerate() {
        for (x, y) in &region.items {
            input.grid[*y as usize][*x as usize].region = Some(id);
        }
    }

    println!("{regions:?}");
    let mut r = 0;
    for region in regions {
        let area = region.items.len();
        let mut perimeter = 0;

        for (x, y) in region.items {
            let region_id = input.grid[y as usize][x as usize].region.unwrap();

            for (dx, dy) in DIRS {
                let visit_x = x + dx;
                if visit_x < 0 || visit_x >= input.width {
                    perimeter += 1;
                    continue;
                }

                let visit_y = y + dy;
                if visit_y < 0 || visit_y >= input.height {
                    perimeter += 1;
                    continue;
                }

                let visit_region_id = input.grid[visit_y as usize][visit_x as usize]
                    .region
                    .unwrap();
                if region_id != visit_region_id {
                    perimeter += 1;
                }
            }
        }

        println!("{}: area: {area}, perimeter: {perimeter}", region.c);
        r += area * perimeter;
    }

    r
}

fn solve_part2(input: &str) -> usize {
    let mut input = parse_input(input);
    let mut regions: Vec<Region> = vec![];
    let mut region_next_id = 0;

    const DIRS: &[(i32, i32)] = &[(-1, 0), (1, 0), (0, -1), (0, 1)];

    for y in 0..input.height {
        for x in 0..input.width {
            let c = input.grid[y as usize][x as usize].c;

            for (dx, dy) in DIRS {
                let visit_x = x + dx;
                if visit_x < 0 || visit_x >= input.width {
                    continue;
                }

                let visit_y = y + dy;
                if visit_y < 0 || visit_y >= input.height {
                    continue;
                }

                let visit_c = input.grid[visit_y as usize][visit_x as usize].c;
                if c == visit_c {
                    let Some(id) = input.grid[visit_y as usize][visit_x as usize].region else {
                        continue;
                    };

                    //println!("{x}x{y} - Visit {visit_x}x{visit_y}. Existing region {id}");
                    input.grid[y as usize][x as usize].region = Some(id);
                    regions[id].items.push((x, y));
                    break;
                }
            }

            if input.grid[y as usize][x as usize].region.is_none() {
                let id = region_next_id;
                region_next_id += 1;

                //println!("{x}x{y}. New region {id}");
                input.grid[y as usize][x as usize].region = Some(id);

                let region = Region {
                    c,
                    items: vec![(x, y)],
                };
                regions.push(region);
            }
        }
    }

    // Merge regions
    loop {
        let mut merged_count = 0;
        let mut merged_regions = vec![regions[0].clone()];

        for i in 1..regions.len() {
            let mut merged = false;
            for j in 0..merged_regions.len() {
                if regions[i].c != merged_regions[j].c {
                    continue;
                }

                let mut merge = false;
                'outer: for (x, y) in &regions[i].items {
                    for (dx, dy) in DIRS {
                        if merged_regions[j].items.contains(&(x + dx, y + dy)) {
                            merge = true;
                            break 'outer;
                        }
                    }
                }

                if merge {
                    merged_regions[j].items.append(&mut regions[i].items);
                    merged_count += 1;
                    merged = true;
                }
            }

            if !merged {
                merged_regions.push(regions[i].clone());
            }
        }

        regions = merged_regions;
        if merged_count == 0 {
            break;
        }
    }

    // Update grid
    for (id, region) in regions.iter().enumerate() {
        for (x, y) in &region.items {
            input.grid[*y as usize][*x as usize].region = Some(id);
        }
    }

    println!("{regions:?}");
    let mut r = 0;
    for region in regions {
        let area = region.items.len();
        let mut horizontal_fences = vec![];
        let mut vertical_fences = vec![];

        for (x, y) in region.items {
            let region_id = input.grid[y as usize][x as usize].region.unwrap();

            // (x, y, horizontal)
            const DIRS: &[(i32, i32, bool)] =
                &[(-1, 0, false), (1, 0, false), (0, -1, true), (0, 1, true)];

            for (dx, dy, horizontal) in DIRS {
                let visit_x = x + dx;
                let visit_y = y + dy;
                if visit_x < 0 || visit_x >= input.width || visit_y < 0 || visit_y >= input.height {
                    if *horizontal {
                        horizontal_fences.push((visit_x, y * 3 + dy));
                    } else {
                        vertical_fences.push((x * 3 + dx, visit_y));
                    }

                    continue;
                }

                let visit_region_id = input.grid[visit_y as usize][visit_x as usize]
                    .region
                    .unwrap();
                if region_id != visit_region_id {
                    if *horizontal {
                        horizontal_fences.push((visit_x, y * 3 + dy));
                    } else {
                        vertical_fences.push((x * 3 + dx, visit_y));
                    }
                }
            }
        }

        horizontal_fences.sort_by(|(x1, y1), (x2, y2)| {
            use std::cmp::Ordering;

            match y1.cmp(y2) {
                Ordering::Greater => Ordering::Greater,
                Ordering::Less => Ordering::Less,
                Ordering::Equal => x1.cmp(&x2),
            }
        });

        vertical_fences.sort_by(|(x1, y1), (x2, y2)| {
            use std::cmp::Ordering;

            match x1.cmp(x2) {
                Ordering::Greater => Ordering::Greater,
                Ordering::Less => Ordering::Less,
                Ordering::Equal => y1.cmp(&y2),
            }
        });

        println!("horizontal_fences: {horizontal_fences:?}");
        println!("vertical_fences: {vertical_fences:?}");

        let mut sides = 0;

        let (mut prev_x, mut prev_y) = horizontal_fences[0];
        sides += 1;
        for i in 1..horizontal_fences.len() {
            let (x, y) = horizontal_fences[i];

            if prev_y == y {
                if prev_x + 1 != x {
                    sides += 1;
                }
            } else {
                sides += 1;
            }

            (prev_x, prev_y) = (x, y);
        }

        let (mut prev_x, mut prev_y) = vertical_fences[0];
        sides += 1;
        for i in 1..vertical_fences.len() {
            let (x, y) = vertical_fences[i];

            if prev_x == x {
                if prev_y + 1 != y {
                    sides += 1;
                }
            } else {
                sides += 1;
            }

            (prev_x, prev_y) = (x, y);
        }

        println!("{}: area: {area}, sides: {sides}", region.c);
        r += area * sides;
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
    fn day12() {
        const TEST_INPUT1: &str = "AAAA
BBCD
BBCC
EEEC";

        const TEST_INPUT2: &str = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";

        const TEST_INPUT3: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

        /*
        assert_eq!(solve_part1(TEST_INPUT1), 140);
        assert_eq!(solve_part1(TEST_INPUT2), 772);
        assert_eq!(solve_part1(TEST_INPUT3), 1930);
        assert_eq!(solve_part1(INPUT), 1396562);
        */

        const TEST_INPUT4: &str = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";

        const TEST_INPUT5: &str = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";

        /*
        assert_eq!(solve_part2(TEST_INPUT1), 80);
        assert_eq!(solve_part2(TEST_INPUT2), 436);
        assert_eq!(solve_part2(TEST_INPUT3), 1206);
        assert_eq!(solve_part2(TEST_INPUT4), 236);
        */
        assert_eq!(solve_part2(TEST_INPUT5), 368);

        //assert_eq!(solve_part2(INPUT), 233007586663131);
    }
}
