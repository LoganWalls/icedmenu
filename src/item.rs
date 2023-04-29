use crate::menu::Message;
use crate::settings::IcedMenuTheme;
use iced::widget::{button, text, Button, Row};
use iced::{Element, Length};
use std::cmp::{Ord, Ordering};

#[derive(Eq, PartialEq, PartialOrd)]
pub struct Item {
    pub index: usize,
    pub key: String,
    pub value: String,
    pub score: Option<u32>,
    pub match_indices: Option<Vec<usize>>,
    pub selected: bool,
}

impl Item {
    pub fn new(index: usize, key: String, value: String) -> Self {
        Self {
            index,
            key,
            value,
            score: None,
            match_indices: None,
            selected: false,
        }
    }
    pub fn view(&self, theme: &IcedMenuTheme) -> Button<Message> {
        let mut content = Vec::new();
        // Selected indicator
        if self.selected {
            content.push(text("> ").into());
        }
        // Item text with match highlights
        let mut texts: Vec<Element<Message>> = self
            .key
            .char_indices()
            .map(|(i, c)| {
                let mut t = text(c).size(theme.item_font_size);
                match (theme.highlight_matches, &self.match_indices) {
                    (true, Some(indices)) => {
                        if indices.contains(&i) {
                            t = t.style(theme.match_highlight_color)
                        }
                    }
                    _ => (),
                }
                t.into()
            })
            .collect();
        content.append(&mut texts);
        button(Row::with_children(content))
            .width(Length::Fill)
            .padding(theme.item_padding)
            .on_press(Message::MouseClicked(self.index))
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.score, other.score) {
            // Sort by score
            (Some(a), Some(b)) => a.cmp(&b),
            // Items with a score should be above those without
            (Some(_), _) => Ordering::Greater,
            (None, Some(_)) => Ordering::Less,
            // Fallback to the order of the items in the input
            (_, _) => self.index.cmp(&other.index).reverse(),
        }
    }
}
