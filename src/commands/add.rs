use std::path::PathBuf;

use crate::services::googlesheets::spreadsheets;
use crate::services::scryfall::{get_cards_from_set, get_set};

pub async fn add_set(
    secrets: PathBuf,
    tokens: PathBuf,
    set_codes: Vec<String>,
) -> Result<Vec<String>, String> {
    let spreadsheets = match spreadsheets(secrets, tokens).await {
        Ok(spreadsheets) => spreadsheets,
        Err(e) => return Err(format!("Can't connect to spreadsheet API: {e}")),
    };

    for set_code in &set_codes {
        println!("Adding set {}", &set_code);
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
            println!(
                "Set {} does not exists in your spreadsheet. Creating it...",
                &set_code
            );
            spreadsheets.create_new_sheet(&set_code).await;
        }

        spreadsheets.add_cards_to_extension(&set_code, cards).await;
    }

    Ok(set_codes)
}
