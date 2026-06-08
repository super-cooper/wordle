//! Common types and values used across [`wordle`] examples.
#![expect(
    clippy::mod_module_files,
    reason = "This is the only way to have common code for examples."
)]
use std::fmt::{
    Display,
    Formatter,
};
use std::io;

use chrono::{
    DateTime,
    Local,
};
use chrono_tz::{
    America,
    Tz,
};
use clap::Parser;
use log::{
    self,
    Level,
    LevelFilter,
    Log,
    Metadata,
    Record,
    SetLoggerError,
};
use serde::Deserialize;

const WORDLE_API_URL: &str = "https://www.nytimes.com/svc/wordle/v2/";
const WORD_LIST_URL: &str = concat!(
    "https://gist.githubusercontent.com/",
    "dracos/",
    "dd0668f281e685bad51479e5acaadb93/",
    "raw/6bfa15d263d6d5b63840a8e5b64e04b382fdb079/",
    "valid-wordle-words.txt"
);

type WordleDate = DateTime<Tz>;

#[allow(
    dead_code,
    reason = "Because of how examples work, the ones which don't use this function will produce a \
              dead code warning."
)]
pub fn now() -> WordleDate {
    // US/New_York is chosen because Wordle is hosted by the New York Times
    Local::now().with_timezone(&America::New_York)
}

struct Logger;

impl Log for Logger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        let msg = format!("{} {}: {}", now(), record.level(), record.args());
        match record.level() {
            Level::Error => eprintln!("{msg}"),
            _ => println!("{msg}"),
        };
    }

    fn flush(&self) {}
}

const LOGGER: Logger = Logger;

pub fn init_logging() -> Result<(), WordleExampleError> {
    log::set_logger(&LOGGER)?;
    log::set_max_level(LevelFilter::Info);
    Ok(())
}

#[derive(Debug)]
pub enum WordleExampleError {
    Wordle { e: wordle::Error },
    Network { e: reqwest::Error },
    Format { msg: String },
    Oof { msg: String },
}

impl Display for WordleExampleError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::Wordle { e } => write!(f, "wordle error: {e}"),
            Self::Network { e } => write!(f, "reqwest error: {e}"),
            Self::Format { msg } => write!(f, "formatting issue: {msg}"),
            Self::Oof { msg } => write!(f, "oof: {msg}"),
        }
    }
}

impl std::error::Error for WordleExampleError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Wordle { e } => Some(e),
            Self::Network { e } => Some(e),
            Self::Format { .. } | Self::Oof { .. } => None,
        }
    }
}

impl From<wordle::Error> for WordleExampleError {
    fn from(e: wordle::Error) -> Self {
        Self::Wordle { e }
    }
}

impl From<io::Error> for WordleExampleError {
    fn from(e: io::Error) -> Self {
        Self::Format { msg: e.to_string() }
    }
}

impl From<reqwest::Error> for WordleExampleError {
    fn from(e: reqwest::Error) -> Self {
        Self::Network { e }
    }
}

impl From<std::fmt::Error> for WordleExampleError {
    fn from(e: std::fmt::Error) -> Self {
        Self::Format { msg: e.to_string() }
    }
}

impl From<SetLoggerError> for WordleExampleError {
    fn from(e: SetLoggerError) -> Self {
        Self::Oof { msg: e.to_string() }
    }
}

#[derive(Deserialize)]
struct WordleSolution {
    solution: String,
}

pub struct WordleApi {
    api_url:     String,
    list_url:    String,
    http_client: reqwest::blocking::Client,
}

impl WordleApi {
    pub fn new(api_url: String, list_url: String) -> Self {
        Self {
            api_url,
            list_url,
            http_client: reqwest::blocking::Client::new(),
        }
    }

    fn fetch_answer_impl(&self, date: WordleDate) -> Result<String, WordleExampleError> {
        let date = date
            .to_rfc3339()
            .split_once('T')
            .map(|(date, _time)| String::from(date))
            .ok_or_else(|| WordleExampleError::Format {
                msg: format!("Could not convert date to RFC 3339 format {date}"),
            })?;

        log::info!("Fetching Wordle answer for {date}");

        let resp = self
            .http_client
            .get(format!("{url}/{date}.json", url = self.api_url))
            .send()?;

        let solution = resp.json::<WordleSolution>()?.solution;

        log::info!("Fetched solution {solution}.");

        Ok(solution)
    }

    fn fetch_word_list_impl(&self) -> Result<Vec<String>, WordleExampleError> {
        log::info!("Fetching Wordle word list");

        let resp = self.http_client.get(&self.list_url).send()?;

        let list = resp
            .text()?
            .trim()
            .split_terminator('\n')
            .map(String::from)
            .collect::<Vec<_>>();

        log::info!("Fetched {} words", list.len());

        Ok(list)
    }
}

impl wordle::Client for WordleApi {
    type Date = WordleDate;
    type Error = WordleExampleError;

    fn fetch_answer(&self, date: Self::Date) -> Result<String, Self::Error> {
        self.fetch_answer_impl(date)
    }

    fn fetch_words(&self) -> Result<impl Iterator<Item = String> + 'static, Self::Error> {
        self.fetch_word_list_impl().map(Vec::into_iter)
    }
}

#[derive(Parser, Debug)]
#[command(about)]
pub struct CommonCli {
    // Comma-separated list of words. If not provided, will fetch from upstream list URL.
    #[arg(long)]
    words:          Option<String>,
    // URL to Wordle API
    #[arg(long, default_value_t = String::from(WORDLE_API_URL))]
    wordle_api_url: String,
    // URL for list of possible solutions
    #[arg(long, default_value_t = String::from(WORD_LIST_URL))]
    word_list_url:  String,
    // The chosen wordle answer. If not provided, will fetch from API.
    #[arg(long)]
    answer:         Option<String>,
}

impl CommonCli {
    pub fn api(&self) -> WordleApi {
        WordleApi::new(self.wordle_api_url.clone(), self.word_list_url.clone())
    }

    pub fn answer(&self) -> Option<String> {
        self.answer.clone()
    }

    pub fn words(&self) -> Option<impl Iterator<Item = &str>> {
        self.words.as_ref().map(|s| s.split(r",|\w"))
    }
}
