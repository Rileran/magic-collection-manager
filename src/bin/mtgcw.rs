use std::path::PathBuf;

use dotenv::dotenv;

use clap::{Parser, Subcommand};
use magic_collection_manager::commands::add::add_set;
use magic_collection_manager::commands::link::link;
use magic_collection_manager::commands::update::update_set;
use magic_collection_manager::config::Config;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let cli = Cli::parse();
    let config = match envy::from_env::<Config>() {
        Ok(config) => config,
        Err(error) => panic!("{:?}", error),
    };

    let secret_file = config.secret_file.unwrap_or(cli.secret_file);
    let token_file = config.token_file.unwrap_or(cli.token_file);

    match cli.command {
        Commands::Add { set_codes } => {
            match tokio::spawn(add_set(secret_file, token_file, set_codes))
                .await
                .unwrap()
            {
                Ok(set_codes) => {
                    println!("Successfully add set {:?} to your spreadsheet.", set_codes)
                }
                Err(e) => println!("An error occured while adding the set: {e}"),
            }
        }
        Commands::Update { set_codes } => {
            match tokio::spawn(update_set(secret_file, token_file, set_codes))
                .await
                .unwrap()
            {
                Ok(set_codes) => {
                    println!("Successfully add set {:?} to your spreadsheet.", set_codes)
                }
                Err(e) => println!("An error occured while adding the set: {e}"),
            }
        }
        Commands::Link => {
            match tokio::spawn(link(secret_file, token_file)).await.unwrap() {
                Ok(()) => println!("Successfully linked to your google account."),
                Err(e) => println!("An error occured when linking your account: {e}"),
            };
        }
    }
}

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    #[clap(short, long)]
    pub verbose: bool,

    /// Secret json file from Google Cloud Plateform
    #[clap(short = 'f', long, default_value = "secrets.json")]
    pub secret_file: PathBuf,

    /// Where your oauth2 tokens will be saved
    #[clap(short, long, default_value = "tokens.json")]
    pub token_file: PathBuf,

    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a set to the collection
    Add {
        /// Three letter set code to add to the collection
        #[clap(validator = validate_set_code)]
        set_codes: Vec<String>,
    },
    /// Update a set pricing
    Update {
        /// Three letter set codes to update the price
        #[clap(validator = validate_set_code)]
        set_codes: Vec<String>,
    },
    /// Link the program with your google sheet
    Link,
}

fn validate_set_code(s: &str) -> Result<(), String> {
    match s.len() == 3 {
        true => Ok(()),
        false => Err(format!("Set code is not 3 character long")),
    }
}
