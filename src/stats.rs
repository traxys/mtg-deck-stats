use crate::Category;
use snafu::Snafu;

fn factorial_fraction(a: u64, k: u64) -> f64 {
    let mut res = 1.0;
    for x in (a - k + 1)..=a {
        res *= x as f64
    }
    res
}

fn calculate_turns(cat_size: u64, deck_size: u64, until_turn: u64) -> Vec<f64> {
    if until_turn + 7 > deck_size || cat_size > deck_size {
        Vec::new()
    } else {
        let mut turns = Vec::new();
        let k_factor = 1.0 / factorial_fraction(deck_size, cat_size);
        for n in 7..=(until_turn + 7) {
            turns.push(1.0 - (factorial_fraction(deck_size - n, cat_size) * k_factor))
        }
        turns
    }
}

pub type TurnStats<'a> = Vec<(&'a Category, Vec<f64>)>;

fn commander_with_categories(categories: &[Category], until_turn: u64) -> TurnStats<'_> {
    categories
        .iter()
        .map(|category| (category, calculate_turns(category.size, 99, until_turn)))
        .collect()
}

#[derive(Debug)]
pub enum GameFormat {
    Commander,
    Standard,
}

#[derive(Debug, Snafu)]
pub enum GameFormatParseError {
    #[snafu(display("Invalid format: {}", format))]
    UnknownFormat { format: String },
}

impl std::str::FromStr for GameFormat {
    type Err = GameFormatParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "commander" | "edh" => Ok(Self::Commander),
            "standard" | "modern" => Ok(Self::Standard),
            s => Err(GameFormatParseError::UnknownFormat {
                format: s.to_owned(),
            }),
        }
    }
}

impl GameFormat {
    pub fn stats<'a>(&self, categories: &'a [Category], turns: u64) -> TurnStats<'a> {
        match self {
            Self::Commander => commander_with_categories(categories, turns),
            Self::Standard => unimplemented!(),
        }
    }
}
