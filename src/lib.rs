pub use anyhow;
pub use log;

use std::path::PathBuf;

use anyhow::Context;
use clap::Parser;
use log::error;

#[derive(Parser)]
struct ChallengeArgs {
    input_file: PathBuf,
}

pub struct Challenge {
    pub input: String,
}

fn load_challenge() -> anyhow::Result<Challenge> {
    let args = ChallengeArgs::parse();
    let input = std::fs::read_to_string(&args.input_file)
        .context("read input file")?
        .replace("\r\n", "\n");
    Ok(Challenge { input })
}

pub fn wrap_main(f: impl FnOnce(Challenge) -> anyhow::Result<()>) {
    env_logger::builder().format_timestamp(None).init();

    match load_challenge().and_then(f) {
        Ok(()) => (),
        Err(error) => {
            error!("{error:?}");
            std::process::exit(1);
        }
    }
}
