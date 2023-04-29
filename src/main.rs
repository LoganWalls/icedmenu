mod item;
mod menu;
mod settings;

use crate::menu::{Flags, IcedMenu};
use crate::settings::IcedMenuTheme;
use iced::{window, Application, Settings};
use std::io;

fn main() -> iced::Result {
    // Get input from stdin
    let items = io::stdin()
        .lines()
        .map(|x| x.unwrap_or_default())
        .collect::<Vec<String>>();
    let prompt = String::from("");
    let query = String::from("");
    let menu_theme = IcedMenuTheme::default();
    let window = window::Settings {
        decorations: false,
        always_on_top: true,
        max_size: Some((
            menu_theme.window_width,
            menu_theme.window_height(items.len() as u16),
        )),
        ..window::Settings::default()
    };

    // Display app
    return IcedMenu::run(Settings {
        flags: Flags {
            prompt,
            items,
            query,
            menu_theme,
        },
        window,
        ..Settings::default()
    });
}
