use std::path::PathBuf;

use crate::google::authenticate;

pub async fn link(secrets: PathBuf, tokens: PathBuf) -> Result<(), String> {
    let auth = match authenticate(secrets, tokens).await {
        Ok(auth) => auth,
        Err(e) => return Err(format!("Please provide a secret file: {e}")),
    };

    let scopes = &["https://www.googleapis.com/auth/spreadsheets"];

    match auth.token(scopes).await {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("{e}")),
    }
}
