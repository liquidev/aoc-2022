use std::ops::RangeInclusive;

use aoc::{
    anyhow::{self, anyhow},
    wrap_main, Challenge,
};

fn split_elf_pair(input: &str) -> anyhow::Result<(&str, &str)> {
    input
        .split_once(',')
        .ok_or_else(|| anyhow!("line does not have a pair of elves: {input}"))
}

fn parse_range(input: &str) -> anyhow::Result<RangeInclusive<usize>> {
    let (lo, hi) = input
        .split_once('-')
        .ok_or_else(|| anyhow!("not a valid pair of numbers: {input}"))?;
    Ok(lo.parse()?..=hi.parse()?)
}

fn fully_overlaps(a: &RangeInclusive<usize>, b: &RangeInclusive<usize>) -> bool {
    (a.start() >= b.start() && a.end() <= b.end()) || (b.start() >= a.start() && b.end() <= a.end())
}

fn partially_overlaps(a: &RangeInclusive<usize>, b: &RangeInclusive<usize>) -> bool {
    a.end() >= b.start() && b.end() >= a.start()
}

fn challenge_main(challenge: Challenge) -> anyhow::Result<()> {
    let mut fully_overlapping = 0;
    let mut partially_overlapping = 0;
    for line in challenge.input.lines() {
        let (first, second) = split_elf_pair(line)?;
        let (first, second) = (parse_range(first)?, parse_range(second)?);
        fully_overlapping += fully_overlaps(&first, &second) as usize;
        partially_overlapping += partially_overlaps(&first, &second) as usize;
    }
    println!("part 1: {fully_overlapping}");
    println!("part 2: {partially_overlapping}");

    Ok(())
}

fn main() {
    wrap_main(challenge_main)
}
