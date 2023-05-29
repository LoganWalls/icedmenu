use std::collections::HashMap;
use std::convert::TryFrom;

use kdl::{KdlDocument, KdlEntry, KdlNode, KdlValue};
use miette::SourceSpan;

use crate::config::ConfigError;

pub enum States {
    Default,
    Hovered,
    Focused,
    Active,
    Pressed,
    Disabled,
}

#[derive(Debug)]
struct StyleAttribute<T> {
    definition_span: SourceSpan,
    value: T,
}

#[derive(Debug, Default)]
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
            let value_def = child.get(0).expect("No value provided for style attribute");
            let definition_span = *child.span();
            let name = child.name().value();

            match name {
                "padding" => result.padding = int_attr(child, value_def)?,
                "margin" => result.margin = int_attr(child, value_def)?,
                "spacing" => result.spacing = int_attr(child, value_def)?,
                "width" => result.width = length_attr(child, value_def)?,
                "height" => result.height = length_attr(child, value_def)?,
                "horizontal_alignment" => {
                    result.horizontal_alignment = Some(StyleAttribute {
                        definition_span,
                        value: match string_value(child, value_def)? {
                            "left" => Ok(iced::alignment::Horizontal::Left),
                            "right" => Ok(iced::alignment::Horizontal::Right),
                            "center" => Ok(iced::alignment::Horizontal::Center),
                            _ => Err(ConfigError::InvalidValue {
                                attr_src: *child.span(),
                                value_src: *value_def.span(),
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
                        value: match string_value(child, value_def)? {
                            "top" => Ok(iced::alignment::Vertical::Top),
                            "bottom" => Ok(iced::alignment::Vertical::Bottom),
                            "center" => Ok(iced::alignment::Vertical::Center),
                            _ => Err(ConfigError::InvalidValue {
                                attr_src: *child.span(),
                                value_src: *value_def.span(),
                                help: String::from(
                                    "vertical_alignment value can be one of: top, bottom, center",
                                ),
                            }),
                        }?,
                    })
                }
                "color" => result.color = color_attr(child, value_def)?,
                _ => {
                    return Err(ConfigError::InvalidStyleAttribute {
                        attr_src: definition_span,
                        help: format!(
                            "Style attributes can be one of: padding, margin, spacing, width, \
                            height, horizontal_alignment, vertical_alignment, color"
                        ),
                    })
                }
            };
        }
        Ok(result)
    }
}

fn int_attr(
    attribute_definition: &KdlNode,
    value_definition: &KdlEntry,
) -> Result<Option<StyleAttribute<u16>>, ConfigError> {
    let attr_span = *attribute_definition.span();
    if let KdlValue::Base10(v) = value_definition.value() {
        u16::try_from(*v).map_err(|_| ())
    } else {
        Err(())
    }
    .map_err(|_| ConfigError::InvalidValue {
        attr_src: attr_span,
        value_src: *value_definition.span(),
        help: format!(
            "The value of a {} style rule should be an integer",
            attribute_definition.name().value()
        ),
    })
    .map(|value| {
        Some(StyleAttribute {
            definition_span: attr_span,
            value,
        })
    })
}

fn length_attr(
    attribute_definition: &KdlNode,
    value_definition: &KdlEntry,
) -> Result<Option<StyleAttribute<iced::Length>>, ConfigError> {
    let attr_span = *attribute_definition.span();
    match value_definition.value() {
        KdlValue::String(v) | KdlValue::RawString(v) => match v.as_str() {
            "fill" => Ok(iced::Length::Fill),
            "shrink" => Ok(iced::Length::Shrink),
            _ => Err(()),
        },
        KdlValue::Base10Float(v) => Ok(iced::Length::Fixed(*v as f32)),
        _ => Err(()),
    }
    .map(|value| {
        Some(StyleAttribute {
            definition_span: attr_span,
            value,
        })
    })
    .map_err(|_| ConfigError::InvalidValue {
        attr_src: attr_span,
        value_src: *value_definition.span(),
        help: format!(
            "{} can be one of: fill, shrink, or a floating point number to specify a fixed size",
            attribute_definition.name().value()
        ),
    })
}

fn string_value<'a>(
    attribute_definition: &KdlNode,
    value_definition: &'a KdlEntry,
) -> Result<&'a str, ConfigError> {
    match value_definition.value() {
        KdlValue::String(v) | KdlValue::RawString(v) => Ok(v.as_str()),
        _ => Err(()),
    }
    .map_err(|_| ConfigError::InvalidValue {
        attr_src: *attribute_definition.span(),
        value_src: *value_definition.span(),
        help: format!(
            "The value of a {} style rule should be a string",
            attribute_definition.name().value()
        ),
    })
}

fn color_attr(
    attribute_definition: &KdlNode,
    value_definition: &KdlEntry,
) -> Result<Option<StyleAttribute<iced::Color>>, ConfigError> {
    let attr_span = *attribute_definition.span();
    let color_str = string_value(attribute_definition, value_definition)?;
    if let Ok(c) = csscolorparser::parse(color_str) {
        let [r, g, b, a] = c.to_rgba8();
        Ok(Some(StyleAttribute {
            definition_span: attr_span,
            value: iced::Color::from_rgba8(r, g, b, (a as f32) / 255.0),
        }))
    } else {
        let name = attribute_definition.name().value();
        Err(ConfigError::InvalidValue {
                attr_src: attr_span,
                value_src: *value_definition.span(),
                help: format!(
                    "The value of a {} style rule should be a string containing a CSS color definition. \
                    \n\tExamples: \
                    \n\t`{name} \"rebeccapurple\" \
                    \n\t`{name} \"#ff0000\" \
                    \n\t`{name} \"rgb(100%, 0%, 10%)\"` \
                    \n\t`{name} \"rgba(255, 0, 0, 1)\"` \
                    \n\t`{name} \"hsl(120, 100%, 50%)\"`",
                    attribute_definition.name().value()
                ),
            })
    }
}
// TODO: read all of the styles from config into hashmap, then as each layout node is created,
// find all of the styles that apply to it and combine them together to form the node's style.
// Then, add a `style` function to each layout node type's module and call it from that node
// type's `view` function.
fn style_rules(node: &KdlNode) -> HashMap<&kdl::KdlIdentifier, GenericStyle> {
    let mut rules = HashMap::new();
    let rule_definitions = node.children().expect("No styles defined").nodes();
    for d in rule_definitions.iter() {
        let target = d.name();
        rules.insert(target, GenericStyle::default());
    }
    rules
}
