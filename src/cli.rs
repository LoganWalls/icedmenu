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
    pub prompt: String,

    /// An initial value for the query
    #[arg(short, long, default_value_t = String::from(""))]
    pub query: String,

    /// Read items from a file instead of stdin
    #[arg(short, long, value_name = "FILE", value_hint = ValueHint::FilePath)]
    pub file: Option<PathBuf>,

    /// Read a theme from a file
    #[arg(short, long, value_name = "FILE", value_hint = ValueHint::FilePath)]
    pub theme: Option<PathBuf>,

    /// How to treat case-sensitivity
    #[arg(long, value_enum, default_value_t = CaseSensitivity::Smart)]
    pub case: CaseSensitivity,

    /// The maximum number of items that can be selected
    #[arg(short, long, default_value_t = 1)]
    pub max: usize,

    /// The maximum number of items that can be displayed at once
    #[arg(long, default_value_t = 10)]
    pub max_visible: usize,

    /// Execute an external command to populate items whenever the query is changed
    /// String args with the value $QUERY will be set to the current query before
    /// each execution.
    #[arg(last = true, value_name = "COMMAND", verbatim_doc_comment, num_args = 1..)]
    pub callback: Option<Vec<String>>,
}
