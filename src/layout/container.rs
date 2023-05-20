use iced::{widget, Element};
use kdl::KdlNode;

use super::{LayoutNode, LayoutNodeData};
use crate::app::{IcedMenu, Message};
use crate::config::ConfigError;

pub fn new(
    node: &KdlNode,
    children: Vec<LayoutNode>,
    classes: Vec<String>,
) -> Result<LayoutNode, ConfigError> {
    super::validate_children(node, children.len(), 1)?;
    Ok(LayoutNode::Container(LayoutNodeData { children, classes }))
}

pub fn view<'a>(data: &'a LayoutNodeData, menu: &'a IcedMenu) -> Element<'a, Message> {
    let child = LayoutNode::view(&data.children[0], menu);
    widget::Container::new(child).into()
}
