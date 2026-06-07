//! Play [`Wordle`] in an interactive REPL.
mod common;

use std::io::{
    self,
    Write,
};

use clap::Parser as _;
use common::{
    CommonCli,
    WordleExampleError,
    init_logging,
};
use wordle::{
    Color,
    Wordle,
};

/// Play Wordle interactively in a REPL.
/// Usage: interactive [OPTIONS]
///
/// Options:
/// --words <WORDS>                   Optional comma-separated list of words
/// --wordle-api-url <WORDLE_API_URL> The API for checking the Wordle answer if one is not provided
/// --word-list-url <WORD_LIST_URL>   The URL used to fetch the newline-separated list of words
///
/// --answer <ANSWER>                 An optional custom answer
fn main() -> Result<(), WordleExampleError> {
    init_logging();
    let cli = CommonCli::parse();
    let list = cli.words();
    let answer = cli.answer();

    let game = match list {
        Some(list) => Wordle::from_list(list),
        None => Wordle::from_client(&cli.api()),
    }?;

    let mut round = match answer {
        Some(answer) => game.round(answer),
        None => game.round_date(&cli.api(), common::today()),
    }?;

    while !round.is_over() {
        print!("Guess: ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let colors = match round.guess(&input) {
            Ok(colors) => colors,
            Err(err) => {
                println!("Oops: {err}");
                continue;
            },
        };

        for (letter, color) in input.chars().map(char::to_uppercase).zip(colors) {
            let ansi_code = match color {
                Color::Green => "32",
                Color::Yellow => "33",
                _ => "30",
            };
            print!("\x1b[0;{ansi_code}m{letter}\x1b[0;0m ");
        }

        println!();
    }

    Ok(())
}
