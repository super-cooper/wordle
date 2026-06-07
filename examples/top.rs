//! Get the highest-value words from a word list.
mod common;

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
///
/// Usage: top [OPTIONS]
///
/// Options:
/// --words <WORDS>                   Optional comma-separated list of words
/// --wordle-api-url <WORDLE_API_URL> The API for checking the Wordle answer if one is not provided
/// --word-list-url <WORD_LIST_URL>   The URL used to fetch the newline-separated list of words
///                                   if a list is not provided via [WORDS]
/// --answer <ANSWER>                 An optional custom answer
fn main() -> Result<(), WordleExampleError> {
    init_logging();
    let cli = Cli::parse();
    let list = cli.common.words();

    let game = match list {
        Some(list) => Wordle::from_list(list),
        None => Wordle::from_client(&cli.common.api()),
    }?;

    let round = game.round(cli.common.answer().unwrap_or("ABCDE".to_string()))?;

    let top_words = round
        .best(ScoreMode::UniqueOnly { n: cli.top.n })
        .into_iter()
        .map(|score| format!("{score}\n"))
        .collect::<String>();

    print!("{top_words}");

    Ok(())
}
