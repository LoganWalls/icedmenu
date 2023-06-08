use iced::widget::container::{Appearance, StyleSheet};
use iced::{widget, Element};
use icedmenu::apply_styles;
use kdl::KdlNode;

use super::LayoutNode;
use crate::app::{IcedMenu, Message};
use crate::config::ConfigError;
use crate::layout::style::GenericStyle;

#[derive(Debug)]
pub struct ContainerNodeData {
    pub child: Box<LayoutNode>,
    pub style: GenericStyle,
}

struct ContainerTheme {
    style: GenericStyle,
}

impl ContainerTheme {
    fn new(style: GenericStyle) -> iced::theme::Container {
        iced::theme::Container::Custom(Box::from(Self { style }))
    }
}

impl StyleSheet for ContainerTheme {
    type Style = iced::Theme;
    fn appearance(&self, iced_theme: &Self::Style) -> Appearance {
        let mut result = Appearance::default();
        result.background = self.style.background;
        result.text_color = self.style.text_color;
        if let Some(v) = self.style.border_width {
            result.border_width = v;
        }
        if let Some(v) = self.style.border_color {
            result.border_color = v;
        }
        result
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

pub fn view<'a>(data: &'a ContainerNodeData, menu: &'a IcedMenu) -> Element<'a, Message> {
    let child = LayoutNode::view(&data.child, menu);
    let style = &data.style;

    let result = widget::Container::new(child).style(ContainerTheme::new(style.clone()));
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
