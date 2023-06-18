use iced::Element;
use icedmenu::{Reflective, UpdateFromOther};
use kdl::KdlNode;

use crate::app::{IcedMenu, Message};
use crate::config::ConfigError;
use crate::item::Item;

use self::items::ItemsNodeData;
use self::style::{GenericStyle, State, StyleLookup};

pub mod column;
pub mod container;
pub mod item_key;
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

#[derive(Debug, Reflective)]
pub enum LayoutNode {
    Container(container::ContainerNodeData),
    Row(NodeData),
    Column(NodeData),
    Query(query::QueryNodeData),
    Items(ItemsNodeData),
    ItemKey(NodeData),
    Text(Box<text::TextNodeData>),
}

impl LayoutNode {
    pub fn new(node: &KdlNode, style_lookup: &StyleLookup) -> Result<Self, ConfigError> {
        let node_type = node.name().value();
        let children = node
            .children()
            .iter()
            .flat_map(|d| d.nodes())
            .map(|child| {
                let c = Self::new(child, style_lookup)?;
                match (node_type, &c) {
                    ("Row" | "Column" | "Col", Self::Items(_)) => Ok(c),
                    (_, Self::Items(_)) => Err(
                        ConfigError::InvalidChildren { 
                            parent_src: *node.span(), 
                            help: format!("{node_type} cannot be the parent of Items. Parent must be Row or Column") 
                        }),
                    _ => Ok(c),
                }
            })
            .collect::<Result<Vec<_>, _>>()?;

        let style_names: Vec<&str> = node
            .entries()
            .iter()
            .filter_map(|e| match e.name() {
                None => e.value().as_string(),
                _ => None,
            })
            .collect();
        let style = style_lookup.style_for(&style_names, node_type, State::Default);
        match node_type {
            "Container" | "Layout" => container::new(node, children, style),
            "Row" => row::new(children, style),
            "Column" | "Col" => column::new(children, style),
            "Text" => text::new(node, children, style),
            "Query" => {
                let mut focused_style = style.clone();
                focused_style.update_from(&style_lookup.style_for(
                    &style_names,
                    node_type,
                    State::Focused,
                ));
                let mut hovered_style = style.clone();
                hovered_style.update_from(&style_lookup.style_for(
                    &style_names,
                    node_type,
                    State::Hovered,
                ));
                query::new(node, children, style, focused_style, hovered_style)
            }
            "Items" => {
                let mut hovered_style = style.clone();
                hovered_style.update_from(&style_lookup.style_for(
                    &style_names,
                    node_type,
                    State::Hovered,
                ));
                let mut pressed_style = style.clone();
                pressed_style.update_from(&style_lookup.style_for(
                    &style_names,
                    node_type,
                    State::Pressed,
                ));
                items::new(node, children, style, hovered_style, pressed_style)
            }
            "ItemKey" => item_key::new(node, children, style),
            _ => Err(ConfigError::InvalidLayoutNode {
                node_src: *node.name().span(),
                help: format!(
                    "Try changing this node to one of: {}",
                    Self::reflect_attr_names().join(",")
                ),
            }),
        }
    }

    pub fn view<'a>(
        node: &'a Self,
        menu: &'a IcedMenu,
        item: Option<&'a Item>,
    ) -> Element<'a, Message> {
        match node {
            Self::Container(data) => container::view(data, menu, item),
            Self::Row(data) => row::view(data, menu, item),
            Self::Column(data) => column::view(data, menu, item),
            Self::Query(data) => query::view(data, menu),
            Self::Text(data) => text::view(data),
            Self::ItemKey(data) => item_key::view(data, item),
            Self::Items(_) => {
                // Layouts are validated so that Items must be the child of a Row or Column
                // (which call item::view() directly) so this branch should never be reached
                unreachable!()
            }
        }
    }
}

fn validate_children(
    node: &KdlNode,
    n_children: usize,
    constraint: usize,
) -> Result<(), ConfigError> {
    if constraint != n_children {
        return Err(ConfigError::InvalidChildren {
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
    Ok(())
}
