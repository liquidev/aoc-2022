use std::collections::HashSet;

use aoc::{
    anyhow::{self, anyhow},
    wrap_main, Challenge,
};

fn find_first_byte_after_marker(input: &[u8], marker_size: usize) -> anyhow::Result<usize> {
    Ok(input
        .windows(marker_size)
        .position(|window| window.iter().copied().collect::<HashSet<_>>().len() == marker_size)
        .ok_or_else(|| anyhow!("no marker packet found"))?
        + marker_size)
}

fn challenge_main(challenge: Challenge) -> anyhow::Result<()> {
    let input = challenge.input.as_bytes();
    let start_of_packet = find_first_byte_after_marker(input, 4)?;
    println!("part 1: {start_of_packet}");
    let start_of_message = find_first_byte_after_marker(input, 14)?;
    println!("part 2: {start_of_message}");

    Ok(())
}

fn main() {
    wrap_main(challenge_main)
}
