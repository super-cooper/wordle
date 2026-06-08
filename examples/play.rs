//! Solve a [`Wordle`] puzzle algorithmically.
mod common;

use clap::Parser as _;
use common::{
    CommonCli,
    WordleExampleError,
    init_logging,
};
use wordle::Wordle;

/// Have the computer play through a full round of Wordle.
fn main() -> Result<(), WordleExampleError> {
    init_logging()?;
    let cli = CommonCli::parse();
    let list = cli.words();
    let answer = cli.answer();

    let game = list.map_or_else(|| Wordle::from_client(&cli.api()), Wordle::from_list)?;

    let play = match answer {
        Some(answer) => game.play(answer)?.into_iter().collect::<Vec<_>>(),
        None => game
            .play_date(&cli.api(), common::now())?
            .into_iter()
            .collect::<Vec<_>>(),
    };

    println!("{}", play.join("\n"));

    log::info!("Done");

    Ok(())
}
