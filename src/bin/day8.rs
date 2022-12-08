fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_string().parse().expect("Invalid char"))
                .collect()
        })
        .collect()
}

fn is_visible_up(input: &Vec<Vec<u8>>, height: Option<u8>, x: usize, y: usize) -> bool {
    let current = input[y][x];
    if let Some(h) = height {
        if current >= h {
            return false;
        }
    }

    if y == 0 {
        return true;
    }

    is_visible_up(input, Some(height.unwrap_or(current)), x, y - 1)
}

fn is_visible_down(input: &Vec<Vec<u8>>, height: Option<u8>, x: usize, y: usize) -> bool {
    let current = input[y][x];
    if let Some(h) = height {
        if current >= h {
            return false;
        }
    }

    if y == input.len() - 1 {
        return true;
    }

    is_visible_down(input, Some(height.unwrap_or(current)), x, y + 1)
}

fn is_visible_left(input: &Vec<Vec<u8>>, height: Option<u8>, x: usize, y: usize) -> bool {
    let current = input[y][x];
    if let Some(h) = height {
        if current >= h {
            return false;
        }
    }

    if x == 0 {
        return true;
    }

    is_visible_left(input, Some(height.unwrap_or(current)), x - 1, y)
}

fn is_visible_right(input: &Vec<Vec<u8>>, height: Option<u8>, x: usize, y: usize) -> bool {
    let current = input[y][x];
    if let Some(h) = height {
        if current >= h {
            return false;
        }
    }

    if x == input[0].len() - 1 {
        return true;
    }

    is_visible_right(input, Some(height.unwrap_or(current)), x + 1, y)
}

fn is_tree_visible(input: &Vec<Vec<u8>>, x: usize, y: usize) -> bool {
    is_visible_up(input, None, x, y)
        || is_visible_down(input, None, x, y)
        || is_visible_left(input, None, x, y)
        || is_visible_right(input, None, x, y)
}

fn part1(input: &Vec<Vec<u8>>) -> u32 {
    let mut count = 0_u32;
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if is_tree_visible(input, x, y) {
                count += 1;
            }
        }
    }
    count
}

enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn move_coordinates(&self, (mut x, mut y): (usize, usize)) -> (usize, usize) {
        match self {
            Direction::North => y -= 1,
            Direction::South => y += 1,
            Direction::West => x -= 1,
            Direction::East => x += 1,
        };

        (x, y)
    }

    fn is_edge(&self, input: &Vec<Vec<u8>>, (x, y): (usize, usize)) -> bool {
        match self {
            Direction::North => y == 0,
            Direction::South => y == input.len() - 1,
            Direction::West => x == 0,
            Direction::East => x == input[0].len() - 1,
        }
    }
}

fn get_view_distance_impl(
    input: &Vec<Vec<u8>>,
    dir: Direction,
    h: Option<u8>,
    x: usize,
    y: usize,
    vd: &mut u32,
) {
    let current = input[y][x];
    if let Some(h) = h {
        if current >= h {
            return;
        }
    }

    if dir.is_edge(input, (x, y)) {
        return;
    }

    *vd += 1;
    let (x, y) = dir.move_coordinates((x, y));
    get_view_distance_impl(input, dir, Some(h.unwrap_or(current)), x, y, vd)
}

fn get_view_distance(input: &Vec<Vec<u8>>, dir: Direction, (x, y): (usize, usize)) -> u32 {
    let mut vd = 0_u32;
    get_view_distance_impl(input, dir, None, x, y, &mut vd);
    vd
}

fn part2(input: &Vec<Vec<u8>>) -> u32 {
    let mut max_score = 0_u32;
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            let n = get_view_distance(input, Direction::North, (x, y));
            let s = get_view_distance(input, Direction::South, (x, y));
            let w = get_view_distance(input, Direction::West, (x, y));
            let e = get_view_distance(input, Direction::East, (x, y));

            let score = n * s * w * e;
            if score > max_score {
                max_score = score;
            }
        }
    }

    max_score
}

fn main() {
    let input = include_str!("day8.txt");
    let input = parse_input(input);

    println!("part 1 : {}", part1(&input));
    println!("part 2 : {}", part2(&input));
}
