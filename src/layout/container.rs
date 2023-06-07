use iced::{widget, Element};
use icedmenu::{apply_styles, define_theme};
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

define_theme!(
    ContainerTheme,
    iced::widget::container::Appearance,
    iced::widget::container::StyleSheet,
    iced::theme::Container;
    background,
    text_color,
    border_width,
    border_color;
);

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
