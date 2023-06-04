use iced::{widget, Element};
use icedmenu::FromGenericStyle;
use kdl::KdlNode;

use super::style::{GenericStyle, StyleAttribute};
use super::LayoutNode;
use crate::app::Message;
use crate::config::ConfigError;

#[derive(Debug)]
pub struct TextNodeData {
    pub style: TextNodeStyle,
    pub value: String,
}

#[derive(Debug, FromGenericStyle)]
pub struct TextNodeStyle {
    color: StyleAttribute<iced::Color>,
    width: StyleAttribute<iced::Length>,
    height: StyleAttribute<iced::Length>,
    horizontal_alignment: StyleAttribute<iced::alignment::Horizontal>,
    vertical_alignment: StyleAttribute<iced::alignment::Vertical>,
}

impl Default for TextNodeStyle {
    fn default() -> Self {
        Self {
            color: StyleAttribute {
                definition_span: None,
                value: iced::Color::BLACK,
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

pub fn new(
    node: &KdlNode,
    children: Vec<LayoutNode>,
    style: GenericStyle,
) -> Result<LayoutNode, ConfigError> {
    super::validate_children(node, children.len(), 0)?;
    if let Some(v) = node.get("value") {
        if let Some(str_value) = v.value().as_string() {
            Ok(LayoutNode::Text(TextNodeData {
                style: style.into(),
                value: str_value.to_string(),
            }))
        } else {
            Err(ConfigError::InvalidArgument {
                arg_src: *v.span(),
                help: "The value for a Text node should be a string: `Text value=\"value\"`"
                    .to_string(),
            })
        }
    } else {
        Err(ConfigError::MissingArgument {
            node_src: *node.span(),
            help: "Text nodes require a value: `Text value=\"value\"`".to_string(),
        })
    }
}

pub fn view(data: &TextNodeData) -> Element<Message> {
    let style = &data.style;
    widget::text(&data.value)
        .style(style.color.value)
        .width(style.width.value)
        .height(style.height.value)
        .horizontal_alignment(style.horizontal_alignment.value)
        .vertical_alignment(style.vertical_alignment.value)
        .into()
}
