use iced::{widget, Element};
use kdl::KdlNode;

use super::{LayoutNode, NodeData};
use crate::app::{IcedMenu, Message};
use crate::config::ConfigError;
use crate::layout::style::GenericStyle;

pub fn new(
    node: &KdlNode,
    children: Vec<LayoutNode>,
    style: GenericStyle,
) -> Result<LayoutNode, ConfigError> {
    super::validate_children(node, children.len(), 1)?;
    Ok(LayoutNode::Container(NodeData { children, style }))
}

pub fn view<'a>(data: &'a NodeData, menu: &'a IcedMenu) -> Element<'a, Message> {
    let child = LayoutNode::view(&data.children[0], menu);
    widget::Container::new(child).into()
}
