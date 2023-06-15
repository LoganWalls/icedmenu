use iced::widget::text_input::{Appearance, StyleSheet};
use iced::{widget, Color, Element};
use icedmenu::{apply_styles, UpdateFromOther};
use kdl::KdlNode;

use super::style::{GenericStyle, StyleLookup};
use super::LayoutNode;
use crate::app::{IcedMenu, Message};
use crate::config::ConfigError;

#[derive(Debug)]
pub struct QueryNodeData {
    pub children: Vec<LayoutNode>,
    pub style: GenericStyle,
    pub focused_style: GenericStyle,
}

pub fn new(
    node: &KdlNode,
    children: Vec<LayoutNode>,
    style: GenericStyle,
    focused_style: GenericStyle,
) -> Result<LayoutNode, ConfigError> {
    super::validate_children(node, children.len(), 0)?;
    Ok(LayoutNode::Query(QueryNodeData {
        children,
        style,
        focused_style,
    }))
}

struct TextInputTheme {
    style: GenericStyle,
    focused_style: GenericStyle,
    default_theme: iced::theme::TextInput,
}

impl TextInputTheme {
    fn create(style: GenericStyle, focused_style: GenericStyle) -> iced::theme::TextInput {
        iced::theme::TextInput::Custom(Box::from(Self {
            style,
            focused_style,
            default_theme: iced::theme::TextInput::default(),
        }))
    }

    fn patch_appearance(mut appear: Appearance, style: &GenericStyle) -> Appearance {
        if let Some(v) = style.background {
            appear.background = v;
        }
        if let Some(v) = style.border_radius {
            appear.border_radius = v;
        }
        if let Some(v) = style.border_width {
            appear.border_width = v;
        }
        if let Some(v) = style.border_color {
            appear.border_color = v;
        }
        if let Some(v) = style.icon_color {
            appear.icon_color = v;
        }
        appear
    }
}

impl StyleSheet for TextInputTheme {
    type Style = iced::Theme;
    fn active(&self, style: &Self::Style) -> Appearance {
        let result = style.active(&self.default_theme);
        Self::patch_appearance(result, &self.style)
    }

    fn focused(&self, style: &Self::Style) -> Appearance {
        let result = style.focused(&self.default_theme);
        Self::patch_appearance(result, &self.focused_style)
    }

    fn hovered(&self, style: &Self::Style) -> Appearance {
        let result = style.hovered(&self.default_theme);
        Self::patch_appearance(result, &self.style)
    }

    fn disabled(&self, style: &Self::Style) -> Appearance {
        let result = style.disabled(&self.default_theme);
        Self::patch_appearance(result, &self.style)
    }

    fn placeholder_color(&self, style: &Self::Style) -> Color {
        style.placeholder_color(&self.default_theme)
    }

    fn value_color(&self, style: &Self::Style) -> Color {
        style.value_color(&self.default_theme)
    }

    fn selection_color(&self, style: &Self::Style) -> Color {
        style.selection_color(&self.default_theme)
    }

    fn disabled_color(&self, style: &Self::Style) -> Color {
        style.disabled_color(&self.default_theme)
    }
}

pub const QUERY_INPUT_ID: &str = "query_input";
pub fn view<'a>(data: &QueryNodeData, menu: &IcedMenu) -> Element<'a, Message> {
    let result = widget::text_input(&menu.cli_args.prompt, &menu.query)
        .on_input(Message::QueryChanged)
        .on_submit(Message::Submitted)
        .id(widget::text_input::Id::new(QUERY_INPUT_ID));
    let style = &data.style;
    apply_styles!(
        result,
        style;
        font,
        width,
        padding;
        size: font_size,
    )
    .style(TextInputTheme::create(
        style.clone(),
        data.focused_style.clone(),
    ))
    .into()
}
