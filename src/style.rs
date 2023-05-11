use std::str::FromStr;

use iced::{theme, widget::container, Color};
use kdl::{KdlDocument, KdlNode};

pub const LAYOUT_KEY: &str = "Layout";
pub const STYLES_KEY: &str = "Styles";

#[derive(Debug)]
pub enum LayoutNodeKind {
    Container,
    Row,
    Column,
    Query,
    Items,
    // KeyText,
}

impl FromStr for LayoutNodeKind {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            LAYOUT_KEY | "Container" => Ok(Self::Container),
            "Row" => Ok(Self::Row),
            "Column" => Ok(Self::Column),
            "Query" => Ok(Self::Query),
            "Items" => Ok(Self::Items),
            // "KeyText" => Ok(Self::KeyText),
            _ => Err(format!("Invalid element name: '{}'", s)),
        }
    }
}

#[derive(Debug)]
pub struct Layout {
    pub kind: LayoutNodeKind,
    pub classes: Vec<String>,
    pub children: Vec<Self>,
}

impl Layout {
    pub fn new(node: &KdlNode) -> miette::Result<Self> {
        // TODO: better errors via meitte
        let kind: LayoutNodeKind = node.name().value().parse().unwrap();
        let children = node
            .children()
            .iter()
            .map(|d| d.nodes())
            .flatten()
            .map(Self::new)
            .collect::<Result<Vec<_>, _>>()?;
        let classes: Vec<String> = node
            .entries()
            .iter()
            .filter_map(|e| match e.name() {
                Some(n) => {
                    if n.value() == "class" {
                        Some(e.value())
                    } else {
                        None
                    }
                }
                None => None,
            })
            .filter_map(|v| v.as_string())
            .map(|s| String::from(s))
            .collect();
        Ok(Self {
            kind,
            classes,
            children,
        })
    }
}

enum StyleAttribute {
    Padding(u16),
    Margin(u16),
    Spacing(u16),
    Color(iced::Color),
    Width(iced::Length),
    Height(iced::Length),
    HorizontalAlignment(iced::alignment::Horizontal),
    VerticalAlignment(iced::alignment::Vertical),
    Font,
}

enum States {
    Hovered,
    Focused,
    Active,
    Pressed,
    Disabled,
}

pub struct StyleRule {
    classes: Vec<String>,
    attributes: Vec<StyleAttribute>,
}

pub fn read_config() -> miette::Result<()> {
    let config: KdlDocument = std::fs::read_to_string("examples/config.kdl")
        .expect("Could not read file")
        .parse()?;
    let window = config
        .get(LAYOUT_KEY)
        .expect(&format!("Could not find {} in your config", LAYOUT_KEY));
    let styles = config
        .get(STYLES_KEY)
        .expect(&format!("Could not find {} in your config", STYLES_KEY));
    dbg!(Layout::new(window).unwrap());

    Ok(())
}

pub struct AppContainer {}

impl AppContainer {
    pub fn new() -> Box<dyn container::StyleSheet<Style = iced::theme::Theme>> {
        Box::new(Self {}) as Box<dyn container::StyleSheet<Style = iced::theme::Theme>>
    }
}

impl container::StyleSheet for AppContainer {
    type Style = iced::theme::Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            // border_radius: 10.0,
            // border_color: Color::BLACK,
            // border_width: 2.0,
            // background: Color::WHITE.into(),
            ..Default::default()
        }
    }
}

#[derive(Debug)]
pub struct IcedMenuTheme {
    pub window_width: u32,
    pub padding: u16,
    pub query_font_size: u16,
    pub query_padding: u16,
    pub item_font_size: u16,
    pub item_padding: u16,
    pub item_spacing: u16,
    pub highlight_matches: bool,
    pub match_highlight_color: Color,
}

impl IcedMenuTheme {
    pub fn window_height(&self, n_items: u16) -> u32 {
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
        Self {
            window_width: 400,
            padding: 10,
            query_font_size: 20,
            query_padding: 10,
            item_font_size: 20,
            item_padding: 10,
            item_spacing: 10,
            highlight_matches: true,
            match_highlight_color: theme::Theme::default().palette().primary,
        }
    }
}
