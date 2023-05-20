use iced::{widget, Element};
use kdl::KdlNode;

use super::LayoutNode;
use crate::app::Message;
use crate::style::ConfigError;

#[derive(Debug)]
pub struct LayoutTextNodeData {
    pub classes: Vec<String>,
    pub value: String,
}

pub fn new(
    node: &KdlNode,
    children: Vec<LayoutNode>,
    classes: Vec<String>,
) -> Result<LayoutNode, ConfigError> {
    super::validate_children(node, children.len(), 0)?;
    if let Some(v) = node.get("value") {
        if let Some(str_value) = v.value().as_string() {
            Ok(LayoutNode::Text(LayoutTextNodeData {
                classes,
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

pub fn view(data: &LayoutTextNodeData) -> Element<Message> {
    widget::text(&data.value).into()
}
