use kdl::KdlNode;

use super::{LayoutNode, LayoutNodeData};
use crate::style::ConfigError;

pub fn new(
    node: &KdlNode,
    children: Vec<LayoutNode>,
    classes: Vec<String>,
) -> Result<LayoutNode, ConfigError> {
    Ok(LayoutNode::Column(LayoutNodeData { children, classes }))
}
