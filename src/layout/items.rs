use iced::widget::button::{Appearance, StyleSheet};
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

struct ButtonTheme {
    style: GenericStyle,
}

impl ButtonTheme {
    fn new(style: GenericStyle) -> iced::theme::Button {
        iced::theme::Button::Custom(Box::from(Self { style }))
    }

    fn patch_appearance(&self, mut appear: Appearance) -> Appearance {
        appear.background = self.style.background;
        if let Some(v) = self.style.border_radius {
            appear.border_radius = v;
        }
        if let Some(v) = self.style.border_width {
            appear.border_width = v;
        }
        if let Some(v) = self.style.border_color {
            appear.border_color = v;
        }
        if let Some(v) = self.style.text_color {
            appear.text_color = v;
        }
        appear
    }
}

impl StyleSheet for ButtonTheme {
    type Style = iced::Theme;
    fn active(&self, _style: &Self::Style) -> Appearance {
        let result = iced::Theme::default().active(&iced::theme::Button::default());
        self.patch_appearance(result)
    }
    fn hovered(&self, _style: &Self::Style) -> Appearance {
        let result = iced::Theme::default().hovered(&iced::theme::Button::default());
        self.patch_appearance(result)
    }
    fn pressed(&self, _style: &Self::Style) -> Appearance {
        let result = iced::Theme::default().pressed(&iced::theme::Button::default());
        self.patch_appearance(result)
    }
    fn disabled(&self, _style: &Self::Style) -> Appearance {
        let result = iced::Theme::default().disabled(&iced::theme::Button::default());
        self.patch_appearance(result)
    }
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
            apply_styles!(result, style; width, height, padding;)
                .style(ButtonTheme::new(style.clone()))
                .into()
        })
        .collect();
    widget::column(items).into()
}
