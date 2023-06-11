use iced::widget::{text, Row};
use iced::{Color, Element};
use icedmenu::apply_styles;
use kdl::KdlNode;

use super::style::GenericStyle;
use super::{LayoutNode, NodeData};
use crate::app::Message;
use crate::config::ConfigError;
use crate::item::Item;

pub fn new(
    node: &KdlNode,
    children: Vec<LayoutNode>,
    style: GenericStyle,
) -> Result<LayoutNode, ConfigError> {
    super::validate_children(node, children.len(), 0)?;
    Ok(LayoutNode::ItemKey(NodeData { children, style }))
}

pub fn view<'a>(data: &NodeData, item: Option<&'a Item>) -> Element<'a, Message> {
    let item = item.expect("no Item provided to ItemKey");
    let style = &data.style;

    let mut content = Vec::new();
    // Selected indicator
    if item.selected {
        content.push(text("> ").into());
    }
    // Item text with match highlights
    let mut texts: Vec<Element<Message>> = item
        .data
        .key
        .char_indices()
        .map(|(i, c)| {
            let mut t = text(c);
            t = apply_styles!(
                t,
                style;
                width,
                height,
                horizontal_alignment,
                vertical_alignment;
                style: color,
                size: font_size,
            );
            // Sets the color of the text that matches the query string
            if let Some(indices) = &item.match_indices {
                if indices.contains(&i) {
                    t = t.style(Color::from_rgb(0.5, 0.5, 1.0))
                }
            }
            t.into()
        })
        .collect();
    content.append(&mut texts);
    Row::with_children(content).into()
}
