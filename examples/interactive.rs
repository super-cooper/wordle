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
fn main() -> Result<(), WordleExampleError> {
    init_logging()?;
    let cli = CommonCli::parse();
    let list = cli.words();
    let answer = cli.answer();

    let game = list.map_or_else(|| Wordle::from_client(&cli.api()), Wordle::from_list)?;

    let mut round = answer.map_or_else(
        || game.round_date(&cli.api(), common::now()),
        |answer| game.round(answer),
    )?;

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
                Color::Gray => "30",
            };
            print!("\x1b[0;{ansi_code}m{letter}\x1b[0;0m ");
        }

        println!();
    }

    Ok(())
}
