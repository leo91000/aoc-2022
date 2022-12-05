fn parse_cargo(cargo: &[&str]) -> Vec<Vec<char>> {
    let cargo_lines: Vec<Vec<Option<char>>> = cargo
        .iter()
        .map(|c| {
            c.chars()
                .enumerate()
                .filter(|(i, _)| i % 4 == 1)
                .map(|(_, c)| if c == ' ' { None } else { Some(c) })
                .collect::<Vec<_>>()
        })
        .collect();

    let max_cargo_size = cargo_lines.iter().map(|c| c.len()).max().unwrap();
    let mut cargos: Vec<Vec<char>> = vec![vec![]; max_cargo_size];

    for cargo_line in cargo_lines {
        for (index, value) in cargo_line.iter().enumerate() {
            let mut cargo_n = cargos.get(index).cloned().unwrap_or(vec![]);
            if let Some(v) = value {
                cargo_n.insert(0, *v);
            }
            cargos[index] = cargo_n;
        }
    }

    cargos
}

#[derive(Debug, Clone)]
struct Move {
    length: usize,
    source: usize,
    target: usize,
}

fn parse_moves(moves: &[&str]) -> Vec<Move> {
    let mut target: Vec<Move> = vec![];

    for m in moves {
        let split = m.split(' ').collect::<Vec<_>>();
        target.push(Move {
            length: split[1].parse().unwrap(),
            source: split[3].parse().unwrap(),
            target: split[5].parse().unwrap(),
        })
    }

    target
}

fn apply_move(mut cargos: Vec<Vec<char>>, moves: Vec<Move>) -> Vec<Vec<char>> {
    for m in moves {
        for _ in 0..m.length {
            let el = cargos[m.source - 1].pop();
            if let Some(e) = el {
                cargos[m.target - 1].push(e);
            }
        }
    }

    cargos
}

fn part1() {
    let (cargo, moves) = include_str!("day5.txt").split_once("\n\n").expect("failed");

    let cargo = cargo.lines().collect::<Vec<_>>();
    let cargo = &cargo[0..(cargo.len() - 1)];
    let cargo = parse_cargo(cargo);

    let moves = moves.lines().collect::<Vec<_>>();
    let moves = parse_moves(&moves);

    let cargo = apply_move(cargo, moves);

    println!(
        "part1 : {}",
        cargo
            .iter()
            .map(|c| c.clone().pop().unwrap_or('!'))
            .collect::<String>()
    );
}
fn apply_move_2(mut cargos: Vec<Vec<char>>, moves: Vec<Move>) -> Vec<Vec<char>> {
    for m in moves {
        let source = &mut cargos[m.source - 1];
        let mut package: Vec<char> = vec![];

        for _ in 0..m.length {
            if let Some(el) = source.pop() {
                package.insert(0, el);
            }
        }

        for e in package {
            cargos[m.target - 1].push(e);
        }
    }

    cargos
}

fn part2() {
    let (cargo, moves) = include_str!("day5.txt").split_once("\n\n").expect("failed");

    let cargo = cargo.lines().collect::<Vec<_>>();
    let cargo = &cargo[0..(cargo.len() - 1)];
    let cargo = parse_cargo(cargo);

    let moves = moves.lines().collect::<Vec<_>>();
    let moves = parse_moves(&moves);

    let cargo = apply_move_2(cargo, moves);

    println!(
        "part2 : {}",
        cargo
            .iter()
            .map(|c| c.clone().pop().unwrap_or('!'))
            .collect::<String>()
    );
}

fn main() {
    part1();
    part2();
}
