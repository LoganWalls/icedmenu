use std::io::{self, Write};

use iced::keyboard::{self, KeyCode};
use iced::widget::{container, row, text, text_input, Column};
use iced::{
    executor, subscription, theme, window, Application, Command, Element, Event, Length, Settings,
    Subscription, Theme,
};

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

#[derive(Default)]
struct Flags {
    prompt: String,
    items: Vec<String>,
    query: String,
    menu_theme: IcedMenuTheme,
}

struct MenuItem {
    key: String,
    value: String,
}

struct IcedMenu {
    prompt: String,
    menu_theme: IcedMenuTheme,
    items: Vec<MenuItem>,
    query: String,
    visible_indices: Vec<usize>,
    selected_indices: Vec<usize>,
    cursor_position: usize,
    query_input_id: text_input::Id,
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
    Quit,
}

struct IcedMenuTheme {
    window_width: u32,
    padding: u16,
    query_font_size: u16,
    query_padding: u16,
    item_font_size: u16,
    item_padding: u16,
    item_spacing: u16,
}

impl IcedMenuTheme {
    fn window_height(&self, n_items: u16) -> u32 {
        (self.query_font_size
            + 2 * self.query_padding
            + n_items * (self.item_font_size + 2 * self.item_padding)
            + n_items * self.item_spacing
            + 2 * self.padding)
            .into()
    }
}

impl Default for IcedMenuTheme {
    fn default() -> Self {
        let font_size = 20;
        let item_padding = 20;
        Self {
            window_width: 400,
            padding: 20,
            query_font_size: font_size,
            query_padding: item_padding,
            item_font_size: font_size,
            item_padding,
            item_spacing: 10,
        }
    }
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
        let query_input_id = text_input::Id::new("query_input");
        let mut menu = Self {
            menu_theme: flags.menu_theme,
            prompt: flags.prompt,
            query: flags.query,
            items,
            visible_indices: Vec::new(),
            selected_indices: Vec::new(),
            cursor_position: 0,
            query_input_id: query_input_id.clone(),
        };
        menu.update_visible_items();
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
            .id(self.query_input_id.clone());
        let mut content = vec![query_input.into()];
        self.visible_items().enumerate().for_each(|(i, x)| {
            let is_under_cursor = i == self.cursor_position;
            let t = container(row![text(&x.key).size(self.menu_theme.item_font_size)])
                .width(Length::Fill)
                .padding(self.menu_theme.item_padding)
                .style(if is_under_cursor {
                    theme::Container::Box
                } else {
                    theme::Container::Transparent
                });
            content.push(t.into());
        });
        Column::with_children(content)
            .padding(self.menu_theme.padding)
            .spacing(self.menu_theme.item_spacing)
            .into()
    }

    fn update(&mut self, message: Message) -> Command<Self::Message> {
        match message {
            Message::QueryChanged(new_query) => {
                let num_items_prev = self.visible_indices.len();
                self.query = new_query;
                self.update_visible_items();
                let num_items = self.visible_indices.len();
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
                Command::none()
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
                (KeyCode::Tab, _) => Some(Message::SelectionToggled),
                (KeyCode::Escape, _) | (KeyCode::D, keyboard::Modifiers::CTRL) => {
                    Some(Message::Quit)
                }
                _ => None,
            },
            _ => None,
        })
    }
}
