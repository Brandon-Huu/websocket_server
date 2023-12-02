#![allow(unused, deprecated)]
mod main_program;
use anyhow::{anyhow, Error, Result};
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use tokio::*;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
#[non_exhaustive]
enum Commands {
    /// Adds files to myapp
    AddKey {
        name: String,
        #[clap(flatten)]
        key: KeyFileSource,
    },
    RemoveKey {
        name: String,
    },
    Run,
}

#[derive(Debug, clap::Args)]
#[group(required = true, multiple = false)]
pub struct KeyFileSource {
    #[clap(short, long)]
    filepath: Option<PathBuf>,

    #[clap(short, long)]
    url: Option<String>,
}
#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::AddKey { name, key } => add_key(name, key),
        Commands::RemoveKey { name } => remove_key(name),
        Commands::Run => main_program::run().await,
    }
}

fn add_key(name: &str, key: &KeyFileSource) -> Result<()> {
    if let Some(path) = &key.filepath {
        add_key_file(name, &path)?;
    }
    if let Some(url) = &key.url {
        add_key_url(name, &url)?;
    }
    Ok(())
}

fn add_key_file(name: &str, key: &PathBuf) -> Result<()> {
    match key.try_exists() {
        Ok(true) => (),
        //Couldn't find original keyfile
        _ => return Err(anyhow!("Could not find the provided source key")),
    };

    let mut destination = std::env::home_dir().unwrap();
    destination.push(".websocket_server");
    destination.push("public_keys");
    destination.push(format!("{}.pub", name));

    match destination.try_exists() {
        //File exists or error
        Ok(true) | Err(_) => return Err(anyhow!("That public key already exists")),
        _ => (),
    };

    std::fs::create_dir_all(&destination.parent().unwrap())?;
    std::fs::copy(&key, &destination)?;
    Ok(())
}
fn add_key_url(name: &str, key: &str) -> Result<()> {
    todo!()
}
fn remove_key(name: &str) -> Result<()> {
    let mut destination = std::env::home_dir().unwrap();
    destination.push(".websocket_server");
    destination.push("public_keys");
    destination.push(format!("{}.pub", name));

    std::fs::remove_file(&destination)?;
    Ok(())
}
