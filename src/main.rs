mod app;
mod callback;
mod cli;
mod config;
mod font;
mod item;
mod layout;

use crate::app::{Flags, IcedMenu};
use clap::Parser;
use iced::{window, Application, Settings};

use self::app::DEFAULT_FONT_SIZE;

fn main() -> iced::Result {
    let cli_args = cli::CliArgs::parse();
    let flags = Flags::new(cli_args);

    // Get input from stdin
    let window = window::Settings {
        decorations: false,
        resizable: false,
        transparent: true,
        always_on_top: true,
        max_size: None,
        ..Default::default()
    };

    // Display app
    let mut settings = Settings::with_flags(flags);
    settings.window = window;
    settings.default_text_size = DEFAULT_FONT_SIZE;
    IcedMenu::run(settings)
}
