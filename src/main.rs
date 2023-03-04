use std::env;
use std::path::{Path, PathBuf};
use std::fs::{OpenOptions};
use std::io::{BufReader, BufRead, BufWriter, Write};

mod error;

use error::{Error, Result};

const MINECRAFT_DIR: &'static str = ".minecraft";
const OPTIONS_FILE_NAME: &'static str = "options.txt";
const OPTIONS_SEPARATOR: &'static str = ":";
const MAX_FPS_OPTION: &'static str = "maxFps";
const GOOD_VALUE: &'static str = "120";

fn main() {
    if let Err(e) = try_main() {
        eprintln!("ERROR: {}", e);
    }
}

fn try_main() -> anyhow::Result<()> {
    let options_file_path = get_options_file_path()?;
    println!("INFO: Options file found at {}", options_file_path.display());

    let mut options = read_options(&options_file_path)?;
    if is_max_fps_zero(options.as_slice())? {
        fix_max_fps(&mut options);
        write_options(options.as_slice(), &options_file_path)?;
        println!("INFO: {} option fixed!", MAX_FPS_OPTION);
    } else {
        println!("INFO: {} option is OK!", MAX_FPS_OPTION);
    }

    unsafe { libc::getchar() };
    
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

//TODO: Want to copy malformed lines as-is. Only line that must be formatted properly is the maxFps line!

fn read_options(path: &Path) -> Result<Vec<Option>> {
    let file = OpenOptions::new().read(true).open(path).map_err(|e| Error::ReadOptionsFile(e))?;
    let reader = BufReader::new(file);
    let mut options = Vec::new();
    for (line_idx, line) in reader.lines().enumerate() {
        let line = line.map_err(|e| Error::ReadOptionsFile(e))?;
        let sep_idx = line.find(OPTIONS_SEPARATOR).ok_or(Error::OptionsFileFormat(line_idx + 1))?;
        let key = &line[..sep_idx];
        let value = &line[sep_idx + 1..];
        options.push(Option { key: key.to_string(), value: value.to_string() });
    }
    Ok(options)
}

fn is_max_fps_zero(options: &[Option]) -> Result<bool> {
    Ok(options.iter().find(|op| op.key == MAX_FPS_OPTION).ok_or(Error::MaxFpsOptionMissing)?.value == "0")
}

fn fix_max_fps(options: &mut Vec<Option>) {
    options.iter_mut().find(|op| op.key == MAX_FPS_OPTION).expect("option missing").value = GOOD_VALUE.to_string();
}

fn write_options(options: &[Option], path: &Path) -> Result<()> {
    let file = OpenOptions::new().write(true).open(path).map_err(|e| Error::WriteOptionsFile(e))?;
    let mut writer = BufWriter::new(file);
    for option in options.iter() {
        writer.write_all(format!("{}{}{}\n", option.key, OPTIONS_SEPARATOR, option.value).as_bytes()).map_err(|e| Error::WriteOptionsFile(e))?;
    }
    writer.flush().map_err(|e| Error::WriteOptionsFile(e))?;
    Ok(())
}