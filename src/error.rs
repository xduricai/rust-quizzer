use std::fmt::Display;

/// An enum that represents all errors that can
/// originate from this library.
#[derive(Debug)]
pub enum Error {
    EmptyInput,
    InvalidAnswerNumber,
    NoQuestionsError,
    Io(std::io::Error),
    Json(serde_json::Error),
    //F(<i32 as FromStr>::Err)
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Pretty-format the error
        match self {
            Error::EmptyInput => write!(f, "The text input you just gave was empty"),
            Error::InvalidAnswerNumber => write!(f, "The answer number you gave was invalid, there are only answers 1-4"),
            Error::NoQuestionsError => write!(f, "There are no questions available"),
            Error::Io(e) => write!(f, "IO error: {e}"),
            Error::Json(e) => write!(f, "JSON error: {e}"),
            //Error::F(e) => write!(f, "i32 parse error: {e}")
        }
    }
}

/// Implementing std::error::Error makes the error
/// play nice with `anyhow`. Note that you must implement
/// [std::fmt::Debug] and [std::fmt::Display] in order to
/// implement [std::error::Error].
impl std::error::Error for Error {}

/// Convert [serde_json::Error] to [Error],
/// allowing us to use the try (`?`) operator
/// to easily convert [Result<_, serde_json::Error>]
/// to [Result<_, Error>].
impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self::Json(e)
    }
}

/// Convert [std::io::Error] to [Error]
/// allowing us to use the try (`?`) operator
/// to easily convert [Result<_, serde_json::Error>]
/// to [Result<_, Error>].
impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}