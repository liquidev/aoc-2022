use aoc::{
    anyhow::{self, anyhow},
    wrap_main, Challenge,
};

fn anyhow_main(challenge: Challenge) -> anyhow::Result<()> {
    Ok(())
}

fn main() {
    wrap_main(anyhow_main)
}
