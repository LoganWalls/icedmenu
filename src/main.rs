mod item;
mod menu;
mod theme;

use crate::menu::{Flags, IcedMenu};
use crate::theme::IcedMenuTheme;
use clap::{Parser, ValueEnum, ValueHint};
use iced::{window, Application, Settings};
use std::error::Error;
use std::fs::File;
use std::io;
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
}

impl CliArgs {
    fn get_items(&self) -> Result<Vec<item::Item>, Box<dyn Error>> {
        match &self.file {
            Some(path) => {
                let source = io::BufReader::new(File::open(path)?);
                Ok(item::parse_items(source)?)
            }
            None => {
                let source = io::stdin();
                Ok(item::parse_items(source)?)
            }
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
    let items = cli_args
        .get_items()
        .expect("Error parsing items is your format correct?");
    let n_visible_items = items.len().min(cli_args.max_visible);
    let flags = Flags {
        items,
        theme: cli_args.get_theme(),
        cli_args,
    };

    // Get input from stdin
    let window = window::Settings {
        decorations: false,
        always_on_top: true,
        max_size: Some((
            flags.theme.window_width,
            flags.theme.window_height(n_visible_items as u16),
        )),
        ..window::Settings::default()
    };

    // Display app
    let mut settings = Settings::with_flags(flags);
    settings.window = window;
    return IcedMenu::run(settings);
}
