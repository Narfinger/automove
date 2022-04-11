use std::{
    fs::{self, read_to_string},
    io,
    path::PathBuf,
};

use ansi_term::Color::{Blue, Green, Red};
use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use serde_derive::{Deserialize, Serialize};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Modify the moves.toml
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Adds to current moves
    Add { pattern: String, path: String },

    /// List all current moves
    List,

    /// Removes an entry with a given match
    Delete { pattern: String },
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    moves: Vec<Move>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
struct Move {
    pattern: String,
    path: String,
    directory: Option<bool>,
}

fn save_config(config: &Config) -> Result<()> {
    let mut config_filename = std::env::current_dir().context("Could not find current dir")?;
    config_filename.push("move.toml");
    let toml = toml::to_string_pretty(config)?;
    fs::write(&config_filename, toml)?;

    Ok(())
}

fn list_config(config: &Config) -> Result<()> {
    for i in &config.moves {
        println!("({} ->  {})", Green.paint(&i.pattern), Blue.paint(&i.path));
    }
    println!();
    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut config: Config = {
        let mut config_filename = std::env::current_dir().context("Could not find current dir")?;
        config_filename.push("move.toml");
        let config_file = read_to_string(&config_filename)
            .with_context(|| format!("could not open config file at: {:?}", &config_filename))?;
        toml::from_str(&config_file)?
    };

    match cli.command {
        Some(Commands::Add { pattern, path }) => {
            let new = Move {
                pattern,
                path,
                directory: None,
            };
            config.moves.push(new);
            save_config(&config)?;
            list_config(&config)?
        }
        Some(Commands::List) => list_config(&config)?,
        Some(Commands::Delete { pattern }) => {
            config.moves.retain(|m: &Move| m.pattern != pattern);
            save_config(&config)?;
            list_config(&config)?
        }
        None => move_files(config)?,
    };

    Ok(())
}

fn move_files(config: Config) -> Result<(), anyhow::Error> {
    let cur_dir = std::env::current_dir().context("Error in getting current directory")?;
    let entries = fs::read_dir(cur_dir)
        .context("Error in reading current directory")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;
    //println!("Entries {:?}", entries);
    let files = entries;
    //let files: Vec<&PathBuf> = entries.iter().collect();
    if files.is_empty() {
        println!("Nothing to move");
    }
    for i in files {
        for m in config.moves.iter() {
            if let Some(filename) = i.file_name().and_then(|s| s.to_str()) {
                if filename.contains(&m.pattern)
                    && ((i.is_file() && !m.directory.unwrap_or(false))
                        || (m.directory.unwrap_or(false) && i.is_dir()))
                {
                    println!(
                        "Matching {} with {}",
                        Green.paint(filename),
                        Blue.paint(&m.pattern)
                    );
                    let mut to = PathBuf::from(&m.path);
                    if to.exists() && to.is_dir() {
                        to.push(&filename);
                        println!("Moving to: {:?}", to);
                        std::fs::rename(&i, &to).with_context(|| {
                            format!(
                                "Moving file from {} to {} did not succeed",
                                Green.paint(filename),
                                Blue.paint(to.to_string_lossy())
                            )
                        })?;
                    } else {
                        println!(
                            "Skipping {} as it does not exist or is not a directory",
                            Red.paint(to.to_string_lossy())
                        );
                    }
                }
            } else {
                println!(
                    "Skipping filename {:?} because conversion did not work",
                    i.file_name()
                );
            }
        }
    }
    Ok(())
}
