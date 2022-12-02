enum MatchResult {
    Win,
    Draw,
    Lost,
}

impl MatchResult {
    pub fn from_expected_result(my_move: u32) -> MatchResult {
        match my_move {
            0 => Self::Lost,
            1 => Self::Draw,
            _ => Self::Win,
        }
    }

    pub fn get_move_for_result(&self, opponent_move: u32) -> u32 {
        match self {
            MatchResult::Win => {
                if opponent_move == 0 || opponent_move == 1 {
                    opponent_move + 1
                } else {
                    0
                }
            }
            MatchResult::Draw => opponent_move,
            MatchResult::Lost => {
                if opponent_move == 1 || opponent_move == 2 {
                    opponent_move - 1
                } else {
                    2
                }
            }
        }
    }

    pub fn from_match(my_move: u32, opponent_move: u32) -> MatchResult {
        if my_move == opponent_move {
            return Self::Draw;
        } else if my_move == 0 && opponent_move == 1
            || my_move == 1 && opponent_move == 2
            || my_move == 2 && opponent_move == 0
        {
            return Self::Lost;
        }
        Self::Win
    }

    pub fn points(&self) -> u32 {
        match self {
            MatchResult::Win => 6,
            MatchResult::Draw => 3,
            MatchResult::Lost => 0,
        }
    }
}

fn main() {
    let data = include_str!("day2.txt");

    let moves: u32 = data
        .lines()
        .map(|round| {
            let chars = round.chars().collect::<Vec<_>>();
            let (opponent_move, my_move) =
                (chars[0] as u32 - 'A' as u32, chars[2] as u32 - 'X' as u32);
            let my_move =
                MatchResult::from_expected_result(my_move).get_move_for_result(opponent_move);
            MatchResult::from_match(my_move, opponent_move).points() + my_move + 1
        })
        .sum();

    dbg!(moves);
}
