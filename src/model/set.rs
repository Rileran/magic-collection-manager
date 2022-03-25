use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Set {
    #[serde(rename = "code")]
    pub set: String,
    pub name: String,
    pub card_count: usize,
    pub search_uri: String,
}
