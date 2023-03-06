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
    unsafe { libc::getchar() };
}

fn try_main() -> anyhow::Result<()> {
    let options_file_path = get_options_file_path()?;
    println!("INFO: Options file found at {}", options_file_path.display());

    let mut options = read_options(&options_file_path)?;
    let max_fps_option = get_max_fps_option(&mut options)?;
    if get_max_fps_value(&max_fps_option)? == "0" {
        *max_fps_option = format!("{}{}{}", MAX_FPS_OPTION, OPTIONS_SEPARATOR, GOOD_VALUE);
        write_options(options.as_slice(), &options_file_path)?;
        println!("INFO: {} option fixed!", MAX_FPS_OPTION);
    } else {
        println!("INFO: {} option is OK!", MAX_FPS_OPTION);
    }
    
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

fn read_options(path: &Path) -> Result<Vec<String>> {
    let file = OpenOptions::new().read(true).open(path).map_err(|e| Error::ReadOptionsFile(e))?;
    let reader = BufReader::new(file);
    let mut options = Vec::new();
    for line in reader.lines() {
        let line = line.map_err(|e| Error::ReadOptionsFile(e))?;
        options.push(line);
    }
    Ok(options)
}

fn get_max_fps_option(options: &mut [String]) -> Result<&mut String> {
    Ok(options.iter_mut().find(|op| op.starts_with(MAX_FPS_OPTION)).ok_or(Error::MaxFpsOptionMissing)?)
}

fn get_max_fps_value(max_fps_option: &String) -> Result<&str> {
    let idx_sep = max_fps_option.find(OPTIONS_SEPARATOR).ok_or(Error::MaxFpsOptionMalformed)?;
    Ok(&max_fps_option[idx_sep + 1..])
}

fn write_options(options: &[String], path: &Path) -> Result<()> {
    let file = OpenOptions::new().write(true).open(path).map_err(|e| Error::WriteOptionsFile(e))?;
    let mut writer = BufWriter::new(file);
    for option in options.iter() {
        writer.write_all(format!("{}\n", option).as_bytes()).map_err(|e| Error::WriteOptionsFile(e))?;
    }
    writer.flush().map_err(|e| Error::WriteOptionsFile(e))?;
    Ok(())
}