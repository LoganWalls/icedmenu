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
    super::validate_children(node, children.len(), 0)?;
    Ok(LayoutNode::Query(LayoutNodeData { children, classes }))
}

pub const QUERY_INPUT_ID: &str = "query_input";
pub fn view<'a>(menu: &IcedMenu) -> Element<'a, Message> {
    widget::text_input(&menu.cli_args.prompt, &menu.query)
        .on_input(Message::QueryChanged)
        .on_submit(Message::Submitted)
        .id(widget::text_input::Id::new(QUERY_INPUT_ID))
        .into()
}
