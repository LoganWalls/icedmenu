use kdl::KdlNode;

use super::{LayoutNode, LayoutNodeData};
use crate::style::ConfigError;

pub fn new(
    node: &KdlNode,
    children: Vec<LayoutNode>,
    classes: Vec<String>,
) -> Result<LayoutNode, ConfigError> {
    super::validate_children(node, children.len(), 0)?;
    Ok(LayoutNode::Query(LayoutNodeData { children, classes }))
}
