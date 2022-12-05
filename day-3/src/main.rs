use std::collections::HashSet;

use aoc::{
    anyhow::{self, anyhow},
    wrap_main, Challenge,
};

fn item_types_in_compartment(compartment: &str) -> HashSet<char> {
    compartment.chars().collect()
}

fn item_priority(item: char) -> Option<usize> {
    match item {
        'a'..='z' => Some(item as usize - b'a' as usize + 1),
        'A'..='Z' => Some(item as usize - b'A' as usize + 27),
        _ => None,
    }
}

fn part_1(challenge: &Challenge) -> anyhow::Result<usize> {
    let mut sum = 0_usize;
    for line in challenge.input.lines() {
        let half = line.len() / 2;
        let (left, right) = line.split_at(half);

        let left = item_types_in_compartment(left);
        let right = item_types_in_compartment(right);

        let mut intersection = left.intersection(&right);
        let repeating = *intersection
            .next()
            .ok_or_else(|| anyhow!("there are no repeating items in this rucksack: {line}"))?;
        assert!(
            intersection.next().is_none(),
            "there should only be one repeating item per rucksack"
        );

        sum += item_priority(repeating).unwrap_or(0);
    }
    Ok(sum)
}

fn part_2(challenge: &Challenge) -> anyhow::Result<usize> {
    let lines = challenge.input.lines().collect::<Vec<_>>();
    let mut sum = 0;
    for three in lines.chunks(3) {
        assert!(
            three.len() == 3,
            "stray elves found (not part of a group of three)"
        );
        let item_types = three
            .iter()
            .map(|rucksack| item_types_in_compartment(rucksack))
            .collect::<Vec<_>>();
        if let &[ref first, ref second, ref third] = &item_types[..] {
            let first_two = first.intersection(second).copied().collect::<HashSet<_>>();
            let mut all = first_two.intersection(third);
            let badge = *all
                .next()
                .ok_or_else(|| anyhow!("did not find badge for this three: {three:?}"))?;
            assert!(
                all.next().is_none(),
                "more than one badge found for this three: {three:?}",
            );
            sum += item_priority(badge).unwrap_or(0);
        } else {
            unreachable!()
        }
    }

    Ok(sum)
}

fn anyhow_main(challenge: Challenge) -> anyhow::Result<()> {
    let part_1 = part_1(&challenge)?;
    let part_2 = part_2(&challenge)?;

    println!("part 1: {part_1}");
    println!("part 2: {part_2}");

    Ok(())
}

fn main() {
    wrap_main(anyhow_main)
}
