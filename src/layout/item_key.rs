use iced::widget::{text, Row};
use iced::Element;
use icedmenu::{apply_styles, UpdateFromOther};
use kdl::KdlNode;

use super::style::GenericStyle;
use super::LayoutNode;
use crate::app::{IcedMenu, Message};
use crate::config::ConfigError;
use crate::item::Item;

#[derive(Debug)]
pub struct ItemKeyNodeData {
    pub children: Vec<LayoutNode>,
    pub style: GenericStyle,
    pub hovered_style: GenericStyle,
    pub selected_style: GenericStyle,
}

pub fn new(
    node: &KdlNode,
    children: Vec<LayoutNode>,
    style: GenericStyle,
    hovered_style: GenericStyle,
    selected_style: GenericStyle,
) -> Result<LayoutNode, ConfigError> {
    super::validate_children(node, children.len(), 0)?;
    Ok(LayoutNode::ItemKey(ItemKeyNodeData {
        children,
        style,
        hovered_style,
        selected_style,
    }))
}

pub fn view<'a>(
    data: &ItemKeyNodeData,
    menu: &IcedMenu,
    item: Option<&'a Item>,
) -> Element<'a, Message> {
    let item = item.expect("no Item provided to ItemKey");
    // Use hovered style if this item is under the cursor
    let style = match (
        menu.visible_items[menu.cursor_position] == item.index,
        item.selected,
    ) {
        (true, true) => {
            let mut s: GenericStyle = data.selected_style;
            s.update_from(&data.hovered_style);
            s
        }
        (true, false) => data.hovered_style,
        (false, true) => data.selected_style,
        (false, false) => data.style,
    };
    let mut content = Vec::new();

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
                vertical_alignment,
                font;
                style: text_color,
                size: font_size,
            );
            // Sets the color of the text that matches the query string
            match (&item.match_indices, style.match_text_color) {
                (Some(indices), Some(color)) if indices.contains(&i) => {
                    t = t.style(color);
                }
                _ => (),
            }
            t.into()
        })
        .collect();
    content.append(&mut texts);
    Row::with_children(content).into()
}
