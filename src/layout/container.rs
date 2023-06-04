use iced::widget::container::{Appearance, StyleSheet};
use iced::{theme, widget, Element};
use icedmenu::FromGenericStyle;
use kdl::KdlNode;

use super::style::StyleAttribute;
use super::LayoutNode;
use crate::app::{IcedMenu, Message};
use crate::config::ConfigError;
use crate::layout::style::GenericStyle;

#[derive(Debug)]
pub struct ContainerNodeData {
    pub child: Box<LayoutNode>,
    pub style: ContainerNodeStyle,
}

#[derive(Clone, FromGenericStyle, Debug)]
pub struct ContainerNodeStyle {
    text_color: StyleAttribute<iced::Color>,
    background: StyleAttribute<iced::Color>,
    width: StyleAttribute<iced::Length>,
    height: StyleAttribute<iced::Length>,
    horizontal_alignment: StyleAttribute<iced::alignment::Horizontal>,
    vertical_alignment: StyleAttribute<iced::alignment::Vertical>,
}

impl Default for ContainerNodeStyle {
    fn default() -> Self {
        Self {
            text_color: StyleAttribute {
                definition_span: None,
                value: iced::Color::BLACK,
            },
            background: StyleAttribute {
                definition_span: None,
                value: iced::Color::TRANSPARENT,
            },
            width: StyleAttribute {
                definition_span: None,
                value: iced::Length::Shrink,
            },
            height: StyleAttribute {
                definition_span: None,
                value: iced::Length::Shrink,
            },
            horizontal_alignment: StyleAttribute {
                definition_span: None,
                value: iced::alignment::Horizontal::Left,
            },
            vertical_alignment: StyleAttribute {
                definition_span: None,
                value: iced::alignment::Vertical::Center,
            },
        }
    }
}

impl StyleSheet for ContainerNodeStyle {
    type Style = iced::Theme;

    fn appearance(&self, _theme: &iced::Theme) -> Appearance {
        Appearance {
            background: Some(iced::Background::Color(self.background.value)),
            text_color: Some(self.text_color.value),
            border_width: 10.0,
            border_color: iced::Color::from_rgb(1.0, 0.0, 0.0),
            ..Default::default()
        }
    }
}

pub fn new(
    node: &KdlNode,
    children: Vec<LayoutNode>,
    style: GenericStyle,
) -> Result<LayoutNode, ConfigError> {
    super::validate_children(node, children.len(), 1)?;
    Ok(LayoutNode::Container(ContainerNodeData {
        child: Box::new(children.into_iter().next().unwrap()),
        style: style.into(),
    }))
}

pub fn view<'a>(data: &'a ContainerNodeData, menu: &'a IcedMenu) -> Element<'a, Message> {
    let child = LayoutNode::view(&data.child, menu);
    let style = &data.style;
    dbg!(style);
    widget::Container::new(child)
        .style(theme::Container::Custom(Box::new(style.clone())))
        .width(style.width.value)
        .height(style.height.value)
        .align_x(style.horizontal_alignment.value)
        .align_y(style.vertical_alignment.value)
        .into()
}
