use std::cmp::{Ord, Ordering};
use std::{error::Error, io};

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, serde::Deserialize, serde::Serialize)]
pub struct ItemData {
    pub key: String,
    #[serde(default)]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Item {
    pub index: usize,
    pub data: ItemData,
    pub score: Option<u32>,
    pub match_indices: Option<Vec<usize>>,
    pub selected: bool,
}

impl Item {
    pub fn new(index: usize, data: ItemData) -> Self {
        Self {
            index,
            data,
            score: None,
            match_indices: None,
            selected: false,
        }
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.score, other.score) {
            // Sort by score
            (Some(a), Some(b)) => a.cmp(&b),
            // Items with a score should be above those without
            (Some(_), _) => Ordering::Greater,
            (None, Some(_)) => Ordering::Less,
            // Fallback to the order of the items in the input
            (_, _) => self.index.cmp(&other.index).reverse(),
        }
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn parse_items(source: impl io::Read) -> Result<Vec<Item>, Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(source);
    let mut result = Vec::new();
    for (i, data) in rdr.deserialize::<ItemData>().enumerate() {
        result.push(Item::new(i, data?));
    }
    Ok(result)
}
