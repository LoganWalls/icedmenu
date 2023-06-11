use iced::widget::container::{Appearance, StyleSheet};
use iced::{widget, Element};
use icedmenu::apply_styles;
use kdl::KdlNode;

use super::LayoutNode;
use crate::app::{IcedMenu, Message};
use crate::config::ConfigError;
use crate::item::Item;
use crate::layout::style::GenericStyle;

#[derive(Debug)]
pub struct ContainerNodeData {
    pub child: Box<LayoutNode>,
    pub style: GenericStyle,
}

struct ContainerTheme {
    style: GenericStyle,
    default_theme: iced::theme::Container,
}

impl ContainerTheme {
    fn create(style: GenericStyle) -> iced::theme::Container {
        iced::theme::Container::Custom(Box::from(Self {
            style,
            default_theme: iced::theme::Container::default(),
        }))
    }

    fn patch_appearance(&self, mut appear: Appearance) -> Appearance {
        appear.background = self.style.background;
        appear.text_color = self.style.text_color;
        if let Some(v) = self.style.border_width {
            appear.border_width = v;
        }
        if let Some(v) = self.style.border_color {
            appear.border_color = v;
        }
        appear
    }
}

impl StyleSheet for ContainerTheme {
    type Style = iced::Theme;
    fn appearance(&self, style: &Self::Style) -> Appearance {
        let result = style.appearance(&self.default_theme);
        self.patch_appearance(result)
    }
}

pub fn new(
    node: &KdlNode,
    children: Vec<LayoutNode>,
    style: GenericStyle,
) -> Result<LayoutNode, ConfigError> {
    super::validate_children(node, children.len(), 1)?;
    Ok(LayoutNode::Container(ContainerNodeData {
        child: Box::new(children.into_iter().next().unwrap()),
        style,
    }))
}

pub fn view<'a>(
    data: &'a ContainerNodeData,
    menu: &'a IcedMenu,
    item: Option<&'a Item>,
) -> Element<'a, Message> {
    let child = LayoutNode::view(&data.child, menu, item);
    let style = &data.style;

    let result = widget::Container::new(child).style(ContainerTheme::create(style.clone()));
    apply_styles!(
        result,
        style;
        width,
        height,
        max_width,
        max_height,
        padding;
        align_x: horizontal_alignment,
        align_y: vertical_alignment,
    )
    .into()
}
