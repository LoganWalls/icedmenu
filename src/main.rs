mod item;
mod menu;
mod theme;

use crate::menu::{Flags, IcedMenu};
use crate::theme::IcedMenuTheme;
use clap::{Parser, ValueEnum};
use iced::{window, Application, Settings};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::PathBuf;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum CaseSensitivity {
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
    /// Read items from a file instead of stdin
    #[arg(short, long, value_name = "FILE")]
    file: Option<PathBuf>,

    /// How to treat case-sensitivity
    #[arg(long, value_enum, default_value_t = CaseSensitivity::Smart)]
    case: CaseSensitivity,

    /// The maximum number of items that can be selected
    #[arg(short, long, default_value_t = 1)]
    max: usize,

    /// The prompt to be displayed
    #[arg(short, long, default_value_t = String::from(""))]
    prompt: String,

    /// An initial value for the query
    #[arg(short, long, default_value_t = String::from(""))]
    query: String,

    /// Read a theme from a file
    #[arg(short, long, value_name = "FILE")]
    theme: Option<PathBuf>,
}

impl CliArgs {
    fn read_items(&self) -> Vec<String> {
        match &self.file {
            Some(path) => {
                let file = File::open(path).unwrap();
                io::BufReader::new(file)
                    .lines()
                    .map(|x| x.unwrap_or_default())
                    .collect::<Vec<String>>()
            }
            None => io::stdin()
                .lines()
                .map(|x| x.unwrap_or_default())
                .collect::<Vec<String>>(),
        }
    }

    fn get_theme(&self) -> IcedMenuTheme {
        match &self.theme {
            Some(path) => todo!(),
            None => IcedMenuTheme::default(),
        }
    }
}

fn main() -> iced::Result {
    let cli_args = CliArgs::parse();
    let flags = Flags::new(cli_args);

    // Get input from stdin
    let window = window::Settings {
        decorations: false,
        always_on_top: true,
        max_size: Some((
            flags.theme.window_width,
            flags.theme.window_height(flags.items.len() as u16),
        )),
        ..window::Settings::default()
    };

    // Display app
    let mut settings = Settings::with_flags(flags);
    settings.window = window;
    return IcedMenu::run(settings);
}
