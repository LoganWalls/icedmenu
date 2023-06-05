use iced::widget::container::Appearance;
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

    let style_fn = |_theme: &iced::Theme| -> Appearance {
        Appearance {
            background: style.background.map(iced::Background::Color),
            text_color: style.text_color,
            // border_width: style.border_width,
            // border_color: style.border_color,
            ..Default::default()
        }
    };

    let result = widget::Container::new(child);
    //FIXME: .style(style_fn);
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
