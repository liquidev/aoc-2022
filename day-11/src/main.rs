use std::{mem, str::FromStr};

use aoc::{
    anyhow::{self, anyhow, bail, Context},
    wrap_main, Challenge,
};

type WorryLevel = u64;

#[derive(Debug, Clone, Copy)]
enum Value {
    Literal(WorryLevel),
    Old,
}

impl Value {
    fn eval(&self, old: WorryLevel) -> WorryLevel {
        match self {
            Value::Literal(x) => *x,
            Value::Old => old,
        }
    }
}

impl FromStr for Value {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "old" => Value::Old,
            _ => Value::Literal(s.parse().context("invalid value integer")?),
        })
    }
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add(Value, Value),
    Mul(Value, Value),
}

impl Operation {
    fn eval(&self, old: WorryLevel) -> WorryLevel {
        match self {
            Operation::Add(x, y) => x.eval(old) + y.eval(old),
            Operation::Mul(x, y) => x.eval(old) * y.eval(old),
        }
    }
}

impl FromStr for Operation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_whitespace();
        tokens.next(); // skip 'new'
        tokens.next(); // skip '='
        let lhs = tokens
            .next()
            .ok_or_else(|| anyhow!("missing left hand side"))?
            .parse()?;
        let operator = tokens.next().ok_or_else(|| anyhow!("missing operator"))?;
        let rhs = tokens
            .next()
            .ok_or_else(|| anyhow!("missing right hand side"))?
            .parse()?;
        Ok(match operator {
            "+" => Operation::Add(lhs, rhs),
            "*" => Operation::Mul(lhs, rhs),
            _ => bail!("invalid operator '{operator}'"),
        })
    }
}

#[derive(Debug, Clone)]
struct MonkeyDescriptor {
    starting_items: Vec<WorryLevel>,
    operation: Operation,
    test_divisible_by: WorryLevel,
    if_true_throw_to: usize,
    if_false_throw_to: usize,
}

impl MonkeyDescriptor {
    fn throw_to(&self, worry_level: WorryLevel) -> usize {
        if worry_level % self.test_divisible_by == 0 {
            self.if_true_throw_to
        } else {
            self.if_false_throw_to
        }
    }
}

impl FromStr for MonkeyDescriptor {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        lines.next(); // skip 'Monkey n:'

        let (_, starting_items) = lines
            .next()
            .ok_or_else(|| anyhow!("missing 'Starting items:' line"))?
            .split_once(": ")
            .ok_or_else(|| anyhow!("starting items line does not have ': ' to split on"))?;
        let starting_items = starting_items
            .split(", ")
            .filter_map(|s| s.parse().ok())
            .collect();

        let (_, operation) = lines
            .next()
            .ok_or_else(|| anyhow!("missing 'Operation:' line"))?
            .split_once(": ")
            .ok_or_else(|| anyhow!("operation line does not have ': ' to split on"))?;
        let operation = operation.parse()?;

        let test = lines
            .next()
            .ok_or_else(|| anyhow!("missing 'Test:' line"))?
            .split_whitespace()
            .last()
            .ok_or_else(|| anyhow!("'Test:' line is empty *somehow*"))?
            .parse()?;
        let if_true = lines
            .next()
            .ok_or_else(|| anyhow!("missing 'If true:' line"))?
            .split_whitespace()
            .last()
            .expect("'If true:' line is empty *somehow*")
            .parse()?;
        let if_false = lines
            .next()
            .ok_or_else(|| anyhow!("missing 'If false:' line"))?
            .split_whitespace()
            .last()
            .expect("'If false:' line is empty *somehow*")
            .parse()?;

        Ok(MonkeyDescriptor {
            starting_items,
            operation,
            test_divisible_by: test,
            if_true_throw_to: if_true,
            if_false_throw_to: if_false,
        })
    }
}

struct Monkey {
    items: Vec<WorryLevel>,
    inspection_count: usize,
}

impl std::fmt::Debug for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Monkey holding {:?} inspected an item {} times",
            self.items, self.inspection_count
        )
    }
}

struct KeepAway<'a> {
    descriptors: &'a [MonkeyDescriptor],
    monkeys: Vec<Monkey>,
}

#[derive(Debug, Clone, Copy)]
struct RoundOptions {
    relief_level: WorryLevel,
}

impl<'a> KeepAway<'a> {
    fn new(descriptors: &'a [MonkeyDescriptor]) -> Self {
        Self {
            descriptors,
            monkeys: descriptors
                .iter()
                .map(|descriptor| Monkey {
                    items: descriptor.starting_items.clone(),
                    inspection_count: 0,
                })
                .collect(),
        }
    }

    fn play_round(&mut self, RoundOptions { relief_level }: RoundOptions) {
        for monkey_index in 0..self.monkeys.len() {
            let items = mem::take(&mut self.monkeys[monkey_index].items);
            for old in items {
                let new = self.descriptors[monkey_index].operation.eval(old);
                self.monkeys[monkey_index].inspection_count += 1;
                let new = new / relief_level;
                let throw_to = self.descriptors[monkey_index].throw_to(new);
                self.monkeys[throw_to].items.push(new);
            }
        }
    }

    fn monkey_business(mut self) -> usize {
        self.monkeys
            .sort_unstable_by_key(|monkey| monkey.inspection_count);
        let mut top_2 = self.monkeys.iter().rev().take(2);
        let first = top_2
            .next()
            .map(|monkey| monkey.inspection_count)
            .unwrap_or(0);
        let second = top_2
            .next()
            .map(|monkey| monkey.inspection_count)
            .unwrap_or(0);
        first * second
    }
}

impl<'a> std::fmt::Debug for KeepAway<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KeepAway")
            .field("monkeys", &self.monkeys)
            .finish_non_exhaustive()
    }
}

fn play_the_game(
    challenge: &Challenge,
    descriptors: &[MonkeyDescriptor],
    round_options: RoundOptions,
    round_count: usize,
) -> usize {
    let mut game = KeepAway::new(descriptors);
    for i in 1..=round_count {
        game.play_round(round_options);
        if challenge.debug_flags.contains("rounds") {
            println!("round {i}: {game:#?}");
        }
    }
    game.monkey_business()
}

fn challenge_main(challenge: Challenge) -> anyhow::Result<()> {
    let mut descriptors = vec![];
    for (i, block) in challenge.input.split("\n\n").enumerate() {
        descriptors.push(
            block
                .parse::<MonkeyDescriptor>()
                .with_context(|| format!("cannot parse monkey descriptor block {i}"))?,
        )
    }

    if challenge.debug_flags.contains("descriptors") {
        dbg!(&descriptors);
    }

    let part_1 = play_the_game(
        &challenge,
        &descriptors,
        RoundOptions { relief_level: 3 },
        20,
    );
    println!("part 1: {part_1}");

    let part_2 = play_the_game(
        &challenge,
        &descriptors,
        RoundOptions { relief_level: 1 },
        10000,
    );
    println!("part 2: {part_2}");

    Ok(())
}

fn main() {
    wrap_main(challenge_main)
}
