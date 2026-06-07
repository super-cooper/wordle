//! A library for solving and playing [Wordle](https://www.nytimes.com/games/wordle/index.html). The
//! entrypoint is the [`Wordle`] struct, which provides simple APIs for custom Wordle games, or the
//! official Wordle game, whose word list and answers can be fetched upstream via a [`Client`]
//! implementation.
//!
//! This crate mimics the mechanics and rules of the actual Wordle game.
mod client;
mod collections;
mod color;
mod constraints;
mod error;
mod game;

pub use client::Client;
pub use color::{
    Color,
    Colors,
};
pub(crate) use constraints::ALPHABET_LENGTH;
pub use constraints::{
    N_GUESSES,
    WORD_LENGTH,
};
pub use error::{
    Error,
    InternalError,
};
pub use game::Wordle;
pub use game::round::Round;
pub use game::round::score::{
    Score,
    ScoreMode,
    ScoreType,
};

/// A specialized [`Result`][std::result::Result] for Wordle [`Error`s][crate::Error].
pub type Result<T> = std::result::Result<T, crate::Error>;
