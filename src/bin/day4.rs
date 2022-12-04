fn parse_number(n: &str) -> Option<u32> {
    n.parse().ok()
}

fn parse_range(range: &str) -> Option<Vec<u32>> {
    range
        .split_once('-')
        .and_then(|(low, high)| parse_number(low).zip(parse_number(high)))
        .map(|(low, high)| (low..=high).collect())
}

fn vec_self_contained(v1: &[u32], v2: &[u32]) -> bool {
    v1.iter().all(|item| v2.contains(item)) || v2.iter().all(|item| v1.contains(item))
}

fn vec_overlap(v1: &[u32], v2: &[u32]) -> bool {
    v1.iter().any(|item| v2.contains(item)) || v2.iter().any(|item| v1.contains(item))
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .flat_map(|line| line.split_once(','))
        .flat_map(|(e1, e2)| parse_range(e1).zip(parse_range(e2)))
        .filter(|(r1, r2)| vec_self_contained(r1, r2))
        .count()
}
fn part2(input: &str) -> usize {
    input
        .lines()
        .flat_map(|line| line.split_once(','))
        .flat_map(|(e1, e2)| parse_range(e1).zip(parse_range(e2)))
        .filter(|(r1, r2)| vec_overlap(r1, r2))
        .count()
}

fn main() {
    let input = include_str!("day4.txt");
    println!("part1 : {:?}", part1(input));
    println!("part2 : {:?}", part2(input));
}
