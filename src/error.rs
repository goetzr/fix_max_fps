use std::error::Error as StdError;
use std::result::Result as StdResult;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum Error {
    AppDataDir,
    FindOptionsFile,
    ReadOptionsFile(io::Error),
    MaxFpsOptionMissing,
    MaxFpsOptionMalformed,
    WriteOptionsFile(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Error::AppDataDir => write!(f, "failed to retrieve the APPDATA environment variable"),
            Error::FindOptionsFile => write!(f, "options file does not exist"),
            Error::ReadOptionsFile(e) => write!(f, "failed to read the options file: {}", e),
            Error::MaxFpsOptionMissing => write!(f, "maxFps option not found"),
            Error::MaxFpsOptionMalformed => write!(f, "maxFps option malformed"),
            Error::WriteOptionsFile(e) => write!(f, "failed to write the options file: {}", e),
        }
    }
}

impl StdError for Error {}

pub type Result<T> = StdResult<T, Error>;