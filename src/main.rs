mod commands;

use clap::{load_yaml, App};
use color_eyre::Result;
use commands::*;
use std::str::FromStr;
use tracing::error;
use tracing_subscriber::EnvFilter;

pub enum Commands {
    JsToTs,
}

impl FromStr for Commands {
    type Err = ();
    fn from_str(input: &str) -> Result<Commands, Self::Err> {
        match input {
            "js-to-ts" => Ok(Commands::JsToTs),
            _ => Err(()),
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    setup()?;

    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();

    match matches.subcommand_name() {
        Some(subcommand_name) => {
            let args = matches.subcommand_matches(subcommand_name).unwrap();
            match Commands::from_str(subcommand_name).unwrap() {
                Commands::JsToTs => {
                    js_to_ts::convert(
                        args.value_of("directory").unwrap_or_default().into(),
                    ).await;
                }
            }
        }
        None => error!("No command provided"),
    };

    Ok(())
}

fn setup() -> Result<()> {
    if std::env::var("RUST_BACKTRACE").is_err() {
        std::env::set_var("RUST_BACKTRACE", "1")
    }
    color_eyre::install()?;

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }
    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    Ok(())
}