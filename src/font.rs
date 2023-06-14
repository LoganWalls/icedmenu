use font_loader::system_fonts;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use kdl::{KdlEntry, KdlNode};
use std::collections::HashMap;

use iced::Font;

use crate::config::ConfigError;
use crate::layout::style::string_value;

pub struct FontLoader {
    lookup: HashMap<&'static str, Font>,
}

impl FontLoader {
    pub fn new() -> Self {
        FontLoader {
            lookup: HashMap::new(),
        }
    }

    pub fn get(&self, font_name: &str) -> Font {
        *self.lookup.get(font_name).unwrap_or_else(|| {
            panic!("Attempted to access font: {font_name}, which has not been loaded")
        })
    }

    pub fn ensure_font<'a>(
        &mut self,
        node: &KdlNode,
        font_name: &'a KdlEntry,
    ) -> Result<&'a str, ConfigError> {
        let name = string_value(node, font_name)?;
        if self.lookup.contains_key(name) {
            return Ok(name);
        }
        let font_props = system_fonts::FontPropertyBuilder::new()
            .family(name)
            .build();
        let (bytes, _) =
            system_fonts::get(&font_props).ok_or_else(|| ConfigError::FontNotFound {
                value_src: *font_name.span(),
                help: format!(
            "Could not find this font on your system.\nSimilar fonts that you have installed: {}",
            similar_system_fonts(name, 5).join(", "),
        ),
            })?;
        let static_name = Box::leak(Box::new((*name).to_owned()));
        let font = Font::External {
            name: static_name,
            bytes: Box::leak(bytes.into_boxed_slice()),
        };
        self.lookup.insert(static_name, font);
        Ok(static_name)
    }
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
