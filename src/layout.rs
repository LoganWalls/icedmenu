use std::collections::HashMap;
use std::convert::TryFrom;

use iced::Element;
use kdl::{KdlDocument, KdlNode, KdlValue};
use miette::SourceSpan;

use crate::app::{IcedMenu, Message};
use crate::config::ConfigError;

pub mod column;
pub mod container;
pub mod items;
pub mod query;
pub mod row;
pub mod text;

#[derive(Debug)]
pub struct LayoutNodeData {
    pub children: Vec<LayoutNode>,
    pub classes: Vec<String>,
}

#[derive(Debug)]
pub enum LayoutNode {
    Container(LayoutNodeData),
    Row(LayoutNodeData),
    Column(LayoutNodeData),
    Query(LayoutNodeData),
    Items(LayoutNodeData),
    Text(text::LayoutTextNodeData),
}

impl LayoutNode {
    fn possible_values() -> String {
        String::from(
            "Container, \
            Row, \
            Column, \
            Query, \
            Items, \
            Text",
        )
    }
}

impl LayoutNode {
    pub fn new(node: &KdlNode) -> Result<Self, ConfigError> {
        let children = node
            .children()
            .iter()
            .map(|d| d.nodes())
            .flatten()
            .map(Self::new)
            .collect::<Result<Vec<_>, _>>()?;
        let classes: Vec<String> = node
            .entries()
            .iter()
            .filter_map(|e| e.name().map(|n| n.value()))
            .map(|s| String::from(s))
            .collect();

        match node.name().value() {
            "Container" | "Layout" => container::new(node, children, classes),
            "Row" => row::new(node, children, classes),
            "Column" => column::new(node, children, classes),
            "Text" => text::new(node, children, classes),
            "Query" => query::new(node, children, classes),
            "Items" => items::new(node, children, classes),
            _ => Err(ConfigError::InvalidLayoutNode {
                node_src: *node.name().span(),
                help: format!(
                    "Try changing this node to one of: {}",
                    Self::possible_values()
                ),
            }),
        }
    }

    pub fn view<'a>(node: &'a Self, menu: &'a IcedMenu) -> Element<'a, Message> {
        match node {
            Self::Container(data) => container::view(data, menu),
            Self::Row(data) => row::view(data, menu),
            Self::Column(data) => column::view(data, menu),
            Self::Query(_) => query::view(menu),
            Self::Items(_) => items::view(menu),
            Self::Text(data) => text::view(data),
        }
    }
}

fn validate_children(
    node: &KdlNode,
    n_children: usize,
    constraint: usize,
) -> Result<(), ConfigError> {
    if constraint != n_children {
        return Err(ConfigError::InvalidNumberOfChildren {
            parent_src: *node.span(),
            help: format!(
                "A {} node must have exactly {} {}, but yours has {}",
                node.name().value(),
                constraint,
                if n_children == 1 { "child" } else { "children" },
                n_children
            ),
        });
    }
    return Ok(());
}

// pub enum StyleAttribute {
//     Padding(u16),
//     Margin(u16),
//     Spacing(u16),
//     Color(iced::Color),
//     Width(iced::Length),
//     Height(iced::Length),
//     HorizontalAlignment(iced::alignment::Horizontal),
//     VerticalAlignment(iced::alignment::Vertical),
// }

pub enum States {
    Default,
    Hovered,
    Focused,
    Active,
    Pressed,
    Disabled,
}

struct StyleAttribute<T> {
    definition_span: SourceSpan,
    value: T,
}

// impl<u16> StyleAttribute<u16> {
//     fn new(definition: &KdlNode, value_entry: &KdlEntry) -> Result<Self, ConfigError> {
//         Ok(Self {
//             definition_span,
//             value: match value_entry.value() {
//                 KdlValue::Base10(x) => Ok(u16::try_from(*x).map_err(|_| err)?),
//                 _ => Err(err),
//             }?,
//         })
//     }
// }
//
// impl<iced::Color> StyleAttribute<iced::Color> {
//     fn new(definition: &KdlNode, value_entry: &KdlEntry) -> Result<Self, ConfigError> {
//         let definition_span = *definition.span();
//         let err = ConfigError::InvalidValue {
//             attr_src: definition_span,
//             value_src: *value_entry.span(),
//             help: format!(
//                 "The value of a {} style rule should be a string",
//                 definition.name().value()
//             ),
//         };
//         Ok(Self {
//             definition_span,
//             value: match value_entry.value() {
//                 // TODO: parse color here
//                 KdlValue::RawString(x) => Ok(u16::try_from(*x).map_err(|_| err)?),
//                 _ => Err(err),
//             }?,
//         })
//     }
// }
//
// impl<iced::Length> StyleAttribute<iced::Length> {
//     fn new(definition: &KdlNode, value_entry: &KdlEntry) -> Result<Self, ConfigError> {
//         let definition_span = *definition.span();
//         let err = ConfigError::InvalidValue {
//             attr_src: definition_span,
//             value_src: *value_entry.span(),
//             help: format!(
//                 "The value of a {} style rule should be a string",
//                 definition.name().value()
//             ),
//         };
//         Ok(Self {
//             definition_span,
//             value: match value_entry.value() {
//                 // TODO: parse color here
//                 KdlValue::RawString(x) => Ok(u16::try_from(*x).map_err(|_| err)?),
//                 _ => Err(err),
//             }?,
//         })
//     }
// }

