pub use anyhow;
pub use log;

use std::{collections::HashSet, path::PathBuf};

use anyhow::Context;
use clap::Parser;
use log::{error, info, LevelFilter};

#[derive(Parser)]
struct ChallengeArgs {
    input_files: Vec<PathBuf>,
    #[clap(long)]
    debug: Vec<String>,
}

pub struct Challenge {
    pub input: String,
    pub debug_flags: HashSet<String>,
}

struct LoadedChallenge {
    filename: PathBuf,
    inner: Challenge,
}

fn load_challenges() -> anyhow::Result<Vec<LoadedChallenge>> {
    let args = ChallengeArgs::parse();
    let mut challenges = vec![];
    let debug_flags: HashSet<String> = args.debug.iter().cloned().collect();
    for filename in args.input_files {
        let input = std::fs::read_to_string(&filename)
            .context("read input file")?
            .replace("\r\n", "\n");
        challenges.push(LoadedChallenge {
            filename,
            inner: Challenge {
                input,
                debug_flags: debug_flags.clone(),
            },
        });
    }
    Ok(challenges)
}

fn run_challenges(mut f: impl FnMut(Challenge) -> anyhow::Result<()>) -> anyhow::Result<()> {
    let challenges = load_challenges().context("cannot load challenges")?;
    for (i, challenge) in challenges.into_iter().enumerate() {
        info!("file #{}: {}", i + 1, challenge.filename.to_string_lossy());
        f(challenge.inner)
            .with_context(|| format!("file #{} {:?} failed", i + 1, challenge.filename))?;
    }
    Ok(())
}

pub fn wrap_main(f: impl FnMut(Challenge) -> anyhow::Result<()>) {
    env_logger::builder()
        .format_timestamp(None)
        .filter_module("aoc", LevelFilter::Debug)
        .init();

    match run_challenges(f) {
        Ok(()) => (),
        Err(error) => {
            error!("{error:?}");
            std::process::exit(1);
        }
    }
}
