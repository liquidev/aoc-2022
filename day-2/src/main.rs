use std::str::FromStr;

use aoc::{
    anyhow::{self, anyhow},
    wrap_main, Challenge,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl FromStr for Move {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "A" | "X" => Move::Rock,
            "B" | "Y" => Move::Paper,
            "C" | "Z" => Move::Scissors,
            _ => Err(anyhow!("invalid move"))?,
        })
    }
}

impl Move {
    fn score(&self) -> usize {
        usize::from(*self as u8)
    }

    fn effective_against(&self) -> Move {
        match self {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        }
    }

    fn weak_against(&self) -> Move {
        self.effective_against().effective_against()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RoundOutcome {
    Loss,
    Draw,
    Win,
}

impl FromStr for RoundOutcome {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "X" => RoundOutcome::Loss,
            "Y" => RoundOutcome::Draw,
            "Z" => RoundOutcome::Win,
            _ => Err(anyhow!("invalid outcome"))?,
        })
    }
}

impl RoundOutcome {
    fn score(&self) -> usize {
        match self {
            RoundOutcome::Loss => 0,
            RoundOutcome::Draw => 3,
            RoundOutcome::Win => 6,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct AssumedRound {
    your_move: Move,
    opponents_move: Move,
}

impl AssumedRound {
    fn your_outcome(&self) -> RoundOutcome {
        match (self.your_move, self.opponents_move) {
            (yours, opponents) if yours == opponents => RoundOutcome::Draw,
            (yours, opponents) if yours.effective_against() == opponents => RoundOutcome::Win,
            _ => RoundOutcome::Loss,
        }
    }

    fn your_score(&self) -> usize {
        self.your_outcome().score() + self.your_move.score()
    }
}

fn parse_assumed_move_plan(input: &str) -> anyhow::Result<Vec<AssumedRound>> {
    let mut move_plan = vec![];
    for line in input.lines() {
        let mut moves = line.split_whitespace();
        let opponent = moves
            .next()
            .ok_or_else(|| anyhow!("missing opponent move"))?;
        let opponent = opponent.parse::<Move>()?;
        let counter = moves
            .next()
            .ok_or_else(|| anyhow!("missing counter move"))?;
        let counter = counter.parse::<Move>()?;
        move_plan.push(AssumedRound {
            opponents_move: opponent,
            your_move: counter,
        })
    }
    Ok(move_plan)
}

fn play_according_to_assumed_plan(plan: &[AssumedRound]) -> usize {
    plan.iter().map(|round| round.your_score()).sum()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ActualRound {
    opponents_move: Move,
    desired_outcome: RoundOutcome,
}

impl ActualRound {
    fn your_move(&self) -> Move {
        match self.desired_outcome {
            RoundOutcome::Win => self.opponents_move.weak_against(),
            RoundOutcome::Draw => self.opponents_move,
            RoundOutcome::Loss => self.opponents_move.effective_against(),
        }
    }

    fn your_score(&self) -> usize {
        AssumedRound {
            opponents_move: self.opponents_move,
            your_move: self.your_move(),
        }
        .your_score()
    }
}

fn parse_actual_move_plan(input: &str) -> anyhow::Result<Vec<ActualRound>> {
    let mut move_plan = vec![];
    for line in input.lines() {
        let mut moves = line.split_whitespace();
        let opponent = moves
            .next()
            .ok_or_else(|| anyhow!("missing opponent move"))?;
        let opponent = opponent.parse::<Move>()?;
        let outcome = moves
            .next()
            .ok_or_else(|| anyhow!("missing desired outcome"))?;
        let outcome = outcome.parse::<RoundOutcome>()?;
        move_plan.push(ActualRound {
            opponents_move: opponent,
            desired_outcome: outcome,
        })
    }
    Ok(move_plan)
}

fn play_according_to_actual_plan(plan: &[ActualRound]) -> usize {
    plan.iter().map(|round| round.your_score()).sum()
}

fn challenge_main(challenge: Challenge) -> anyhow::Result<()> {
    let move_plan = parse_assumed_move_plan(&challenge.input)?;
    let score = play_according_to_assumed_plan(&move_plan);
    println!("part 1 (assumed score): {score}");

    let move_plan = parse_actual_move_plan(&challenge.input)?;
    let score = play_according_to_actual_plan(&move_plan);
    println!("part 2 (actual score): {score}");

    Ok(())
}

fn main() {
    wrap_main(challenge_main)
}
