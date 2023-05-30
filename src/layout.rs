use iced::Element;
use kdl::KdlNode;

use crate::app::{IcedMenu, Message};
use crate::config::ConfigError;

use self::style::{GenericStyle, StyleLookup};

pub mod column;
pub mod container;
pub mod items;
pub mod query;
pub mod row;
pub mod style;
pub mod text;

#[derive(Debug)]
pub struct NodeData {
    pub children: Vec<LayoutNode>,
    pub style: GenericStyle,
}

#[derive(Debug)]
pub enum LayoutNode {
    Container(NodeData),
    Row(NodeData),
    Column(NodeData),
    Query(NodeData),
    Items(NodeData),
    Text(text::TextNodeData),
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
    pub fn new(node: &KdlNode, styles: &StyleLookup) -> Result<Self, ConfigError> {
        let children = node
            .children()
            .iter()
            .map(|d| d.nodes())
            .flatten()
            .map(|c| Self::new(c, styles))
            .collect::<Result<Vec<_>, _>>()?;
        let classes = node
            .entries()
            .iter()
            .filter_map(|e| e.name().map(|n| n.value()))
            .collect();

        let node_type = node.name().value();
        let style = styles.style_for(classes, node_type);

        match node_type {
            "Container" | "Layout" => container::new(node, children, style),
            "Row" => row::new(node, children, style),
            "Column" => column::new(node, children, style),
            "Text" => text::new(node, children, style),
            "Query" => query::new(node, children, style),
            "Items" => items::new(node, children, style),
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
