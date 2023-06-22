use iced::{widget, Element};
use icedmenu::{apply_height_styles, apply_styles, apply_width_styles};
use kdl::KdlNode;

use super::style::GenericStyle;
use super::LayoutNode;
use crate::app::Message;
use crate::config::ConfigError;

#[derive(Debug)]
pub struct TextNodeData {
    pub style: GenericStyle,
    pub value: String,
}

pub fn new(
    node: &KdlNode,
    children: Vec<LayoutNode>,
    style: GenericStyle,
) -> Result<LayoutNode, ConfigError> {
    super::validate_children(node, children.len(), 0)?;
    if let Some(v) = node.get("value") {
        if let Some(str_value) = v.value().as_string() {
            Ok(LayoutNode::Text(Box::new(TextNodeData {
                style,
                value: str_value.to_string(),
            })))
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
    let result = widget::text(&data.value);
    apply_styles!(
        result,
        style;
        font,
        width,
        height,
        horizontal_alignment,
        vertical_alignment;
        style: text_color,
        size: font_size,
    )
    .into()
}

pub fn height(data: &TextNodeData) -> u32 {
    let style = &data.style;
    let font = style.font_size.unwrap_or(crate::app::DEFAULT_FONT_SIZE) as u32;
    apply_height_styles!(font, style)
}

pub fn width(data: &TextNodeData) -> u32 {
    let style = &data.style;
    let font = style.font_size.unwrap_or(crate::app::DEFAULT_FONT_SIZE) as u32;
    apply_width_styles!(
        (data.value.chars().count() as f32 * 0.7) as u32 * font,
        style
    )
}
