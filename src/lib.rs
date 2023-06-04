pub use icedmenu_derive::{FromGenericStyle, Reflective};

pub trait Reflective {
    fn reflect_attr_names() -> Vec<&'static str>;
}
