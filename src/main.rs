use std::io::{self, Write};

use iced::keyboard::{self, KeyCode};
use iced::widget::{container, text, text_input, Column};
use iced::{
    executor, subscription, window, Application, Color, Command, Element, Event, Settings,
    Subscription, Theme,
};

fn main() -> iced::Result {
    // Get input from stdin
    let items = io::stdin()
        .lines()
        .map(|x| x.unwrap_or_default())
        .collect::<Vec<String>>();
    let prompt = String::from("Choose you character");
    let initial_query = String::from("");

    // Display app
    return IcedMenu::run(Settings {
        flags: Flags {
            prompt,
            items,
            initial_query,
        },
        window: window::Settings {
            // decorations: false,
            // always_on_top: true,
            ..window::Settings::default()
        },
        ..Settings::default()
    });
}

#[derive(Default)]
struct Flags {
    prompt: String,
    items: Vec<String>,
    initial_query: String,
}

struct MenuItem {
    key: String,
    value: String,
}

struct IcedMenu {
    prompt: String,
    items: Vec<MenuItem>,
    query: String,
    visible_indices: Vec<usize>,
    selected_indices: Vec<usize>,
    cursor_position: usize,
}

#[derive(Debug, Clone)]
enum CursorMoveDirection {
    Up,
    Down,
    Reset,
}

#[derive(Debug, Clone)]
enum Message {
    QueryChanged(String),
    CursorMoved(CursorMoveDirection),
    SelectionToggled,
    Submitted,
}

impl IcedMenu {
    fn update_visible_items(&mut self) {
        self.visible_indices = self
            .items
            .iter()
            .enumerate()
            .filter(|(i, item)| {
                item.key
                    .to_lowercase()
                    .starts_with(&self.query.to_lowercase())
                    || self.selected_indices.contains(i)
            })
            .take(50)
            .map(|(i, _)| i)
            .collect()
    }

    fn move_cursor(&mut self, direction: CursorMoveDirection) {
        let num_items = self.visible_indices.len();
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

    fn visible_item(&self, visible_index: usize) -> &MenuItem {
        &self.items[self.visible_indices[visible_index]]
    }

    fn visible_items(&self) -> impl Iterator<Item = &MenuItem> {
        self.visible_indices.iter().map(|i| &self.items[*i])
    }
}

impl Application for IcedMenu {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = Flags;
    type Theme = Theme;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let items = flags
            .items
            .iter()
            .map(|x| MenuItem {
                key: x.to_string(),
                value: x.to_string(),
            })
            .collect();
        let mut menu = Self {
            prompt: flags.prompt,
            items,
            query: flags.initial_query,
            visible_indices: Vec::new(),
            selected_indices: Vec::new(),
            cursor_position: 0,
        };
        menu.update_visible_items();
        (menu, Command::none())
    }

    fn title(&self) -> String {
        return String::from("");
    }

    fn view(&self) -> Element<Message> {
        let query_box = text_input(&self.prompt, &self.query)
            .on_input(Message::QueryChanged)
            .on_submit(Message::Submitted)
            .padding(10)
            .size(30);
        let mut content = vec![query_box.into()];
        self.visible_items().enumerate().for_each(|(i, x)| {
            let is_selected = i == self.cursor_position;
            let t = container(text(&x.key).style(if is_selected {
                Color::WHITE
            } else {
                Color::BLACK
            }));
            content.push(t.into());
        });
        return Column::with_children(content)
            .padding(20)
            .spacing(20)
            .into();
    }

    fn update(&mut self, message: Message) -> Command<Self::Message> {
        match message {
            Message::QueryChanged(new_query) => {
                let num_items_prev = self.visible_indices.len();
                self.query = new_query;
                self.update_visible_items();
                if self.visible_indices.len() != num_items_prev {
                    self.move_cursor(CursorMoveDirection::Reset)
                }
            }
            Message::CursorMoved(direction) => self.move_cursor(direction),
            Message::SelectionToggled => {
                let selected_index = self.visible_indices[self.cursor_position];
                let existing_index = self
                    .selected_indices
                    .iter()
                    .position(|&x| x == selected_index);
                match existing_index {
                    Some(i) => {
                        self.selected_indices.remove(i);
                    }
                    None => self.selected_indices.push(selected_index),
                }
            }
            Message::Submitted => {
                let selected_items: Vec<&MenuItem> = if self.selected_indices.len() > 0 {
                    self.selected_indices
                        .iter()
                        .map(|i| &self.items[*i])
                        .collect()
                } else {
                    vec![self.visible_item(self.cursor_position)]
                };
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
                return window::close();
            }
        }
        return Command::none();
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
                (KeyCode::Tab, _) => Some(Message::SelectionToggled),
                _ => None,
            },
            _ => None,
        })
    }
}
