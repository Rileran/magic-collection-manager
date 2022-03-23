use std::path::PathBuf;

use crate::google;

pub async fn add_set(secrets: PathBuf, tokens: PathBuf, set: String) -> Result<String, String> {
    let spreadsheets = match google::spreadsheets(secrets, tokens).await {
        Ok(spreadsheets) => spreadsheets,
        Err(e) => return Err(format!("Can't connect to spreadsheet API: {e}")),
    };

    Ok(set)
}
