use std::env;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{BufReader, BufRead};

mod error;

use error::{Error, Result};

const MINECRAFT_DIR: &'static str = ".minecraft";
const OPTIONS_FILE_NAME: &'static str = "options.txt";
const OPTIONS_SEPARATOR: &'static str = ":";
const MAX_FPS_OPTION: &'static str = "maxFps";

fn main() {
    if let Err(e) = try_main() {
        eprintln!("ERROR: {}", e);
    }
}

fn try_main() -> anyhow::Result<()> {
    let options_file_path = get_options_file_path()?;
    println!("INFO: Options file found at {}", options_file_path.display());

    let options = read_options(&options_file_path)?;
    
    Ok(())
}

fn get_options_file_path() -> Result<PathBuf> {
    let Ok(app_data_dir) = env::var("APPDATA") else { return Err(Error::AppDataDir) };

    let mut options_file_path = PathBuf::new();
    options_file_path.push(app_data_dir);
    options_file_path.push(MINECRAFT_DIR);
    options_file_path.push(OPTIONS_FILE_NAME);

    if !options_file_path.exists() {
        return Err(Error::FindOptionsFile);
    }

    Ok(options_file_path)
}

struct Option {
    key: String,
    value: String,
}

fn read_options(path: &Path) -> Result<Vec<Option>> {
    let file = File::open(path).map_err(|e| Error::ReadOptionsFile(e))?;
    let reader = BufReader::new(file);
    let mut options = Vec::new();
    for (line_num, line) in reader.lines().enumerate() {
        let line = line.map_err(|e| Error::ReadOptionsFile(e))?;
        let parts: Vec<&str> = line.split(OPTIONS_SEPARATOR).collect();
        if parts.len() != 2 {
            return Err(Error::OptionsFileFormat(line_num));
        }
        options.push(Option { key: parts[0].to_string(), value: parts[1].to_string() });
    }
    Ok(options)
}
