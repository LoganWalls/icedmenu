use crate::app::Message;
use iced::widget::{button, text, Button, Row};
use iced::{Color, Element};
use std::cmp::{Ord, Ordering};
use std::{error::Error, io};

#[derive(Debug, Eq, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
pub struct ItemData {
    pub key: String,
    #[serde(default)]
    pub value: Option<String>,
}

#[derive(Debug, Eq, PartialEq, PartialOrd)]
pub struct Item {
    pub index: usize,
    pub data: ItemData,
    pub score: Option<u32>,
    pub match_indices: Option<Vec<usize>>,
    pub selected: bool,
}

impl Item {
    pub fn new(index: usize, data: ItemData) -> Self {
        Self {
            index,
            data,
            score: None,
            match_indices: None,
            selected: false,
        }
    }
    pub fn view(&self) -> Button<Message> {
        let mut content = Vec::new();
        // Selected indicator
        if self.selected {
            content.push(text("> ").into());
        }
        // Item text with match highlights
        let mut texts: Vec<Element<Message>> = self
            .data
            .key
            .char_indices()
            .map(|(i, c)| {
                let mut t = text(c).size(20);
                if let Some(indices) = &self.match_indices {
                    if indices.contains(&i) {
                        t = t.style(Color::from_rgb(0.5, 0.5, 1.0))
                    }
                }
                t.into()
            })
            .collect();
        content.append(&mut texts);
        button(Row::with_children(content)).on_press(Message::MouseClicked(self.index))
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

pub fn parse_items(source: impl io::Read) -> Result<Vec<Item>, Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(source);
    let mut result = Vec::new();
    for (i, data) in rdr.deserialize::<ItemData>().enumerate() {
        result.push(Item::new(i, data?));
    }
    Ok(result)
}
