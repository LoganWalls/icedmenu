use iced::widget::{text, Row};
use iced::Element;
use icedmenu::{
    apply_height_styles, apply_styles, apply_width_styles, get_item_style, UpdateFromOther,
};
use kdl::KdlNode;

use super::style::GenericStyle;
use super::LayoutNode;
use crate::app::{IcedMenu, Message};
use crate::config::ConfigError;
use crate::item::Item;

#[derive(Debug)]
pub struct ItemKeyNodeData {
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
    Ok(LayoutNode::ItemKey(Box::new(ItemKeyNodeData {
        style,
        hovered_style,
        selected_style,
    })))
}

pub fn view<'a>(
    data: &ItemKeyNodeData,
    menu: &IcedMenu,
    item: Option<&'a Item>,
) -> Element<'a, Message> {
    let item = item.expect("no Item provided to ItemKey");
    // Use hovered style if this item is under the cursor
    let style = get_item_style!(item, data, menu);
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
                height,
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
    Row::with_children(content)
        .spacing(0)
        .width(iced::Length::Shrink)
        .into()
}

pub fn height(data: &ItemKeyNodeData, menu: &IcedMenu, item: Option<&Item>) -> u32 {
    let item = item.expect("no Item provided to ItemKey");
    let style = get_item_style!(item, data, menu);
    apply_height_styles!(
        style.font_size.unwrap_or(crate::app::DEFAULT_FONT_SIZE) as u32,
        style
    )
}

pub fn width(data: &ItemKeyNodeData, menu: &IcedMenu, item: Option<&Item>) -> u32 {
    let item = item.expect("no Item provided to ItemKey");
    let style = get_item_style!(item, data, menu);
    apply_width_styles!(
        item.data.key.chars().count() as u32
            * style.font_size.unwrap_or(crate::app::DEFAULT_FONT_SIZE) as u32,
        style
    )
}
