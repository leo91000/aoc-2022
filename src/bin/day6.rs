use std::collections::HashSet;

fn first_marker(input: &str, len: usize) -> Option<usize> {
    let chars = input.chars();
    for (i, _) in chars.enumerate() {
        input.chars().nth(i + len - 1)?;
        let slice = input[i..=i + len - 1].chars().collect::<HashSet<_>>();
        if slice.len() == len {
            return Some(i + len);
        }
    }
    unreachable!();
}

fn part1(input: &str) -> Option<usize> {
    first_marker(input, 4)
}

fn part2(input: &str) -> Option<usize> {
    first_marker(input, 14)
}

fn main() {
    let input = include_str!("day6.txt").trim();
    println!("{:?}", part1(input));
    println!("{:?}", part2(input));
}
