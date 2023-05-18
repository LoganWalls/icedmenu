use iced::{theme, widget::container, Color};
use kdl::KdlNode;
use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

pub const LAYOUT_KEY: &str = "Layout";
pub const STYLES_KEY: &str = "Styles";

#[derive(Debug)]
pub struct LayoutNodeData {
    pub children: Vec<LayoutNode>,
    pub classes: Vec<String>,
}

#[derive(Debug)]
pub struct LayoutTextNodeData {
    pub children: Vec<LayoutNode>,
    pub classes: Vec<String>,
    pub value: String,
}

#[derive(Debug)]
pub enum LayoutNode {
    Container(LayoutNodeData),
    Row(LayoutNodeData),
    Column(LayoutNodeData),
    Query(LayoutNodeData),
    Items(LayoutNodeData),
    Text(LayoutTextNodeData),
}

impl LayoutNode {
    fn possible_values() -> String {
        String::from(
            "Container, \
            Row, \
            Column, \
            Text, \
            Query, \
            Items",
        )
    }

    fn children_constraint(&self) -> Option<usize> {
        match self {
            Self::Query(_) | Self::Items(_) | Self::Text(_) => Some(0),
            Self::Container(_) => Some(1),
            _ => None,
        }
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
            .filter_map(|e| match e.name() {
                Some(n) => {
                    if n.value() == "class" {
                        Some(e.value())
                    } else {
                        None
                    }
                }
                None => None,
            })
            .filter_map(|v| v.as_string())
            .map(|s| String::from(s))
            .collect();

        // Determine what kind of node this is.
        let name = node.name();
        match name.value() {
            "Container" | "Layout" => Ok(Self::Container(LayoutNodeData { children, classes })),
            "Row" => Ok(Self::Row(LayoutNodeData { children, classes })),
            "Column" => Ok(Self::Column(LayoutNodeData { children, classes })),
            "Text" => {
                if let Some(v) = node.get("value") {
                    if let Some(str_value) = v.value().as_string() {
                        Ok(Self::Text(LayoutTextNodeData {
                            children,
                            classes,
                            value: str_value.to_string(),
                        }))
                    } else {
                        Err(ConfigError::InvalidArgument {
                            arg_src: *v.span(),
                            help: "The value for a Text node should be a string: `Text value=\"value\"`"
                                .to_string(),
                        })
                    }
                } else {
                    Err(ConfigError::MissingArgument {
                        node_src: *node.span(),
                        help: "Text nodes require a value: `Text value=\"value\"`".to_string(),
                    })
                }
            }
            "Query" => Ok(Self::Query(LayoutNodeData { children, classes })),
            "Items" => Ok(Self::Items(LayoutNodeData { children, classes })),
            // "KeyText" => Ok(Self::KeyText),
            _ => Err(ConfigError::InvalidLayoutNode {
                node_src: *name.span(),
                help: format!(
                    "Try changing this node to one of: {}",
                    Self::possible_values()
                ),
            }),
        }

        // // Validate the node's children
        // let children = node.children();
        // let constraint = kind.children_constraint();
        // match (children, constraint) {
        //     (Some(c), Some(n)) => {
        //         let child_nodes = c.nodes();
        //         let cn = child_nodes.len();
        //         if cn != n {
        //             Err(ConfigError::InvalidNumberOfChildren {
        //                 parent_src: *node.span(),
        //                 help: format!(
        //                     "A {} node must have exactly {} {}, but yours has {}",
        //                     name,
        //                     n,
        //                     if n == 1 { "child" } else { "children" },
        //                     cn
        //                 ),
        //             })
        //         } else {
        //             Ok(kind)
        //         }
        //     }
        //     _ => Ok(kind),
        // }
        //
        // Ok(Self {
        //     kind,
        //     classes,
        //     children,
        // })
    }
}

#[derive(Error, Diagnostic, Debug)]
pub enum ConfigError {
    #[error("Invalid layout node type")]
    #[diagnostic()]
    InvalidLayoutNode {
        #[label("Invalid node type")]
        node_src: SourceSpan,
        #[help]
        help: String,
    },

    #[error("Unsupported number of children")]
    #[diagnostic()]
    InvalidNumberOfChildren {
        #[label("Node with unsupported children")]
        parent_src: SourceSpan,
        #[help]
        help: String,
    },

    #[error("Missing argument")]
    #[diagnostic()]
    MissingArgument {
        #[label("Node is missing required argument")]
        node_src: SourceSpan,
        #[help]
        help: String,
    },

    #[error("Invalid argument")]
    #[diagnostic()]
    InvalidArgument {
        #[label("Argument is invalid")]
        arg_src: SourceSpan,
        #[help]
        help: String,
    },
}

enum StyleAttribute {
    Padding(u16),
    Margin(u16),
    Spacing(u16),
    Color(iced::Color),
    Width(iced::Length),
    Height(iced::Length),
    HorizontalAlignment(iced::alignment::Horizontal),
    VerticalAlignment(iced::alignment::Vertical),
    Font,
}

enum States {
    Hovered,
    Focused,
    Active,
    Pressed,
    Disabled,
}

pub struct StyleRule {
    classes: Vec<String>,
    attributes: Vec<StyleAttribute>,
}

pub struct AppContainer {}

impl AppContainer {
    pub fn new() -> Box<dyn container::StyleSheet<Style = iced::theme::Theme>> {
        Box::new(Self {}) as Box<dyn container::StyleSheet<Style = iced::theme::Theme>>
    }
}

impl container::StyleSheet for AppContainer {
    type Style = iced::theme::Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            // border_radius: 10.0,
            // border_color: Color::BLACK,
            // border_width: 2.0,
            // background: Color::WHITE.into(),
            ..Default::default()
        }
    }
}

#[derive(Debug)]
pub struct IcedMenuTheme {
    pub window_width: u32,
    pub padding: u16,
    pub query_font_size: u16,
    pub query_padding: u16,
    pub item_font_size: u16,
    pub item_padding: u16,
    pub item_spacing: u16,
    pub highlight_matches: bool,
    pub match_highlight_color: Color,
}

impl IcedMenuTheme {
    pub fn window_height(&self, n_items: u16) -> u32 {
        (self.query_font_size
            + 2 * self.query_padding
            + n_items * (self.item_font_size + 2 * self.item_padding)
            + n_items * self.item_spacing
            + 2 * self.padding)
            .into()
    }
}

impl Default for IcedMenuTheme {
    fn default() -> Self {
        Self {
            window_width: 400,
            padding: 10,
            query_font_size: 20,
            query_padding: 10,
            item_font_size: 20,
            item_padding: 10,
            item_spacing: 10,
            highlight_matches: true,
            match_highlight_color: theme::Theme::default().palette().primary,
        }
    }
}
