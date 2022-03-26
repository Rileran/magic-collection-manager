use std::fmt;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Rarity {
    COMMON,
    UNCOMMON,
    RARE,
    MYTHIC,
    SPECIAL,
}

impl fmt::Display for Rarity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Rarity::COMMON => write!(f, "C"),
            Rarity::UNCOMMON => write!(f, "U"),
            Rarity::RARE => write!(f, "R"),
            Rarity::MYTHIC => write!(f, "M"),
            Rarity::SPECIAL => write!(f, "S"),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Card {
    pub name: String,
    pub set: String,
    pub collector_number: String,
    pub price: Option<usize>,
    pub rarity: Rarity,
}

impl Card {
    pub fn to_vec_string(self) -> Vec<String> {
        let price = match self.price {
            Some(price) => format!("{:.2}", (price as f64) / 100.0).replace(".", ","),
            None => String::new(),
        };

        vec![
            self.collector_number,
            self.rarity.to_string(),
            self.name,
            price,
        ]
    }
}
