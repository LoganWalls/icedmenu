use iced::widget::container;
use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

pub const LAYOUT_KEY: &str = "Layout";
pub const STYLES_KEY: &str = "Styles";

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

    #[error("Unsupported children")]
    #[diagnostic()]
    InvalidChildren {
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

    #[error("Empty style")]
    #[diagnostic()]
    EmptyStyle {
        #[label("Style attribute")]
        attr_src: SourceSpan,
        #[help]
        help: String,
    },

    #[error("Invalid style")]
    #[diagnostic()]
    InvalidStyleAttribute {
        #[label("Style attribute")]
        attr_src: SourceSpan,
        #[help]
        help: String,
    },

    #[error("Invalid attribute value")]
    #[diagnostic()]
    InvalidValue {
        #[label("Style attribute")]
        attr_src: SourceSpan,
        #[label("Value")]
        value_src: SourceSpan,
        #[help]
        help: String,
    },

    #[error("Font not found")]
    #[diagnostic()]
    FontNotFound {
        #[label("Missing font")]
        value_src: SourceSpan,
        #[help]
        help: String,
    },
}

pub struct AppContainer;

impl AppContainer {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Box<dyn container::StyleSheet<Style = iced::theme::Theme>> {
        Box::new(Self {}) as Box<dyn container::StyleSheet<Style = iced::theme::Theme>>
    }
}

impl container::StyleSheet for AppContainer {
    type Style = iced::theme::Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        _style.appearance(&iced::theme::Container::default())
    }
}
