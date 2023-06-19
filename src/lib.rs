pub use icedmenu_derive::{Reflective, UpdateFromOther};

pub trait Reflective {
    fn reflect_attr_names() -> Vec<&'static str>;
}

pub trait UpdateFromOther {
    fn update_from(&mut self, other: &Self);
}

#[macro_export]
macro_rules! get_item_style {
    ($item:ident, $item_data:ident, $menu:ident) => {
        match (
            $menu.visible_items[$menu.cursor_position] == $item.index,
            $item.selected,
        ) {
            (true, true) => {
                let mut s: GenericStyle = $item_data.selected_style;
                s.update_from(&$item_data.hovered_style);
                s
            }
            (true, false) => $item_data.hovered_style,
            (false, true) => $item_data.selected_style,
            (false, false) => $item_data.style,
        }
    };
}

#[macro_export]
macro_rules! apply_height_styles {
    ($value:expr, $style:ident) => {{
        let mut result = $value;
        if let Some(iced::Length::Fixed(height)) = $style.height {
            result = height as u32;
        }
        if let Some(max_height) = $style.max_height {
            result = std::cmp::min(result, max_height as u32)
        }
        result
    }};
}

#[macro_export]
macro_rules! apply_width_styles {
    ($value:expr, $style:ident) => {{
        let mut result = $value;
        if let Some(iced::Length::Fixed(width)) = $style.width {
            result = width as u32;
        }
        if let Some(max_width) = $style.max_width {
            result = std::cmp::min(result, max_width as u32)
        }
        result
    }};
}

#[macro_export]
macro_rules! apply_styles {
    ($widget:ident, $style:ident; $($attr:ident),*; $($f:ident : $alias:ident),* $(,)?) => {
        {
            let mut result = $widget;
            let style = $style;
            $(
                if let Some(v) = style.$attr {
                    result = result.$attr(v);
                }
            )*
            $(
                if let Some(v) = style.$alias {
                    result = result.$f(v);
                }
            )*
            result
        }
    };
}
