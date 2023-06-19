use iced::widget::button::{Appearance, StyleSheet};
use iced::Element;
use icedmenu::{
    apply_height_styles, apply_styles, apply_width_styles, get_item_style, UpdateFromOther,
};
use kdl::KdlNode;

use super::style::GenericStyle;
use super::LayoutNode;
use crate::app::{IcedMenu, Message};
use crate::config::ConfigError;

#[derive(Debug)]
pub struct ItemsNodeData {
    pub child: Box<LayoutNode>,
    pub style: GenericStyle,
    pub hovered_style: GenericStyle,
    pub pressed_style: GenericStyle,
    pub selected_style: GenericStyle,
}

pub fn new(
    node: &KdlNode,
    children: Vec<LayoutNode>,
    style: GenericStyle,
    hovered_style: GenericStyle,
    pressed_style: GenericStyle,
    selected_style: GenericStyle,
) -> Result<LayoutNode, ConfigError> {
    super::validate_children(node, children.len(), 1)?;
    match &children[0] {
        LayoutNode::Container(_) | LayoutNode::Row(_) | LayoutNode::Column(_) => Ok(()),
        _ => Err(ConfigError::InvalidChildren {
            parent_src: *node.span(),
            help: String::from(
                "The child of an Items node should be one of: Container, Row, Column",
            ),
        }),
    }?;
    Ok(LayoutNode::Items(ItemsNodeData {
        child: Box::new(children.into_iter().next().unwrap()),
        style,
        hovered_style,
        pressed_style,
        selected_style,
    }))
}

struct ButtonTheme {
    style: GenericStyle,
    hovered_style: GenericStyle,
    pressed_style: GenericStyle,
    default_theme: iced::theme::Button,
}

impl ButtonTheme {
    fn create(
        style: GenericStyle,
        hovered_style: GenericStyle,
        pressed_style: GenericStyle,
    ) -> iced::theme::Button {
        iced::theme::Button::Custom(Box::from(Self {
            style,
            hovered_style,
            pressed_style,
            default_theme: iced::theme::Button::Text,
        }))
    }

    fn patch_appearance(mut appear: Appearance, style: &GenericStyle) -> Appearance {
        appear.background = style.background;
        if let Some(v) = style.border_radius {
            appear.border_radius = v;
        }
        if let Some(v) = style.border_width {
            appear.border_width = v;
        }
        if let Some(v) = style.border_color {
            appear.border_color = v;
        }
        if let Some(v) = style.text_color {
            appear.text_color = v;
        }
        appear
    }
}

impl StyleSheet for ButtonTheme {
    type Style = iced::Theme;
    fn active(&self, style: &Self::Style) -> Appearance {
        let result = style.active(&self.default_theme);
        Self::patch_appearance(result, &self.style)
    }
    fn hovered(&self, style: &Self::Style) -> Appearance {
        let result = style.hovered(&self.default_theme);
        Self::patch_appearance(result, &self.hovered_style)
    }
    fn pressed(&self, style: &Self::Style) -> Appearance {
        let result = style.pressed(&self.default_theme);
        Self::patch_appearance(result, &self.pressed_style)
    }
    fn disabled(&self, style: &Self::Style) -> Appearance {
        let result = style.disabled(&self.default_theme);
        Self::patch_appearance(result, &self.style)
    }
}

pub fn views<'a>(data: &'a ItemsNodeData, menu: &'a IcedMenu) -> Vec<Element<'a, Message>> {
    menu.visible_items
        .iter()
        .map(|item_index| {
            let item = &menu.items[*item_index];
            let children = LayoutNode::view(&data.child, menu, Some(item));
            let result = iced::widget::button(children).on_press(Message::MouseClicked(item.index));
            let style = get_item_style!(item, data, menu);
            apply_styles!(result, style; width, height, padding;)
                .style(ButtonTheme::create(
                    style,
                    data.hovered_style,
                    data.pressed_style,
                ))
                .into()
        })
        .collect()
}

pub fn heights(data: &ItemsNodeData, menu: &IcedMenu) -> Vec<u32> {
    menu.visible_items
        .iter()
        .map(|item_index| {
            let item = &menu.items[*item_index];
            let style = get_item_style!(item, data, menu);
            let item_height = LayoutNode::height(&data.child, menu, Some(item));
            apply_height_styles!(item_height + 2 * style.padding.unwrap_or(0) as u32, style)
                + 2 * style.border_width.unwrap_or(0.0) as u32
        })
        .collect()
}

pub fn widths(data: &ItemsNodeData, menu: &IcedMenu) -> Vec<u32> {
    menu.visible_items
        .iter()
        .map(|item_index| {
            let item = &menu.items[*item_index];
            let style = get_item_style!(item, data, menu);
            let item_width = LayoutNode::width(&data.child, menu, Some(item));
            apply_width_styles!(item_width + 2 * style.padding.unwrap_or(0) as u32, style)
                + 2 * style.border_width.unwrap_or(0.0) as u32
        })
        .collect()
}
