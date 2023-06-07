use iced::{widget, Element};
use icedmenu::apply_styles;
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
    super::validate_children(node, children.len(), 0)?;
    Ok(LayoutNode::Items(NodeData { children, style }))
}

pub fn view<'a>(data: &NodeData, menu: &'a IcedMenu) -> Element<'a, Message> {
    let style = &data.style;
    let items = menu
        .visible_items
        .iter()
        .enumerate()
        .map(|(visible_index, item_index)| {
            let item = &menu.items[*item_index];
            let result = item.view().style(if menu.cursor_position == visible_index {
                iced::theme::Button::Primary
            } else {
                iced::theme::Button::Text
            });
            apply_styles!(result, style; width, height, padding;).into()
        })
        .collect();
    widget::column(items).into()
}
