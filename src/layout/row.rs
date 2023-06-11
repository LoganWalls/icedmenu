use iced::{widget, Element};
use icedmenu::apply_styles;

use super::style::GenericStyle;
use super::{items, LayoutNode, NodeData};
use crate::app::{IcedMenu, Message};
use crate::config::ConfigError;
use crate::item::Item;

pub fn new(children: Vec<LayoutNode>, style: GenericStyle) -> Result<LayoutNode, ConfigError> {
    Ok(LayoutNode::Row(NodeData { children, style }))
}

pub fn view<'a>(
    data: &'a NodeData,
    menu: &'a IcedMenu,
    item: Option<&'a Item>,
) -> Element<'a, Message> {
    let children = data
        .children
        .iter()
        .flat_map(|child| match child {
            LayoutNode::Items(data) => items::view(data, menu),
            _ => vec![LayoutNode::view(child, menu, item)],
        })
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
