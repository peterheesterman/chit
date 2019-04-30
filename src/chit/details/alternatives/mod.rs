use std::io::Read;
use std::error::Error;
use std::fmt;
use reqwest::StatusCode;

#[derive(Serialize, Deserialize)]
pub struct AlternativesSet {
    pub name: String,
    pub alternatives: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Alternatives {
    pub description: String,
    pub sets: Vec<AlternativesSet>,
}

#[derive(Debug)]
pub enum RetrieveAlternativesError {
    ParseError(serde_json::Error),
    ConnectionError(reqwest::Error),
    ReadError(std::io::Error),
}

impl fmt::Display for RetrieveAlternativesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RetrieveAlternativesError::ParseError(err) => write!(f, "Parse error: {}", err),
            RetrieveAlternativesError::ReadError(err) => write!(f, "Read error: {}", err),
            RetrieveAlternativesError::ConnectionError(err) => write!(f, "Connection error: {}", err),
        }
    }
}


// This is important for other errors to wrap this one.
impl Error for RetrieveAlternativesError {
    fn description(&self) -> &str {
        match self {
            RetrieveAlternativesError::ParseError(err) => err.description(),
            RetrieveAlternativesError::ReadError(err) => err.description(),
            RetrieveAlternativesError::ConnectionError(err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        // Generic error, underlying cause isn't tracked.
        Some(match self {
            RetrieveAlternativesError::ParseError(err) => err,
            RetrieveAlternativesError::ReadError(err) => err,
            RetrieveAlternativesError::ConnectionError(err) => err,
        })
    }
}

impl From<reqwest::Error> for RetrieveAlternativesError {
    fn from(item: reqwest::Error) -> Self {
        RetrieveAlternativesError::ConnectionError(item)
    }
}

impl From<serde_json::Error> for RetrieveAlternativesError {
    fn from(item: serde_json::Error) -> Self {
        RetrieveAlternativesError::ParseError(item)
    }
}

impl From<std::io::Error> for RetrieveAlternativesError {
    fn from(item: std::io::Error) -> Self {
        RetrieveAlternativesError::ReadError(item)
    }
}

const ALTERNATIVES_URL: &str =
    "https://raw.githubusercontent.com/peterheesterman/chit/master/alternatives.json";

pub fn get_alternatives() -> Result<Alternatives, RetrieveAlternativesError> {
    let mut res = reqwest::get(ALTERNATIVES_URL)?;

    let mut res_body = String::new();

    res.read_to_string(&mut res_body)?;

    let alternatives: Alternatives =
        serde_json::from_str(&res_body)?;

    Ok(alternatives)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_alternatives() {
        let alternatives = get_alternatives().expect("Alternative retrieval failed");
        assert_ne!(alternatives.sets.len(), 0)
    }
}
