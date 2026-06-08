//! Get the highest-value words from a word list.
mod common;

use std::fmt::Write;

use clap::{
    Args,
    Parser,
};
use common::{
    CommonCli,
    WordleExampleError,
    init_logging,
};
use wordle::{
    ScoreMode,
    Wordle,
};

#[derive(Args, Debug)]
struct TopArgs {
    // The number of top words to display
    #[arg(short, default_value_t = 5)]
    n: usize,
}

#[derive(Parser, Debug)]
#[command(about)]
struct Cli {
    #[command(flatten)]
    common: CommonCli,
    #[command(flatten)]
    top:    TopArgs,
}

/// Get the top N words from a word list.
fn main() -> Result<(), WordleExampleError> {
    init_logging()?;
    let cli = Cli::parse();
    let list = cli.common.words();

    let game = match list {
        Some(list) => Wordle::from_list(list),
        None => Wordle::from_client(&cli.common.api()),
    }?;

    let round = game.round(cli.common.answer().unwrap_or_else(|| "ABCDE".to_string()))?;

    let top_words = round
        .best(ScoreMode::UniqueOnly { n: cli.top.n })
        .into_iter()
        .try_fold(
            String::new(),
            |mut s, score| -> Result<String, std::fmt::Error> {
                writeln!(s, "{score}")?;
                Ok(s)
            },
        )?;

    print!("{top_words}");

    log::info!("Done");

    Ok(())
}
