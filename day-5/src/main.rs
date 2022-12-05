use aoc::{
    anyhow::{self, anyhow},
    wrap_main, Challenge,
};

fn parse_stacks(stacks: &str) -> Vec<Vec<char>> {
    stacks.lines().map(|line| line.chars().collect()).collect()
}

#[derive(Debug)]
struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

fn parse_instructions(instructions: &str) -> anyhow::Result<Vec<Instruction>> {
    let mut result = vec![];
    for line in instructions.lines() {
        let mut words = line.split_whitespace();
        words.next().ok_or_else(|| anyhow!("missing 'move' word"))?; // 'move'
        let count = words
            .next()
            .ok_or_else(|| anyhow!("missing count"))?
            .parse()?;
        words.next().ok_or_else(|| anyhow!("missing 'from' word"))?; // 'from'
        let from = words
            .next()
            .ok_or_else(|| anyhow!("missing from-index"))?
            .parse::<usize>()?;
        words.next().ok_or_else(|| anyhow!("missing 'to' word"))?; // 'to'
        let to = words
            .next()
            .ok_or_else(|| anyhow!("missing from-index"))?
            .parse::<usize>()?;
        assert!(
            words.next().is_none(),
            "too many words on this line: {line}"
        );

        let (from, to) = (from - 1, to - 1);
        result.push(Instruction { count, from, to })
    }
    Ok(result)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Crane {
    CrateMover9000,
    CrateMover9001,
}

impl Crane {
    fn run_instructions(&self, mut stacks: Vec<Vec<char>>, instructions: &[Instruction]) -> String {
        let mut result = String::new();
        let mut temp = vec![];

        for instruction in instructions {
            let from_stack = &mut stacks[instruction.from];
            temp.extend(from_stack.drain(from_stack.len() - instruction.count..));
            if let Crane::CrateMover9000 = self {
                temp.reverse();
            }
            stacks[instruction.to].append(&mut temp);
        }

        result.extend(
            stacks
                .iter()
                .map(|stack| stack.last().copied().unwrap_or('!')),
        );
        result
    }
}

fn anyhow_main(challenge: Challenge) -> anyhow::Result<()> {
    let (stacks, instructions) = challenge.input.split_once("\n\n").ok_or_else(|| {
        anyhow!("input must be structured like: [initial stack]\\n\\n[instructions]")
    })?;

    let stacks = parse_stacks(stacks);
    let instructions = parse_instructions(instructions)?;

    let part_1 = Crane::CrateMover9000.run_instructions(stacks.clone(), &instructions);
    println!("part 1: {part_1}");
    let part_2 = Crane::CrateMover9001.run_instructions(stacks, &instructions);
    println!("part 2: {part_2}");

    Ok(())
}

fn main() {
    wrap_main(anyhow_main)
}
