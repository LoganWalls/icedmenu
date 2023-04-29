use crate::settings::IcedMenuTheme;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use iced::keyboard::{self, KeyCode};
use iced::widget::Column;
use iced::widget::{button, text, text_input, Button, Row};
use iced::{
    executor, subscription, theme, window, Application, Command, Element, Event, Length,
    Subscription, Theme,
};
use std::cmp::{Ord, Ordering};
use std::io::{self, Write};

struct Item {
    key: String,
    value: String,
}

#[derive(Eq, PartialEq, PartialOrd)]
struct MatchedItem {
    item_index: usize,
    score: Option<u32>,
    match_indices: Option<Vec<usize>>,
}

impl Ord for MatchedItem {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.score, other.score) {
            // Sort by score
            (Some(a), Some(b)) => a.cmp(&b),
            // Items with a score should be above those without
            (Some(_), _) => Ordering::Greater,
            (None, Some(_)) => Ordering::Less,
            // Fallback to the order of the items in the input
            (_, _) => self.item_index.cmp(&other.item_index).reverse(),
        }
    }
}

pub struct IcedMenu {
    prompt: String,
    menu_theme: IcedMenuTheme,
    items: Vec<Item>,
    query: String,
    matches: Vec<MatchedItem>,
    selected_indices: Vec<usize>,
    cursor_position: usize,
    fuzzy_matcher: SkimMatcherV2,
}

impl IcedMenu {
    fn update_matches(&mut self) {
        self.matches = self
            .items
            .iter()
            .enumerate()
            .filter_map(|(i, item)| self.match_item(item, i))
            .collect::<Vec<_>>();
        self.matches.sort_by(|a, b| b.cmp(a));
    }

    fn match_item(&self, item: &Item, item_index: usize) -> Option<MatchedItem> {
        // Don't bother matching the query is empty or the item is selected already
        if self.query.is_empty() || self.selected_indices.contains(&item_index) {
            return Some(MatchedItem {
                item_index,
                score: None,
                match_indices: None,
            });
        }
        match self.fuzzy_matcher.fuzzy_indices(&item.key, &self.query) {
            Some((score, match_indices)) => Some(MatchedItem {
                item_index,
                score: Some(score as u32),
                match_indices: Some(match_indices),
            }),
            None => {
                if self.selected_indices.contains(&item_index) {
                    Some(MatchedItem {
                        item_index,
                        score: None,
                        match_indices: None,
                    })
                } else {
                    None
                }
            }
        }
    }

    fn move_cursor(&mut self, direction: CursorMoveDirection) {
        let num_items = self.matches.len();
        self.cursor_position = match direction {
            CursorMoveDirection::Up => {
                if self.cursor_position == 0 {
                    num_items - 1
                } else {
                    self.cursor_position - 1
                }
            }
            CursorMoveDirection::Down => {
                if self.cursor_position == (num_items - 1) {
                    0
                } else {
                    self.cursor_position + 1
                }
            }
            CursorMoveDirection::Reset => 0,
        }
    }

    fn toggle_selection(&mut self, index: usize) {
        let existing_index = self.selected_indices.iter().position(|&x| x == index);
        match existing_index {
            Some(i) => {
                self.selected_indices.remove(i);
            }
            None => self.selected_indices.push(index),
        }
    }

    fn match_under_cursor(&self) -> &MatchedItem {
        &self.matches[self.cursor_position]
    }

    fn matched_item(&self, matched: &MatchedItem) -> &Item {
        &self.items[matched.item_index]
    }

    fn submit(&self, indices: &Vec<usize>) {
        let selected_items: Vec<&Item> = indices.iter().map(|i| &self.items[*i]).collect();
        io::stdout()
            .write_all(
                (selected_items
                    .iter()
                    .map(|item| item.value.clone())
                    .collect::<Vec<String>>()
                    .join("\n"))
                .as_bytes(),
            )
            .unwrap();
    }

