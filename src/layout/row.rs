use iced::{widget, Element};
use kdl::KdlNode;

use super::{LayoutNode, LayoutNodeData};
use crate::menu::{IcedMenu, Message};
use crate::style::ConfigError;

pub fn new(
    node: &KdlNode,
    children: Vec<LayoutNode>,
    classes: Vec<String>,
) -> Result<LayoutNode, ConfigError> {
    Ok(LayoutNode::Row(LayoutNodeData { children, classes }))
}

pub fn view<'a>(data: &'a LayoutNodeData, menu: &'a IcedMenu) -> Element<'a, Message> {
    let children = data
        .children
        .iter()
        .map(|c| LayoutNode::view(&c, menu))
        .collect();
    widget::row(children).into()
}
