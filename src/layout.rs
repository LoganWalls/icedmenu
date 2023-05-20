use kdl::KdlNode;

use crate::style::ConfigError;

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
