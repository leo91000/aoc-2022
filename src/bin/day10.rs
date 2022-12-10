use std::{fmt::Display, str::FromStr};

use anyhow::anyhow;

#[derive(Debug)]
enum Command {
    Noop,
    Addx(i32),
}

impl Command {
    fn get_cycle_cost(&self) -> u32 {
        match self {
            Command::Noop => 1,
            Command::Addx(_) => 2,
        }
    }

    fn apply_to(&self, x: &mut i32) {
        if let Command::Addx(amount) = self {
            *x += amount
        }
    }
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("noop") {
            Ok(Command::Noop)
        } else if s.starts_with("addx") {
            let (_, amount) = s.split_once(' ').ok_or(anyhow!(
                "No space separator for addx command. Can't find amount to addx"
            ))?;
            let amount = amount
                .parse()
                .map_err(|e| anyhow!("Invalid digit {amount} : {e:?}"))?;

            Ok(Self::Addx(amount))
        } else {
            Err(anyhow!("Unknown command {s}"))
        }
    }
}

#[derive(Debug)]
struct InstructionSet {
    commands: Vec<Command>,
}

const BOARD_LINES: u32 = 6;
const LINE_LENGTH: u32 = 40;

impl InstructionSet {
    fn get_x_at_cycle(&self, cycle: u32) -> i32 {
        let mut x = 1;
        let mut current_cycle = 0_u32;
        let mut i = 0;

        loop {
            let command = &self.commands[i];
            current_cycle += command.get_cycle_cost();

            if current_cycle >= cycle {
                break;
            }

            command.apply_to(&mut x);
            i += 1;
        }

        x
    }

    fn get_signal_at_cycle(&self, cycle: u32) -> i32 {
        self.get_x_at_cycle(cycle) * cycle as i32
    }

    fn draw_board(&self) -> Board {
        let mut board = vec![];

        for i in 0..BOARD_LINES {
            let mut line = vec![];

            for j in 1..=LINE_LENGTH {
                let cycle = j + LINE_LENGTH * i;
                let x = self.get_x_at_cycle(cycle) as i64;
                let j = j as i64 - 1;

                let pixel = if j == x - 1 || j == x || j == x + 1 {
                    Pixel::Hash
                } else {
                    Pixel::Dot
                };

                line.push(pixel);
            }

            board.push(line);
        }

        Board { board }
    }
}

impl FromStr for InstructionSet {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let commands = s.lines().map(|l| l.parse()).collect::<Result<_, _>>()?;
        Ok(InstructionSet { commands })
    }
}

enum Pixel {
    Dot,
    Hash,
}

impl Display for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Dot => write!(f, "."),
            Self::Hash => write!(f, "#"),
        }
    }
}

struct Board {
    board: Vec<Vec<Pixel>>,
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.board {
            for pixel in line {
                write!(f, "{pixel}")?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

fn part1(instructions: &InstructionSet) -> i32 {
    instructions.get_signal_at_cycle(20)
        + instructions.get_signal_at_cycle(60)
        + instructions.get_signal_at_cycle(100)
        + instructions.get_signal_at_cycle(140)
        + instructions.get_signal_at_cycle(180)
        + instructions.get_signal_at_cycle(220)
}

fn part2(instructions: &InstructionSet) -> Board {
    instructions.draw_board()
}

fn main() {
    let input = include_str!("day10.txt");

    let instructions: InstructionSet = input.parse().expect("Failed to parse input");

    println!("part1 : {}", part1(&instructions));
    println!("part2 : \n{}", part2(&instructions));
}
