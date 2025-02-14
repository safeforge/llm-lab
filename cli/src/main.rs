use anyhow::{bail, Result};
use clap::{Parser, Subcommand};
use dotenvy::dotenv;
use log::error;
use std::process;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// execute example command
    Example { id: String },
}

fn main() {
    // load all environment variables from .env before doing anything
    dotenv().ok();

    // Initializes the global logger with an env logger.
    env_logger::init();

    if let Err(e) = run(Cli::parse()) {
        error!("{:?}", e);
        process::exit(1);
    }
}

fn run(args: Cli) -> Result<()> {
    match args.command {
        Command::Example { id } => {
            let example = examples::REGISTER.get(&id[..]);

            match example {
                Some(example) => example.main(),
                None => bail!("Example {id} not found"),
            }
        }
    }
}
