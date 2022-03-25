use std::path::PathBuf;

use crate::services::googlesheets::spreadsheets;
use crate::services::scryfall::{get_cards_from_set, get_set};

pub async fn add_set(
    secrets: PathBuf,
    tokens: PathBuf,
    set_code: String,
) -> Result<String, String> {
    let set = match get_set(&set_code).await {
        Ok(set) => set,
        Err(e) => return Err(format!("{e}")),
    };

    let cards = match get_cards_from_set(set).await {
        Ok(cards) => cards,
        Err(e) => return Err(format!("{e}")),
    };

    println!("{:?}", cards);

    let _spreadsheets = match spreadsheets(secrets, tokens).await {
        Ok(spreadsheets) => spreadsheets,
        Err(e) => return Err(format!("Can't connect to spreadsheet API: {e}")),
    };

    Ok(set_code)
}
