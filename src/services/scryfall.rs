use reqwest::{self, Error};
use serde::Deserialize;

use crate::model::card::{Card, Rarity};
use crate::model::set::Set;

const SCRYFALL_SET_ENDPOINT: &str = "https://api.scryfall.com/sets";

pub async fn get_set(set: &String) -> Result<Set, Error> {
    let response = reqwest::get(format!(
        "{}/{}",
        SCRYFALL_SET_ENDPOINT,
        set.to_ascii_lowercase()
    ))
    .await?
    .json::<Set>()
    .await?;

    Ok(response)
}

#[derive(Deserialize, Debug)]
struct SearchResponse {
    has_more: bool,
    next_page: Option<String>,
    data: Vec<CardResponse>,
}

#[derive(Deserialize, Debug)]
struct CardResponse {
    name: String,
    set: String,
    collector_number: String,
    prices: CardPricesResponse,
    rarity: Rarity,
}

#[derive(Deserialize, Debug)]
struct CardPricesResponse {
    eur: Option<String>,
}

impl Into<Card> for CardResponse {
    fn into(self) -> Card {
        Card {
            name: self.name,
            set: self.set,
            collector_number: self.collector_number,
            price: self
                .prices
                .eur
                .map(|eur| (eur.parse::<f64>().unwrap() * 100.0) as usize),
            rarity: self.rarity,
        }
    }
}

pub async fn get_cards_from_set(set: Set) -> Result<Vec<Card>, Error> {
    let mut cards: Vec<CardResponse> = Vec::with_capacity(set.card_count);

    let mut next_page = set.search_uri;
    loop {
        let mut search_response = reqwest::get(next_page)
            .await?
            .json::<SearchResponse>()
            .await?;

        cards.append(&mut search_response.data);

        if !search_response.has_more {
            break;
        }
        next_page = search_response.next_page.unwrap();
    }

    Ok(cards.into_iter().map(CardResponse::into).collect())
}
