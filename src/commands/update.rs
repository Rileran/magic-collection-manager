use std::path::PathBuf;

use crate::services::googlesheets::spreadsheets;
use crate::services::scryfall::{get_cards_from_set, get_set};

pub async fn update_set(
    secrets: PathBuf,
    tokens: PathBuf,
    set_codes: Vec<String>,
) -> Result<Vec<String>, String> {
    let spreadsheets = match spreadsheets(secrets, tokens).await {
        Ok(spreadsheets) => spreadsheets,
        Err(e) => return Err(format!("Can't connect to spreadsheet API: {e}")),
    };

    for set_code in &set_codes {
        println!("Updating set {}", &set_code);
        let set = match get_set(&set_code).await {
            Ok(set) => set,
            Err(e) => return Err(format!("{e}")),
        };

        let cards = match get_cards_from_set(set).await {
            Ok(cards) => cards,
            Err(e) => return Err(format!("{e}")),
        };

        let titles: Vec<String> = spreadsheets.get_titles().await;

        if !titles.contains(&set_code) {
            panic!(
                "Set {} does not exists in your spreadsheet. Creating it first using add command.",
                &set_code
            )
        }

        spreadsheets.update_card_prices(&set_code, cards).await;
    }
    Ok(set_codes)
}
