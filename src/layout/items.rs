use iced::widget::button::{Appearance, StyleSheet};
use iced::Element;
use icedmenu::apply_styles;
use kdl::KdlNode;

use super::style::GenericStyle;
use super::LayoutNode;
use crate::app::{IcedMenu, Message};
use crate::config::ConfigError;

#[derive(Debug)]
pub struct ItemsNodeData {
    pub child: Box<LayoutNode>,
    pub style: GenericStyle,
}

pub fn new(
    node: &KdlNode,
    children: Vec<LayoutNode>,
    style: GenericStyle,
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
    }))
}

struct ButtonTheme {
    style: GenericStyle,
    default_theme: iced::theme::Button,
}

impl ButtonTheme {
    fn create(style: GenericStyle) -> iced::theme::Button {
        iced::theme::Button::Custom(Box::from(Self {
            style,
            default_theme: iced::theme::Button::default(),
        }))
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
    fn active(&self, style: &Self::Style) -> Appearance {
        let result = style.active(&self.default_theme);
        self.patch_appearance(result)
    }
    fn hovered(&self, style: &Self::Style) -> Appearance {
        let result = style.hovered(&self.default_theme);
        self.patch_appearance(result)
    }
    fn pressed(&self, style: &Self::Style) -> Appearance {
        let result = style.pressed(&self.default_theme);
        self.patch_appearance(result)
    }
    fn disabled(&self, style: &Self::Style) -> Appearance {
        let result = style.disabled(&self.default_theme);
        self.patch_appearance(result)
    }
}

pub fn view<'a>(data: &'a ItemsNodeData, menu: &'a IcedMenu) -> Vec<Element<'a, Message>> {
    let style = &data.style;
    menu.visible_items
        .iter()
        .enumerate()
        .map(|(_visible_index, item_index)| {
            let item = &menu.items[*item_index];
            let children = LayoutNode::view(&data.child, menu, Some(item));
            let result = iced::widget::button(children).on_press(Message::MouseClicked(item.index));
            //     ;.style(if menu.cursor_position == visible_index {
            //     iced::theme::Button::Primary
            // } else {
            //     iced::theme::Button::Text
            // });
            apply_styles!(result, style; width, height, padding;)
                .style(ButtonTheme::create(style.clone()))
                .into()
        })
        .collect()
}
