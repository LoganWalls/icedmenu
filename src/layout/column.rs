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
    Ok(LayoutNode::Column(LayoutNodeData { children, classes }))
}

pub fn view<'a>(data: &'a LayoutNodeData, menu: &'a IcedMenu) -> Element<'a, Message> {
    let children = data
        .children
        .iter()
        .map(|c| LayoutNode::view(&c, menu))
        .collect();
    widget::column(children).into()
}
