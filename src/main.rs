use clap::{Parser, Subcommand};

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add { set } => {
            println!("Add set {set}")
        }
        Commands::Update { set } => {
            println!("Update set {:?}", set)
        }
        Commands::Link => {
            println!("Linking with google sheet")
        }
    }
}

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    #[clap(short, long)]
    pub verbose: bool,

    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a set to the collection
    Add {
        /// Three letter set code to add to the collection
        #[clap(validator = validate_set_code)]
        set: String,
    },
    /// Update a set pricing
    Update {
        /// List of three letter set codes to update the price
        #[clap(validator = validate_set_code)]
        set: Vec<String>,
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
