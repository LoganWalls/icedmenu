use iced::{theme, Color};

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
