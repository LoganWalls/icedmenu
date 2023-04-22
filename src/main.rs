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

struct IcedMenu {
    prompt: String,
    items: Vec<String>,
    query: String,
    visible_items: Vec<String>,
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
    Submitted,
}

fn filter_items(items: &Vec<String>, query: &str) -> Vec<String> {
    let mut new_items = items.clone();
    new_items.retain(|item| item.starts_with(query));
    new_items.truncate(50);
    return new_items;
}

impl IcedMenu {
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
        }
    }
}

impl Application for IcedMenu {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = Flags;
    type Theme = Theme;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let visible_items = filter_items(&flags.items, &flags.initial_query);
        (
            Self {
                prompt: flags.prompt,
                items: flags.items,
                query: flags.initial_query,
                visible_items,
                cursor_position: 0,
            },
            Command::none(),
        )
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
        self.visible_items.iter().enumerate().for_each(|(i, x)| {
            let is_selected = i == self.cursor_position;
            let t = container(text(x).style(if is_selected {
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
                self.query = new_query;
                let num_items_prev = self.visible_items.len();
                self.visible_items = filter_items(&self.items, &self.query);
                if self.visible_items.len() != num_items_prev {
                    self.move_cursor(CursorMoveDirection::Reset)
                }
            }
            Message::CursorMoved(direction) => self.move_cursor(direction),
            Message::Submitted => {
                let selected_item = &self.visible_items[self.cursor_position];
                io::stdout().write_all((selected_item).as_bytes()).unwrap();
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
                (KeyCode::K, keyboard::Modifiers::CTRL)
                | (KeyCode::Up, _)
                | (KeyCode::Tab, keyboard::Modifiers::SHIFT) => {
                    Some(Message::CursorMoved(CursorMoveDirection::Up))
                }
                (KeyCode::J, keyboard::Modifiers::CTRL)
                | (KeyCode::Down, _)
                | (KeyCode::Tab, _) => Some(Message::CursorMoved(CursorMoveDirection::Down)),
                _ => None,
            },
            _ => None,
        })
    }
}
