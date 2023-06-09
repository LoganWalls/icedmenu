use iced::{widget, Element};
use icedmenu::apply_styles;

use super::style::GenericStyle;
use super::{LayoutNode, NodeData};
use crate::app::{IcedMenu, Message};
use crate::config::ConfigError;

pub fn new(children: Vec<LayoutNode>, style: GenericStyle) -> Result<LayoutNode, ConfigError> {
    Ok(LayoutNode::Row(NodeData { children, style }))
}

pub fn view<'a>(data: &'a NodeData, menu: &'a IcedMenu) -> Element<'a, Message> {
    let children = data
        .children
        .iter()
        .map(|c| LayoutNode::view(c, menu))
        .collect();
    let result = widget::row(children);
    let style = &data.style;
    apply_styles!(
        result,
        style;
        width,
        height,
        spacing,
        padding,
        align_items;
    )
    .into()
}
