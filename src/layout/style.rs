use std::collections::HashMap;
use std::convert::TryFrom;

use kdl::{KdlDocument, KdlNode, KdlValue};
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

struct StyleAttribute<T> {
    definition_span: SourceSpan,
    value: T,
}

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