struct GenericStyle {
    padding: Option<StyleAttribute<u16>>,
    margin: Option<StyleAttribute<u16>>,
    spacing: Option<StyleAttribute<u16>>,
    color: Option<StyleAttribute<iced::Color>>,
    width: Option<StyleAttribute<iced::Length>>,
    height: Option<StyleAttribute<iced::Length>>,
    horizontal_alignment: Option<StyleAttribute<iced::alignment::Horizontal>>,
    vertical_alignment: Option<StyleAttribute<iced::alignment::Vertical>>,
}
impl GenericStyle {
    fn new(doc: &KdlDocument) -> Result<Self, ConfigError> {
        let mut result = Self::default();
        for child in doc.nodes().iter() {
            let value_definition = child.get(0).expect("No value provided for style attribute");
            let definition_span = *child.span();
            let name = child.name().value();

            let int_attr = || {
                if let KdlValue::Base10(v) = value_definition.value() {
                    u16::try_from(*v).map_err(|_| ())
                } else {
                    Err(())
                }
                .map_err(|_| ConfigError::InvalidValue {
                    attr_src: definition_span,
                    value_src: *value_definition.span(),
                    help: format!("The value of a {} style rule should be an integer", name),
                })
                .map(|value| {
                    Some(StyleAttribute {
                        definition_span,
                        value,
                    })
                })
            };

            let length_attr = || {
                match value_definition.value() {
                        KdlValue::String(v) | KdlValue::RawString(v) => {
                            match v.as_str() {
                                "fill" => Ok(iced::Length::Fill),
                                "shrink" => Ok(iced::Length::Shrink),
                                _ => Err(())
                            }
                        },
                        KdlValue::Base10Float(v) =>  Ok(iced::Length::Fixed(*v as f32)),
                        _ => Err(()),
                    }.map(|value| Some(StyleAttribute{
                        definition_span,
                        value
                    })).map_err(|_| ConfigError::InvalidValue { attr_src: definition_span, value_src: *value_definition.span(), help: format!("{} can be one of: fill, shrink, or a floating point number to specify a fixed size", name) })
            };

            let string_value = || {
                match value_definition.value() {
                    KdlValue::String(v) | KdlValue::RawString(v) => Ok(v.as_str()),
                    _ => Err(()),
                }
                .map_err(|_| ConfigError::InvalidValue {
                    attr_src: definition_span,
                    value_src: *value_definition.span(),
                    help: format!("The value of a {} style rule should be a string", name),
                })
            };

            match name {
                "padding" => result.padding = int_attr()?,
                "margin" => result.margin = int_attr()?,
                "spacing" => result.spacing = int_attr()?,
                "width" => result.width = length_attr()?,
                "height" => result.height = length_attr()?,
                "horizontal_alignment" => {
                    result.horizontal_alignment = Some(StyleAttribute {
                        definition_span,
                        value: match string_value()? {
                            "left" => Ok(iced::alignment::Horizontal::Left),
                            "right" => Ok(iced::alignment::Horizontal::Right),
                            "center" => Ok(iced::alignment::Horizontal::Center),
                            _ => Err(ConfigError::InvalidValue {
                                attr_src: definition_span,
                                value_src: *value_definition.span(),
                                help: String::from(
                                    "horizontal_alignment value can be one of: left, right, center",
                                ),
                            }),
                        }?,
                    })
                }
                "vertical_alignment" => {
                    result.vertical_alignment = Some(StyleAttribute {
                        definition_span,
                        value: match string_value()? {
                            "top" => Ok(iced::alignment::Vertical::Top),
                            "bottom" => Ok(iced::alignment::Vertical::Bottom),
                            "center" => Ok(iced::alignment::Vertical::Center),
                            _ => Err(ConfigError::InvalidValue {
                                attr_src: definition_span,
                                value_src: *value_definition.span(),
                                help: String::from(
                                    "vertical_alignment value can be one of: top, bottom, center",
                                ),
                            }),
                        }?,
                    })
                }
                // TODO: implement
                "color" => result.color = None,
                _ => (),
            };
        }
        Ok(result)
    }
}

impl Default for GenericStyle {
    fn default() -> Self {
        Self {
            padding: None,
            margin: None,
            spacing: None,
            color: None,
            width: None,
            height: None,
            horizontal_alignment: None,
            vertical_alignment: None,
        }
    }
}

fn style_rules(node: &KdlNode) -> HashMap<&kdl::KdlIdentifier, GenericStyle> {
    let mut rules = HashMap::new();
    let rule_definitions = node.children().expect("No styles defined").nodes();
    for d in rule_definitions.iter() {
        let target = d.name();
        rules.insert(target, GenericStyle::default());
    }
    rules
}
