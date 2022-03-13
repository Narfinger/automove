use std::{
    fs::{self, read_to_string},
    io,
    path::PathBuf,
};

use ansi_term::Color::{Blue, Green, Red};
use anyhow::{Context, Result};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    moves: Vec<Move>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Move {
    pattern: String,
    path: String,
}

fn main() -> Result<()> {
    let config: Config = {
        let mut config_filename = std::env::current_dir().context("Could not find current dir")?;
        config_filename.push("move.toml");
        let config_file = read_to_string(&config_filename)
            .with_context(|| format!("could not open config file at: {:?}", &config_filename))?;
        toml::from_str(&config_file)?
    };

    let cur_dir = std::env::current_dir().context("Error in getting current directory")?;
    let entries = fs::read_dir(cur_dir)
        .context("Error in reading current directory")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    //println!("Entries {:?}", entries);
    let files: Vec<&PathBuf> = entries
        .iter()
        .filter(|p| !p.is_dir())
        .filter(|p| p.extension().map_or(false, |f| f == "mkv"))
        .collect();
    if files.is_empty() {
        println!("Nothing to move");
    }
    for i in files {
        for m in config.moves.iter() {
            if let Some(filename) = i.file_name().and_then(|s| s.to_str()) {
                if filename.contains(&m.pattern) {
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
