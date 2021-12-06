use std::{
    fs::{self, read_to_string},
    io,
    path::PathBuf,
};

use anyhow::Result;
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
        let config_file = read_to_string("move.toml")?;
        toml::from_str(&config_file)?
    };

    println!("{:?}", config);

    let cur_dir = std::env::current_dir()?;
    let entries = fs::read_dir(cur_dir)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    for i in entries
        .iter()
        .filter(|p| !p.is_dir())
        .filter(|p| p.extension().map_or(false, |f| f == "mkv"))
    {
        for m in config.moves.iter() {
            let filename = i.file_name().unwrap().to_str().unwrap().to_owned();
            if filename.contains(&m.pattern) {
                println!("Matching {} with {}", &filename, &m.pattern);
                let to = PathBuf::from(&m.path).with_file_name(&filename);
                std::fs::rename(i, to)?;
            }
        }
    }

    Ok(())
}
