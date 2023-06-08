pub use icedmenu_derive::{Reflective, UpdateFromOther};

pub trait Reflective {
    fn reflect_attr_names() -> Vec<&'static str>;
}

pub trait UpdateFromOther {
    fn update_from(&mut self, other: &Self);
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
