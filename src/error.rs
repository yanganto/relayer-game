//! Error management module
use failure_derive::*;
use toml;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "The config should be TOML: {}", 0)]
    TOMLParseError(toml::de::Error),
    #[fail(display = "Unexpected parameter: {}", 0)]
    ParameterError(&'static str),
    #[fail(display = "Command line args or options are not correct: {}", 0)]
    CliError(String),
    #[fail(display = "Patch Parameter `{}` is not valid", 0)]
    PatchParameterError(String),
    #[fail(display = "Unexpected: {}", 0)]
    UnknownError(&'static str),
}

impl From<std::num::ParseFloatError> for Error {
    fn from(_: std::num::ParseFloatError) -> Self {
        Error::ParameterError("Parameter cannot parse as number")
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::CliError(format!("{}", err))
    }
}

impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Self {
        Error::TOMLParseError(err)
    }
}

impl From<&'static str> for Error {
    fn from(s: &'static str) -> Self {
        Error::UnknownError(s)
    }
}
