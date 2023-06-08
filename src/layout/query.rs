use iced::widget::text_input::{Appearance, StyleSheet};
use iced::{widget, Color, Element};
use icedmenu::apply_styles;
use kdl::KdlNode;

use super::style::GenericStyle;
use super::{LayoutNode, NodeData};
use crate::app::{IcedMenu, Message};
use crate::config::ConfigError;

pub fn new(
    node: &KdlNode,
    children: Vec<LayoutNode>,
    style: GenericStyle,
) -> Result<LayoutNode, ConfigError> {
    super::validate_children(node, children.len(), 0)?;
    Ok(LayoutNode::Query(NodeData { children, style }))
}

struct TextInputTheme {
    style: GenericStyle,
}

impl TextInputTheme {
    fn new(style: GenericStyle) -> iced::theme::TextInput {
        iced::theme::TextInput::Custom(Box::from(Self { style }))
    }
}

// TODO: figure out how to pull in defaults here
impl StyleSheet for TextInputTheme {
    type Style = iced::Theme;
    fn active(&self, style: &Self::Style) -> Appearance {
        let palette = style.extended_palette();
        Appearance {
            background: self
                .style
                .background
                .unwrap_or(palette.background.base.color.into()),
            border_radius: self.style.border_radius.unwrap_or(2.0),
            border_width: self.style.border_width.unwrap_or(1.0),
            border_color: self
                .style
                .border_color
                .unwrap_or(palette.background.strong.color),
            icon_color: self
                .style
                .icon_color
                .unwrap_or(palette.background.weak.text),
        }
    }

    fn focused(&self, style: &Self::Style) -> Appearance {
        self.active(style)
    }

    fn hovered(&self, style: &Self::Style) -> Appearance {
        let palette = style.extended_palette();
        Appearance {
            background: self
                .style
                .background
                .unwrap_or(palette.background.base.color.into()),
            border_radius: self.style.border_radius.unwrap_or(2.0),
            border_width: self.style.border_width.unwrap_or(1.0),
            border_color: self
                .style
                .border_color
                .unwrap_or(palette.background.base.color),
            icon_color: self
                .style
                .icon_color
                .unwrap_or(palette.background.weak.text),
        }
    }

    fn disabled(&self, style: &Self::Style) -> Appearance {
        let palette = style.extended_palette();
        Appearance {
            background: self
                .style
                .background
                .unwrap_or(palette.background.weak.color.into()),
            border_radius: self.style.border_radius.unwrap_or(2.0),
            border_width: self.style.border_width.unwrap_or(1.0),
            border_color: self
                .style
                .border_color
                .unwrap_or(palette.background.strong.color),
            icon_color: self
                .style
                .icon_color
                .unwrap_or(palette.background.strong.text),
        }
    }

    fn placeholder_color(&self, style: &Self::Style) -> Color {
        style.extended_palette().background.strong.color
    }

    fn value_color(&self, style: &Self::Style) -> Color {
        style.extended_palette().background.base.color
    }

    fn selection_color(&self, style: &Self::Style) -> Color {
        style.extended_palette().background.weak.color
    }

    fn disabled_color(&self, style: &Self::Style) -> Color {
        self.placeholder_color(style)
    }
}

pub const QUERY_INPUT_ID: &str = "query_input";
pub fn view<'a>(data: &NodeData, menu: &IcedMenu) -> Element<'a, Message> {
    let result = widget::text_input(&menu.cli_args.prompt, &menu.query)
        .on_input(Message::QueryChanged)
        .on_submit(Message::Submitted)
        .id(widget::text_input::Id::new(QUERY_INPUT_ID));
    let style = &data.style;
    apply_styles!(
        result,
        style;
        width,
        padding;
        size: font_size,
    )
    .style(TextInputTheme::new(style.clone()))
    .into()
}
