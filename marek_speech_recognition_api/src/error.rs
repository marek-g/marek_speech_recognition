use std::error::Error;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;

#[derive(Debug)]
#[non_exhaustive]
pub enum SpeechError {
    LoadLibraryError(String),
    LanguageFolderError(PathBuf),
    Unknown,
}

impl Display for SpeechError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for SpeechError {}

pub type SpeechResult<T = (), E = SpeechError> = Result<T, E>;
