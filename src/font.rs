use font_loader::system_fonts;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use kdl::{KdlEntry, KdlNode};
use once_cell::sync::OnceCell;
use std::collections::HashMap;

use iced::Font;

use crate::config::ConfigError;

static FONT_LOOKUP: OnceCell<HashMap<&'static str, Font>> = OnceCell::new();

pub fn initialize_fonts(font_names: Vec<(&KdlNode, &KdlEntry)>) -> Result<(), ConfigError> {
    let mut lookup: HashMap<&'static str, Font> = HashMap::new();
    for (node, font_name) in font_names {
        let name = font_name
            .value()
            .as_string()
            .ok_or_else(|| ConfigError::InvalidValue {
                attr_src: *node.span(),
                value_src: *font_name.span(),
                help: "Font values should be strings".to_string(),
            })?
            .to_owned();
        let font_props = system_fonts::FontPropertyBuilder::new()
            .family(&name)
            .build();
        let (bytes, _) =
            system_fonts::get(&font_props).ok_or_else(|| ConfigError::FontNotFound {
                value_src: *font_name.span(),
                help: format!(
            "Could not find this font on your system.\nSimilar fonts that you have installed: {}",
            similar_system_fonts(&name, 5).join(", "),
        ),
            })?;
        let static_name = Box::leak(Box::new(name));
        let font = Font::External {
            name: static_name,
            bytes: Box::leak(bytes.into_boxed_slice()),
        };
        lookup.insert(static_name, font);
    }
    FONT_LOOKUP.set(lookup).unwrap();
    Ok(())
}

pub fn get_font(font_name: &str) -> Font {
    *FONT_LOOKUP
        .get()
        .expect("Font looked up before FONT_LOOKUP initialized")
        .get(font_name)
        .expect("Font not in lookup")
}

fn similar_system_fonts(pattern: &str, n: usize) -> Vec<String> {
    let matcher = SkimMatcherV2::default().ignore_case();
    let mut result: Vec<_> = system_fonts::query_all()
        .iter()
        .filter_map(|name| {
            matcher
                .fuzzy_match(name, pattern)
                .map(|score| (name.to_owned(), score))
        })
        .collect();
    dbg!(system_fonts::query_all());
    dbg!(pattern);
    result.sort_by_key(|(_, score)| *score);
    result
        .iter()
        .take(n)
        .map(|(name, _)| name.to_owned())
        .collect()
}
