use iced::{theme, widget::container, Color};
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

// pub struct StyleRule {
//     classes: Vec<String>,
//     attributes: Vec<StyleAttribute>,
// }

pub struct AppContainer {}

impl AppContainer {
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
