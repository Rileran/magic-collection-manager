use std::io::Error;
use std::path::PathBuf;

use google_sheets4::hyper::client::HttpConnector;
use google_sheets4::hyper::Client;
use google_sheets4::hyper_rustls::HttpsConnector;
use google_sheets4::oauth2::authenticator::Authenticator;
use google_sheets4::oauth2::{self, InstalledFlowAuthenticator, InstalledFlowReturnMethod};
use google_sheets4::Sheets;

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

pub async fn spreadsheets(secrets: PathBuf, tokens: PathBuf) -> Result<Sheets, Error> {
    let auth = authenticate(secrets, tokens).await?;

    Ok(Sheets::new(
        Client::builder().build(HttpsConnector::with_native_roots()),
        auth,
    ))
}
