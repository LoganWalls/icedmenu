use clap::{Parser, ValueEnum, ValueHint};
use std::path::PathBuf;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum CaseSensitivity {
    /// Case-insensitive only when query is entirely lowercase
    Smart,
    /// Case-sensitive search
    Respect,
    /// Case-insensitive search
    Ignore,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    /// The prompt to be displayed
    #[arg(short, long, default_value_t = String::from(""))]
    prompt: String,

    /// An initial value for the query
    #[arg(short, long, default_value_t = String::from(""))]
    query: String,

    /// Read items from a file instead of stdin
    #[arg(short, long, value_name = "FILE", value_hint = ValueHint::FilePath)]
    file: Option<PathBuf>,

    /// Read a theme from a file
    #[arg(short, long, value_name = "FILE", value_hint = ValueHint::FilePath)]
    theme: Option<PathBuf>,

    /// How to treat case-sensitivity
    #[arg(long, value_enum, default_value_t = CaseSensitivity::Smart)]
    case: CaseSensitivity,

    /// The maximum number of items that can be selected
    #[arg(short, long, default_value_t = 1)]
    max: usize,

    /// The maximum number of items that can be displayed at once
    #[arg(long, default_value_t = 10)]
    max_visible: usize,

    /// Execute an external command to populate items whenever the query is changed
    /// The $QUERY env variable will be set to the current query before each execution
    #[arg(long, value_name = "COMMAND", verbatim_doc_comment)]
    callback: Option<String>,
}
