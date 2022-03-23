use std::path::PathBuf;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub token_file: Option<PathBuf>,
    pub secret_file: Option<PathBuf>,
}
