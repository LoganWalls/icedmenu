use iced::{widget, Element};
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

#[derive(Debug, Default)]
pub struct TextNodeStyle {
    color: Option<StyleAttribute<iced::Color>>,
    width: Option<StyleAttribute<iced::Length>>,
    height: Option<StyleAttribute<iced::Length>>,
    horizontal_alignment: Option<StyleAttribute<iced::alignment::Horizontal>>,
    vertical_alignment: Option<StyleAttribute<iced::alignment::Vertical>>,
}

impl From<GenericStyle> for TextNodeStyle {
    fn from(value: GenericStyle) -> Self {
        let mut result = Self::default();
        if value.color.is_some() {
            result.color = value.color
        }
        if value.width.is_some() {
            result.width = value.width
        }
        if value.height.is_some() {
            result.height = value.height
        }
        if value.horizontal_alignment.is_some() {
            result.horizontal_alignment = value.horizontal_alignment
        }
        if value.vertical_alignment.is_some() {
            result.vertical_alignment = value.vertical_alignment
        }
        result
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
    widget::text(&data.value).into()
}
