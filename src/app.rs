use crate::callback::Callback;
use crate::cli::{CaseSensitivity, CliArgs};
use crate::config::{AppContainer, LAYOUT_KEY, STYLES_KEY};
use crate::item::{self, Item};
use crate::layout::{style::parse_styles, LayoutNode};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use iced::keyboard::{self, KeyCode};
use iced::widget::{container, text_input};
use iced::{
    executor, subscription, window, Application, Command, Element, Event, Subscription, Theme,
};
use std::error::Error;
use std::io::{self, Write};
use std::path::PathBuf;

pub struct IcedMenu {
    pub cli_args: CliArgs,
    pub items: Vec<Item>,
    pub visible_items: Vec<usize>,
    pub selected_items: Vec<usize>,
    pub query: String,
    pub cursor_position: usize,
    fuzzy_matcher: SkimMatcherV2,
    callback: Option<Callback>,
    layout: LayoutNode,
}

impl IcedMenu {
    fn update_items(&mut self) {
        if let Some(callback) = &mut self.callback {
            self.items = callback.call(&self.query);
            self.items.truncate(self.cli_args.max_visible);
            self.selected_items = Vec::new();
            self.visible_items = Vec::from_iter(0..self.items.len());
            return;
        }
        self.items.iter_mut().for_each(|item| {
            if self.query.is_empty() || item.selected {
                item.score = None;
                item.match_indices = None;
                return;
            }
            match self
                .fuzzy_matcher
                .fuzzy_indices(&item.data.key, &self.query)
            {
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

        let mut candidates: Vec<&Item> = self
            .items
            .iter()
            .filter(|item| self.query.is_empty() || item.score.is_some())
            .collect();
        candidates.sort_by(|a, b| b.cmp(a));
        self.visible_items = candidates
            .iter()
            .take(self.cli_args.max_visible - self.selected_items.len())
            .map(|item| item.index)
            .chain(self.selected_items.iter().copied())
            .collect();
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

    fn update_selection(&mut self, index: usize, change: SelectionChange) {
        let mut item = &mut self.items[index];
        match change {
            SelectionChange::Select => {
                if self.selected_items.len() < self.cli_args.max {
                    self.selected_items.push(index);
                    item.selected = true;
                }
            }
            SelectionChange::Deselect => {
                self.selected_items.swap_remove(
                    self.selected_items
                        .iter()
                        .position(|x| *x == index)
                        .unwrap(),
                );
                item.selected = false;
            }
            SelectionChange::Toggle => {
                let new_change = SelectionChange::toggle_change(item.selected);
                self.update_selection(index, new_change);
            }
        };
    }

    fn index_under_cursor(&self) -> usize {
        self.visible_items[self.cursor_position]
    }

    pub fn window_height(n_items: u16) -> u32 {
        let query_font_size = 20;
        let item_font_size = 20;
        let query_padding = 10;
        let item_padding = 10;
        let padding = 10;
        let item_spacing = 10;
        (query_font_size
            + 2 * query_padding
            + n_items * (item_font_size + 2 * item_padding)
            + n_items * item_spacing
            + 2 * padding)
            .into()
    }

    fn submit(&self) {
        let selected_items: Vec<&Item> = self.items.iter().filter(|item| item.selected).collect();
        io::stdout()
            .write_all(
                (selected_items
                    .iter()
                    .map(|item| match &item.data.value {
                        Some(value) => value.clone(),
                        None => item.data.key.clone(),
                    })
                    .collect::<Vec<String>>()
                    .join("\n"))
                .as_bytes(),
            )
            .unwrap();
    }
}

fn new_matcher(cli_args: &CliArgs) -> SkimMatcherV2 {
    let matcher = SkimMatcherV2::default();
    match cli_args.case {
        CaseSensitivity::Smart => matcher.smart_case(),
        CaseSensitivity::Respect => matcher.respect_case(),
        CaseSensitivity::Ignore => matcher.ignore_case(),
    }
}

#[derive(Debug, Clone)]
pub enum CursorMoveDirection {
    Up,
    Down,
    Reset,
}

#[derive(Debug, Clone)]
enum SelectionChange {
    Select,
    Deselect,
    Toggle,
}

impl SelectionChange {
    fn toggle_change(selected: bool) -> Self {
        if selected {
            Self::Deselect
        } else {
            Self::Select
        }
    }
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

pub struct Flags {
    pub cli_args: CliArgs,
    pub items: Vec<Item>,
    pub layout: LayoutNode,
    pub callback: Option<Callback>,
}

impl Flags {
    pub fn new(cli_args: CliArgs) -> Self {
        let mut callback = cli_args.callback.clone().map(Callback::new);
        Self {
            items: Self::get_items(&cli_args.file, &cli_args.query, &mut callback)
                .expect("Error while parsing items"),
            layout: Self::get_layout(&cli_args.theme).unwrap(),
            callback,
            cli_args,
        }
    }

    fn get_items(
        path: &Option<PathBuf>,
        query: &str,
        callback: &mut Option<Callback>,
    ) -> Result<Vec<Item>, Box<dyn Error>> {
        match (path, callback) {
            (Some(p), _) => {
                let source = std::fs::File::open(p)?;
                Ok(item::parse_items(source)?)
            }
            (None, Some(c)) => Ok(c.call(query)),
            (None, None) => {
                let source = io::stdin();
                Ok(item::parse_items(source)?)
            }
        }
    }

    fn get_layout(path: &Option<PathBuf>) -> miette::Result<LayoutNode> {
        // TODO: replace unwrap with default style
        let source_path = path.as_ref().unwrap();
        let source = std::fs::read_to_string(source_path).expect("Could not read file");
        let config: kdl::KdlDocument = source.parse()?;
        let layout_definition = config
            .get(LAYOUT_KEY)
            .unwrap_or_else(|| panic!("Could not find {} in your config", LAYOUT_KEY));
        let styles_definition = config
            .get(STYLES_KEY)
            .unwrap_or_else(|| panic!("Could not find {} in your config", STYLES_KEY));

        let wrap_error = |e| {
            miette::Report::from(e)
                .wrap_err("Could not read config file")
                .with_source_code(miette::NamedSource::new(
                    source_path.to_str().unwrap(),
                    source.to_owned(),
                ))
        };
        let styles = parse_styles(styles_definition).map_err(wrap_error)?;
        LayoutNode::new(layout_definition, &styles).map_err(wrap_error)
    }
}

impl Application for IcedMenu {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = Flags;
    type Theme = Theme;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let query_input_id = text_input::Id::new(crate::layout::query::QUERY_INPUT_ID);
        let mut menu = Self {
            fuzzy_matcher: new_matcher(&flags.cli_args),
            query: flags.cli_args.query.clone(),
            items: flags.items,
            callback: flags.callback,
            cli_args: flags.cli_args,
            layout: flags.layout,
            visible_items: Vec::new(),
            selected_items: Vec::new(),
            cursor_position: 0,
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
        self.cli_args.prompt.clone()
    }

    // fn theme(&self) -> Self::Theme {
    //     Theme::custom(theme::Palette {
    //         background: Color::TRANSPARENT,
    //         ..Theme::default().palette()
    //     })
    // }

    fn view(&self) -> Element<Message> {
        let menu = LayoutNode::view(&self.layout, self, None);
        container(menu)
            .style(iced::theme::Container::Custom(AppContainer::new()))
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
                    window::resize::<Message>(1000, Self::window_height(num_items as u16))
                } else {
                    Command::none()
                }
            }
            Message::CursorMoved(direction) => {
                self.move_cursor(direction);
                Command::none()
            }
            Message::CursorSelectionToggled => {
                self.update_selection(self.index_under_cursor(), SelectionChange::Toggle);
                Command::none()
            }
            Message::MouseClicked(index) => {
                self.update_selection(index, SelectionChange::Toggle);
                self.submit();
                window::close()
            }
            Message::Submitted => {
                self.update_selection(self.index_under_cursor(), SelectionChange::Select);
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
