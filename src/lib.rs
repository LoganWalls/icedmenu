pub use icedmenu_derive::{Reflective, UpdateFromOther};

pub trait Reflective {
    fn reflect_attr_names() -> Vec<&'static str>;
}

pub trait UpdateFromOther {
    fn update_from(&mut self, other: &Self);
}

#[macro_export]
macro_rules! define_theme {
    ($widget_name:ident, $appearance:ty, $stylesheet:ty, $theme_enum:ty; $($attr:ident),*; $($aliased_attr:ident : $alias:ident),* $(,)?) => {

            struct $widget_name {
                style: crate::layout::style::GenericStyle,
            }

            impl $widget_name {
                fn new(style: crate::layout::style::GenericStyle) -> $theme_enum {
                    <$theme_enum>::Custom(Box::from(Self { style }))
                }
            }

            impl $stylesheet for $widget_name {
                type Style = iced::Theme;

                fn appearance(&self, _: &Self::Style) -> $appearance {
                    let mut result = <$appearance>::default();
                    $(
                        if let Some(v) = self.style.$attr {
                            result.$attr = v;
                        }
                    )*
                    $(
                        if let Some(v) = self.style.$alias {
                            result.$aliased_attr = v;
                        }
                    )*
                    result
                }
           }

    };
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
