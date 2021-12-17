use std::{
    fs::{self, read_to_string},
    io,
    path::PathBuf,
};

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
    for i in entries
        .iter()
        .filter(|p| !p.is_dir())
        .filter(|p| p.extension().map_or(false, |f| f == "mkv"))
    {
        for m in config.moves.iter() {
            if let Some(filename) = i.file_name().and_then(|s| s.to_str()) {
                if filename.contains(&m.pattern) {
                    println!("Matching {} with {}", &filename, &m.pattern);
                    let mut to = PathBuf::from(&m.path);
                    to.push(&filename);
                    println!("Moving to: {:?}", to);
                    std::fs::rename(&i, &to).with_context(|| {
                        format!(
                            "Moving file from {} to {:?} did not succeed",
                            &filename, &to
                        )
                    })?;
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
