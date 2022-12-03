#![feature(iter_array_chunks)]

fn unique_items(sect: &str) -> u64 {
    sect.bytes()
        .map(|c| match c {
            b'a'..=b'z' => 1 + c - b'a',
            b'A'..=b'Z' => 27 + c - b'A',
            _ => unreachable!(),
        })
        .fold(0u64, |acc, n| acc | (1 << n))
}

pub fn one(bags: &str) -> u32 {
    bags.lines()
        .map(|bag| bag.split_at(bag.len() / 2))
        .map(|(l, r)| [l, r].map(unique_items))
        .map(|[l, r]| u64::trailing_zeros(l & r))
        .sum()
}

pub fn two(bags: &str) -> u32 {
    bags.lines()
        .array_chunks::<3>() // unstable
        .map(|bags| bags.map(unique_items))
        .map(|[a, b, c]| a & b & c)
        .map(u64::trailing_zeros)
        .sum()
}

fn main() {
    let input = include_str!("day3.txt");
    println!("1: {}", one(input));
    println!("2: {}", two(input));
}