    fn render_item(
        &self,
        item: &Item,
        matched: &MatchedItem,
        under_cursor: bool,
        selected: bool,
    ) -> Button<Message> {
        let mut content = Vec::new();
        // Selected indicator
        if selected {
            content.push(text("> ").into());
        }
        // Item text with match highlights
        let mut texts: Vec<Element<Message>> = item
            .key
            .char_indices()
            .map(|(i, c)| {
                let mut t = text(c).size(self.menu_theme.item_font_size);
                match (self.menu_theme.highlight_matches, &matched.match_indices) {
                    (true, Some(indices)) => {
                        if indices.contains(&i) {
                            t = t.style(self.menu_theme.match_highlight_color)
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
            .padding(self.menu_theme.item_padding)
            .style(if under_cursor {
                theme::Button::Primary
            } else {
                theme::Button::Text
            })
            .on_press(Message::MouseClicked(matched.item_index))
    }
}

impl Default for IcedMenu {
    fn default() -> Self {
        Self {
            menu_theme: IcedMenuTheme::default(),
            prompt: String::from(""),
            query: String::from(""),
            items: Vec::new(),
            matches: Vec::new(),
            selected_indices: Vec::new(),
            cursor_position: 0,
            fuzzy_matcher: SkimMatcherV2::default(),
        }
    }
}

#[derive(Default)]
pub struct Flags {
    pub prompt: String,
    pub items: Vec<String>,
    pub query: String,
    pub menu_theme: IcedMenuTheme,
}

#[derive(Debug, Clone)]
pub enum CursorMoveDirection {
    Up,
    Down,
    Reset,
}

#[derive(Debug, Clone)]
pub enum Message {
    QueryChanged(String),
    CursorMoved(CursorMoveDirection),
    CursorSelectionToggled,
    MouseClicked(usize),
    Submitted,
    Quit,
}

const QUERY_INPUT_ID: &str = "query_input";

impl Application for IcedMenu {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = Flags;
    type Theme = Theme;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let items = flags
            .items
            .iter()
            .map(|x| Item {
                key: x.to_string(),
                value: x.to_string(),
            })
            .collect();
        let query_input_id = text_input::Id::new(QUERY_INPUT_ID);
        let mut menu = Self {
            menu_theme: flags.menu_theme,
            prompt: flags.prompt,
            query: flags.query,
            items,
            matches: Vec::new(),
            selected_indices: Vec::new(),
            cursor_position: 0,
            fuzzy_matcher: SkimMatcherV2::default(),
        };
        menu.update_matches();
        (
            menu,
            Command::batch(vec![
                text_input::focus(query_input_id),
                window::gain_focus(),
            ]),
        )
    }

    fn title(&self) -> String {
        return self.prompt.clone();
    }

    fn view(&self) -> Element<Message> {
        let query_input = text_input(&self.prompt, &self.query)
            .size(self.menu_theme.query_font_size)
            .on_input(Message::QueryChanged)
            .on_submit(Message::Submitted)
            .padding(self.menu_theme.query_padding)
            .id(text_input::Id::new(QUERY_INPUT_ID));
        let mut content = vec![query_input.into()];
        self.matches.iter().enumerate().for_each(|(i, m)| {
            content.push(
                self.render_item(
                    self.matched_item(m),
                    m,
                    i == self.cursor_position,
                    self.selected_indices.contains(&m.item_index),
                )
                .into(),
            );
        });
        Column::with_children(content)
            .padding(self.menu_theme.padding)
            .spacing(self.menu_theme.item_spacing)
            .into()
    }

    fn update(&mut self, message: Message) -> Command<Self::Message> {
        match message {
            Message::QueryChanged(new_query) => {
                let num_items_prev = self.matches.len();
                self.query = new_query;
                self.update_matches();
                let num_items = self.matches.len();
                if num_items != num_items_prev {
                    self.move_cursor(CursorMoveDirection::Reset);
                    window::resize::<Message>(
                        self.menu_theme.window_width,
                        self.menu_theme.window_height(num_items as u16),
                    )
                } else {
                    Command::none()
                }
            }
            Message::CursorMoved(direction) => {
                self.move_cursor(direction);
                Command::none()
            }
            Message::CursorSelectionToggled => {
                self.toggle_selection(self.match_under_cursor().item_index);
                Command::none()
            }
            Message::MouseClicked(index) => {
                self.toggle_selection(index);
                self.submit(&self.selected_indices);
                window::close()
            }
            Message::Submitted => {
                self.toggle_selection(self.match_under_cursor().item_index);
                self.submit(&self.selected_indices);
                window::close()
            }
            Message::Quit => window::close(),
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        subscription::events_with(|event, status| match (event, status) {
            (
                Event::Keyboard(keyboard::Event::KeyPressed {
                    key_code,
                    modifiers,
                }),
                _,
            ) => match (key_code, modifiers) {
                (KeyCode::K, keyboard::Modifiers::CTRL) | (KeyCode::Up, _) => {
                    Some(Message::CursorMoved(CursorMoveDirection::Up))
                }
                (KeyCode::J, keyboard::Modifiers::CTRL) | (KeyCode::Down, _) => {
                    Some(Message::CursorMoved(CursorMoveDirection::Down))
                }
                (KeyCode::Tab, _) => Some(Message::CursorSelectionToggled),
                (KeyCode::Escape, _) | (KeyCode::D, keyboard::Modifiers::CTRL) => {
                    Some(Message::Quit)
                }
                _ => None,
            },
            _ => None,
        })
    }
}
