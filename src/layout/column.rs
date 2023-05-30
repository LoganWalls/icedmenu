use iced::{widget, Element};
use kdl::KdlNode;

use super::style::GenericStyle;
use super::{LayoutNode, NodeData};
use crate::app::{IcedMenu, Message};
use crate::config::ConfigError;

pub fn new(
    node: &KdlNode,
    children: Vec<LayoutNode>,
    style: GenericStyle,
) -> Result<LayoutNode, ConfigError> {
    Ok(LayoutNode::Column(NodeData { children, style }))
}

pub fn view<'a>(data: &'a NodeData, menu: &'a IcedMenu) -> Element<'a, Message> {
    let children = data
        .children
        .iter()
        .map(|c| LayoutNode::view(&c, menu))
        .collect();
    widget::column(children).into()
}
