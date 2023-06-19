use iced::{widget, Element};
use icedmenu::{apply_height_styles, apply_styles, apply_width_styles};

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
            LayoutNode::Items(data) => items::views(data, menu),
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

pub fn height(data: &NodeData, menu: &IcedMenu, item: Option<&Item>) -> u32 {
    let children_height = data
        .children
        .iter()
        .flat_map(|child| match child {
            LayoutNode::Items(data) => items::heights(data, menu),
            _ => vec![LayoutNode::height(child, menu, item)],
        })
        .max()
        .unwrap_or(0);
    let style = &data.style;
    apply_height_styles!(
        children_height + 2 * style.padding.unwrap_or(0) as u32,
        style
    )
}

pub fn width(data: &NodeData, menu: &IcedMenu, item: Option<&Item>) -> u32 {
    let children_width: u32 = data
        .children
        .iter()
        .flat_map(|child| match child {
            LayoutNode::Items(data) => items::widths(data, menu),
            _ => vec![LayoutNode::width(child, menu, item)],
        })
        .sum();
    let n_children = data
        .children
        .iter()
        .map(|c| match c {
            LayoutNode::Items(_) => menu.visible_items.len() as u32,
            _ => 1,
        })
        .sum::<u32>();
    let style = &data.style;
    apply_width_styles!(
        children_width
            + 2 * style.padding.unwrap_or(0) as u32
            + n_children.saturating_sub(1) * style.spacing.unwrap_or(0) as u32,
        style
    )
}
