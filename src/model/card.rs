use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Rarity {
    COMMON,
    UNCOMMON,
    RARE,
    MYTHIC,
}

#[derive(Deserialize, Debug)]
pub struct Card {
    pub name: String,
    pub set: String,
    pub collector_number: usize,
    pub price: Option<usize>,
    pub rarity: Rarity,
}
