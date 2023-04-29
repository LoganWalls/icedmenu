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

#[derive(Eq, PartialEq, PartialOrd)]
struct Item {
    index: usize,
    key: String,
    value: String,
    score: Option<u32>,
    match_indices: Option<Vec<usize>>,
    selected: bool,
}

impl Item {
    fn new(index: usize, key: String, value: String) -> Self {
        Self {
            index,
            key,
            value,
            score: None,
            match_indices: None,
            selected: false,
        }
    }
    fn view(&self, theme: &IcedMenuTheme) -> Button<Message> {
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

pub struct IcedMenu {
    prompt: String,
    menu_theme: IcedMenuTheme,
    items: Vec<Item>,
    visible_items: Vec<usize>,
    query: String,
    cursor_position: usize,
    fuzzy_matcher: SkimMatcherV2,
}

impl IcedMenu {
    fn update_items(&mut self) {
        self.items.iter_mut().for_each(|item| {
            if self.query.is_empty() || item.selected {
                item.score = None;
                item.match_indices = None;
                return;
            }
            match self.fuzzy_matcher.fuzzy_indices(&item.key, &self.query) {
                Some((score, match_indices)) => {
                    item.score = Some(score as u32);
                    item.match_indices = Some(match_indices);
                }
                None => {
                    item.score = None;
                    item.match_indices = None;
                }
            }
        });

        if self.query.is_empty() {
            self.visible_items = self.items.iter().map(|item| item.index).collect();
            return;
        }
        let mut candidates: Vec<&Item> = self
            .items
            .iter()
            .filter(|item| item.score.is_some() || item.selected)
            .collect();
        candidates.sort_by(|a, b| b.cmp(&a));
        self.visible_items = candidates.iter().map(|item| item.index).collect();
    }

    fn move_cursor(&mut self, direction: CursorMoveDirection) {
        let num_items = self.visible_items.len();
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
        };
    }

    fn toggle_selection(&mut self, index: usize) {
        let mut item = &mut self.items[index];
        item.selected = !item.selected;
    }

    fn index_under_cursor(&self) -> usize {
        self.visible_items[self.cursor_position]
    }

    fn submit(&self) {
        let selected_items: Vec<&Item> = self.items.iter().filter(|item| item.selected).collect();
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
}

impl Default for IcedMenu {
    fn default() -> Self {
        Self {
            menu_theme: IcedMenuTheme::default(),
            prompt: String::from(""),
            query: String::from(""),
            items: Vec::new(),
            visible_items: Vec::new(),
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
            .enumerate()
            .map(|(i, x)| Item::new(i, x.to_string(), x.to_string()))
            .collect();
        let query_input_id = text_input::Id::new(QUERY_INPUT_ID);
        let mut menu = Self {
            menu_theme: flags.menu_theme,
            prompt: flags.prompt,
            query: flags.query,
            items,
            visible_items: Vec::new(),
            cursor_position: 0,
            fuzzy_matcher: SkimMatcherV2::default(),
        };
        menu.update_items();
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
        let mut menu_items: Vec<Element<Message>> = self
            .visible_items
            .iter()
            .enumerate()
            .map(|(visible_index, item_index)| {
                let item = &self.items[*item_index];
                item.view(&self.menu_theme)
                    .style(if self.cursor_position == visible_index {
                        theme::Button::Primary
                    } else {
                        theme::Button::Text
                    })
                    .into()
            })
            .collect();
        content.append(&mut menu_items);

        Column::with_children(content)
            .padding(self.menu_theme.padding)
            .spacing(self.menu_theme.item_spacing)
            .into()
    }

    fn update(&mut self, message: Message) -> Command<Self::Message> {
        match message {
            Message::QueryChanged(new_query) => {
                let num_items_prev = self.visible_items.len();
                self.query = new_query;
                self.update_items();
                let num_items = self.visible_items.len();
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
                self.toggle_selection(self.index_under_cursor());
                Command::none()
            }
            Message::MouseClicked(index) => {
                self.toggle_selection(index);
                self.submit();
                window::close()
            }
            Message::Submitted => {
                self.toggle_selection(self.index_under_cursor());
                self.submit();
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
