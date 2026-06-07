//! Solve a [`Wordle`] puzzle algorithmically.
mod common;

use clap::Parser as _;
use common::{
    CommonCli,
    WordleExampleError,
    init_logging,
};
use wordle::Wordle;

// Have the computer play through a full round of Wordle.
//
// Usage: play [OPTIONS]

// Options:
// --words          <WORDS>
//
// --wordle-api-url <WORDLE_API_URL>
// --word-list-url  <WORD_LIST_URL>
// --answer         <ANSWER>
fn main() -> Result<(), WordleExampleError> {
    init_logging();
    let cli = CommonCli::parse();
    let list = cli.words();
    let answer = cli.answer();

    let game = match list {
        Some(list) => Wordle::from_list(list),
        None => Wordle::from_client(&cli.api()),
    }?;

    let play = match answer {
        Some(answer) => game.play(answer)?.into_iter().collect::<Vec<_>>(),
        None => game
            .play_date(&cli.api(), common::today())?
            .into_iter()
            .collect::<Vec<_>>(),
    };

    println!("{}", play.join("\n"));

    Ok(())
}
