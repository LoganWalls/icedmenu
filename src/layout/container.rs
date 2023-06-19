use iced::widget::container::{Appearance, StyleSheet};
use iced::{widget, Element};
use icedmenu::{apply_height_styles, apply_styles, apply_width_styles};
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

    let result = widget::Container::new(child).style(ContainerTheme::create(*style));
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

pub fn height(data: &ContainerNodeData, menu: &IcedMenu, item: Option<&Item>) -> u32 {
    let style = &data.style;
    let child_height = LayoutNode::height(&data.child, menu, item);
    let padding = style.padding.unwrap_or(0) as u32;
    let border_width = style.border_width.unwrap_or(0.0) as u32;
    apply_height_styles!(child_height + 2 * padding, style) + 2 * border_width
}

pub fn width(data: &ContainerNodeData, menu: &IcedMenu, item: Option<&Item>) -> u32 {
    let style = &data.style;
    let child_width = LayoutNode::width(&data.child, menu, item);
    let padding = style.padding.unwrap_or(0) as u32;
    let border_width = style.border_width.unwrap_or(0.0) as u32;
    apply_width_styles!(child_width + 2 * padding, style) + 2 * border_width
}
