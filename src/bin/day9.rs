use std::str::FromStr;

use anyhow::anyhow;

#[derive(Debug)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl FromStr for Direction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "L" => Self::Left,
            "U" => Self::Up,
            "R" => Self::Right,
            "D" => Self::Down,
            _ => {
                return Err(anyhow!(
                    "Invalid direction, expected one of L, U, R, D but found {s}"
                ))
            }
        })
    }
}

#[derive(Debug)]
struct Movement {
    direction: Direction,
    amount: u32,
}

impl FromStr for Movement {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, amount) = s
            .split_once(' ')
            .ok_or(anyhow!("No space separator in given string"))?;

        Ok(Self {
            direction: direction.parse()?,
            amount: amount.parse()?,
        })
    }
}

#[derive(Debug)]
struct MovementList {
    list: Vec<Movement>,
}

impl FromStr for MovementList {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let list = s.lines().flat_map(|l| l.parse()).collect();
        Ok(Self { list })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
struct Position {
    x: i64,
    y: i64,
}

#[derive(Debug)]
enum CatchUpMovement {
    None,
    One(Direction),
    Diagonal(Direction, Direction),
}

impl CatchUpMovement {
    fn from_head_tail(head: &Position, tail: &Position) -> Self {
        if head.x == tail.x {
            if head.y > tail.y + 1 {
                Self::One(Direction::Up)
            } else if head.y < tail.y - 1 {
                Self::One(Direction::Down)
            } else {
                Self::None
            }
        } else if head.y == tail.y {
            if head.x > tail.x + 1 {
                Self::One(Direction::Right)
            } else if head.x < tail.x - 1 {
                Self::One(Direction::Left)
            } else {
                Self::None
            }
        } else if head.x > tail.x + 1 && head.y >= tail.y + 1
            || head.x >= tail.x + 1 && head.y > tail.y + 1
        {
            Self::Diagonal(Direction::Up, Direction::Right)
        } else if head.x > tail.x + 1 && head.y <= tail.y - 1
            || head.x >= tail.x + 1 && head.y < tail.y - 1
        {
            Self::Diagonal(Direction::Down, Direction::Right)
        } else if head.x < tail.x - 1 && head.y >= tail.y + 1
            || head.x <= tail.x - 1 && head.y > tail.y + 1
        {
            Self::Diagonal(Direction::Up, Direction::Left)
        } else if head.x < tail.x - 1 && head.y <= tail.y - 1
            || head.x <= tail.x - 1 && head.y < tail.y - 1
        {
            Self::Diagonal(Direction::Down, Direction::Left)
        } else {
            Self::None
        }
    }

    fn apply(&self, tail: &mut Position) {
        match self {
            CatchUpMovement::None => {}
            CatchUpMovement::One(d) => tail.move_to_direction(d),
            CatchUpMovement::Diagonal(d1, d2) => {
                tail.move_to_direction(d1);
                tail.move_to_direction(d2);
            }
        }
    }
}

impl Position {
    fn move_to_direction(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
            Direction::Right => self.x += 1,
            Direction::Left => self.x -= 1,
        };
    }

    fn catch_up_with_head(&mut self, head: &Position) {
        CatchUpMovement::from_head_tail(head, self).apply(self)
    }

    fn move_with_tail(&mut self, tail: &mut Position, movement: &Movement) -> Vec<Position> {
        let mut history = vec![];

        for _ in 1..=movement.amount {
            self.move_to_direction(&movement.direction);
            tail.catch_up_with_head(self);
            history.push(*tail);
        }

        history
    }
    fn move_with_tails(&mut self, tails: &mut [Position], movement: &Movement) -> Vec<Position> {
        let mut history = vec![];

        for _ in 1..=movement.amount {
            self.move_to_direction(&movement.direction);
            let mut previous = *self;
            for i in 0..tails.len() {
                tails[i].catch_up_with_head(&previous);
                previous = tails[i];
            }
            history.push(tails[8]);
        }

        history
    }
}

impl From<(i64, i64)> for Position {
    fn from((x, y): (i64, i64)) -> Self {
        Self { x, y }
    }
}

fn part1(input: &MovementList) -> usize {
    let mut head = Position::from((0, 0));
    let mut tail = Position::from((0, 0));

    let mut history = vec![tail];
    for movement in &input.list {
        let mut new_history = head.move_with_tail(&mut tail, movement);
        history.append(&mut new_history);
    }

    history.sort_unstable();
    history.dedup();
    history.len()
}

fn part2(input: &MovementList) -> usize {
    let mut head = Position::from((0, 0));
    let mut tails = vec![Position::from((0, 0)); 9];

    let mut history = vec![tails[8]];
    for movement in &input.list {
        let mut new_history = head.move_with_tails(&mut tails, movement);
        history.append(&mut new_history);
    }

    history.sort_unstable();
    history.dedup();
    history.len()
}

fn main() {
    let input: MovementList = include_str!("day9.txt").parse().unwrap();

    println!("part 1 : {:?}", part1(&input));
    println!("part 2 : {:?}", part2(&input));
}
