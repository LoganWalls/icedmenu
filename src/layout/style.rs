use std::collections::HashMap;
use std::convert::TryFrom;
use std::iter::once;

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

#[derive(Debug, Clone, Copy)]
pub struct StyleAttribute<T> {
    pub definition_span: Option<SourceSpan>,
    pub value: T,
}

#[derive(Debug, Default)]
pub struct GenericStyle {
    pub padding: Option<StyleAttribute<u16>>,
    pub margin: Option<StyleAttribute<u16>>,
    pub spacing: Option<StyleAttribute<u16>>,
    pub color: Option<StyleAttribute<iced::Color>>,
    pub width: Option<StyleAttribute<iced::Length>>,
    pub height: Option<StyleAttribute<iced::Length>>,
    pub horizontal_alignment: Option<StyleAttribute<iced::alignment::Horizontal>>,
    pub vertical_alignment: Option<StyleAttribute<iced::alignment::Vertical>>,
}

impl GenericStyle {
    fn new(doc: &KdlDocument) -> Result<Self, ConfigError> {
        let mut result = Self::default();
        for child in doc.nodes().iter() {
            let value_def = child.get(0).expect("No value provided for style attribute");
            let definition_span = Some(*child.span());
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
                                    "`horizontal_alignment` can be one of: left, right, center",
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
                                    "`vertical_alignment` can be one of: top, bottom, center",
                                ),
                            }),
                        }?,
                    })
                }
                "color" => result.color = color_attr(child, value_def)?,
                _ => {
                    return Err(ConfigError::InvalidStyleAttribute {
                        attr_src: *child.span(),
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

    fn update_from(&mut self, style: &Self) {
        if style.padding.is_some() {
            self.padding = style.padding
        }
        if style.margin.is_some() {
            self.margin = style.margin
        }
        if style.spacing.is_some() {
            self.spacing = style.spacing
        }
        if style.color.is_some() {
            self.color = style.color
        }
        if style.width.is_some() {
            self.width = style.width
        }
        if style.height.is_some() {
            self.height = style.height
        }
        if style.horizontal_alignment.is_some() {
            self.horizontal_alignment = style.horizontal_alignment
        }
        if style.vertical_alignment.is_some() {
            self.vertical_alignment = style.vertical_alignment
        }
    }
}

fn int_attr(
    attribute_definition: &KdlNode,
    value_definition: &KdlEntry,
) -> Result<Option<StyleAttribute<u16>>, ConfigError> {
    let attr_span = *attribute_definition.name().span();
    if let KdlValue::Base10(v) = value_definition.value() {
        u16::try_from(*v).map_err(|_| ())
    } else {
        Err(())
    }
    .map_err(|_| ConfigError::InvalidValue {
        attr_src: attr_span,
        value_src: *value_definition.span(),
        help: format!(
            "The value of a `{}` attribute should be an integer",
            attribute_definition.name().value()
        ),
    })
    .map(|value| {
        Some(StyleAttribute {
            definition_span: Some(attr_span),
            value,
        })
    })
}

fn length_attr(
    attribute_definition: &KdlNode,
    value_definition: &KdlEntry,
) -> Result<Option<StyleAttribute<iced::Length>>, ConfigError> {
    let attr_span = *attribute_definition.name().span();
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
            definition_span: Some(attr_span),
            value,
        })
    })
    .map_err(|_| ConfigError::InvalidValue {
        attr_src: attr_span,
        value_src: *value_definition.span(),
        help: format!(
            "`{}` can be one of: fill, shrink, or a floating point number to specify a fixed size",
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
        attr_src: *attribute_definition.name().span(),
        value_src: *value_definition.span(),
        help: format!(
            "The value of a `{}` attribute should be a string",
            attribute_definition.name().value()
        ),
    })
}

fn color_attr(
    attribute_definition: &KdlNode,
    value_definition: &KdlEntry,
) -> Result<Option<StyleAttribute<iced::Color>>, ConfigError> {
    let attr_span = *attribute_definition.name().span();
    let color_str = string_value(attribute_definition, value_definition)?;
    if let Ok(c) = csscolorparser::parse(color_str) {
        let [r, g, b, a] = c.to_rgba8();
        Ok(Some(StyleAttribute {
            definition_span: Some(attr_span),
            value: iced::Color::from_rgba8(r, g, b, (a as f32) / 255.0),
        }))
    } else {
        let name = attribute_definition.name().value();
        Err(ConfigError::InvalidValue {
                attr_src: attr_span,
                value_src: *value_definition.span(),
                help: format!(
                    "The value of a `{}` attribute should be a string containing a CSS color definition. \
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

pub struct StyleLookup {
    styles: HashMap<String, GenericStyle>,
}
impl StyleLookup {
    pub fn style_for(&self, classes: Vec<&str>, node_type: &str) -> GenericStyle {
        let mut style = GenericStyle::default();
        for c in once(node_type).chain(classes) {
            if let Some(s) = self.styles.get(c) {
                style.update_from(s);
            }
        }
        style
    }
}

// TODO: read all of the styles from config into hashmap, then as each layout node is created,
// find all of the styles that apply to it and combine them together to form the node's style.
// Then, add a `style` function to each layout node type's module and call it from that node
// type's `view` function.
pub fn parse_styles(node: &KdlNode) -> Result<StyleLookup, ConfigError> {
    let mut styles: HashMap<String, GenericStyle> = HashMap::new();
    let style_definitions = node.children().expect("No styles defined").nodes();
    for style_definition in style_definitions.iter() {
        let target = style_definition.name().value();
        let style_attrs = style_definition.children().ok_or(ConfigError::EmptyStyle {
            attr_src: *style_definition.span(),
            help: String::from("Try deleting this style or adding an attribute to it"),
        })?;
        let style = GenericStyle::new(style_attrs)?;
        match styles.get_mut(target) {
            Some(existing_style) => existing_style.update_from(&style),
            None => {
                styles.insert(target.to_string(), style);
            }
        }
    }
    Ok(StyleLookup { styles })
}
