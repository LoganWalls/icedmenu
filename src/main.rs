mod callback;
mod cli;
mod item;
mod layout;
mod menu;
mod style;

use crate::menu::{Flags, IcedMenu};
use clap::Parser;
use iced::{window, Application, Settings};

fn main() -> iced::Result {
    let cli_args = cli::CliArgs::parse();
    let flags = Flags::new(cli_args);
    let n_visible_items = flags.items.len().min(flags.cli_args.max_visible);

    // Get input from stdin
    let window = window::Settings {
        decorations: false,
        resizable: false,
        transparent: true,
        always_on_top: true,
        max_size: Some((1000, IcedMenu::window_height(n_visible_items as u16))),
        ..Default::default()
    };

    // Display app
    let mut settings = Settings::with_flags(flags);
    settings.window = window;
    return IcedMenu::run(settings);
}
