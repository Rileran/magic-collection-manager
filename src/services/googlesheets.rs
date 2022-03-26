use std::io::Error;
use std::path::PathBuf;

use google_sheets4::api::{
    BatchUpdateSpreadsheetRequest, DuplicateSheetRequest, DuplicateSheetResponse,
    UpdateValuesResponse, ValueRange,
};
use google_sheets4::hyper::client::HttpConnector;
use google_sheets4::hyper::Client;
use google_sheets4::hyper_rustls::HttpsConnector;
use google_sheets4::oauth2::authenticator::Authenticator;
use google_sheets4::oauth2::{self, InstalledFlowAuthenticator, InstalledFlowReturnMethod};
use google_sheets4::Sheets;

use crate::model::card::Card;

pub struct Spreadsheet {
    pub id: String,
    sheets: Sheets,
}

impl Spreadsheet {
    pub fn new(id: String, sheets: Sheets) -> Spreadsheet {
        Spreadsheet {
            id: id,
            sheets: sheets,
        }
    }

    pub async fn get_titles(&self) -> Vec<String> {
        self.sheets
            .spreadsheets()
            .get(&self.id)
            .doit()
            .await
            .unwrap()
            .1
            .sheets
            .unwrap()
            .into_iter()
            .map(|sheet| sheet.properties.unwrap().title.unwrap())
            .collect()
    }

    pub async fn create_new_sheet(&self, set_code: &String) -> DuplicateSheetResponse {
        let dupliate_request = DuplicateSheetRequest {
            insert_sheet_index: Some(0),
            new_sheet_name: Some(set_code.clone()),
            source_sheet_id: Some(422889464),
            ..Default::default()
        };

        self.sheets
            .spreadsheets()
            .batch_update(
                BatchUpdateSpreadsheetRequest {
                    requests: Some(vec![google_sheets4::api::Request {
                        duplicate_sheet: Some(dupliate_request),
                        ..Default::default()
                    }]),
                    ..Default::default()
                },
                &self.id,
            )
            .doit()
            .await
            .unwrap()
            .1
            .replies
            .unwrap()[0]
            .clone()
            .duplicate_sheet
            .unwrap()
            .clone()
    }

    pub async fn add_cards_to_extension(
        &self,
        set_code: &String,
        cards: Vec<Card>,
    ) -> UpdateValuesResponse {
        let range_end = cards.len() + 1;
        let values = ValueRange {
            values: Some(cards.into_iter().map(Card::to_vec_string).collect()),
            ..Default::default()
        };

        self.sheets
            .spreadsheets()
            .values_update(
                values,
                &self.id,
                format!("{}!A2:D{}", set_code, range_end).as_str(),
            )
            .value_input_option("USER_ENTERED")
            .doit()
            .await
            .unwrap()
            .1
    }

    pub async fn update_card_prices(
        &self,
        set_code: &String,
        cards: Vec<Card>,
    ) -> UpdateValuesResponse {
        let range_end = cards.len() + 1;
        let values = ValueRange {
            values: Some(
                cards
                    .into_iter()
                    .map(|card| {
                        vec![match card.price {
                            Some(price) => {
                                format!("{:.2}", (price as f64) / 100.0).replace(".", ",")
                            }
                            None => String::new(),
                        }]
                    })
                    .collect(),
            ),
            ..Default::default()
        };

        println!("{:?}", values);

        self.sheets
            .spreadsheets()
            .values_update(
                values,
                &self.id,
                format!("{}!D2:D{}", set_code, range_end).as_str(),
            )
            .value_input_option("USER_ENTERED")
            .doit()
            .await
            .unwrap()
            .1
    }
}

pub async fn authenticate(
    secrets: PathBuf,
    tokens: PathBuf,
) -> Result<Authenticator<HttpsConnector<HttpConnector>>, Error> {
    let secret = oauth2::read_application_secret(secrets).await?;

    let auth = InstalledFlowAuthenticator::builder(secret, InstalledFlowReturnMethod::HTTPRedirect)
        .persist_tokens_to_disk(tokens)
        .build()
        .await?;

    Ok(auth)
}

pub async fn spreadsheets(secrets: PathBuf, tokens: PathBuf) -> Result<Spreadsheet, Error> {
    let auth = authenticate(secrets, tokens).await?;

    let sheets = Sheets::new(
        Client::builder().build(HttpsConnector::with_native_roots()),
        auth,
    );

    Ok(Spreadsheet::new(
        String::from("1sWflgPPZ-TBsQ-IC1Vjq9dq6mK06VwddtOEpx3rcMG4"),
        sheets,
    ))
}
