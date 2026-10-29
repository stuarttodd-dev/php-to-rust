use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum ConfigLineError {
    MissingEquals,
    EmptyKey,
    InvalidInteger {
        value: String,
        source: ParseIntError,
    },
}

impl fmt::Display for ConfigLineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigLineError::MissingEquals => {
                write!(f, "expected 'key=value' with an '=' separator")
            }
            ConfigLineError::EmptyKey => write!(f, "key cannot be empty"),
            ConfigLineError::InvalidInteger { value, .. } => {
                write!(f, "value {:?} is not a valid i32", value)
            }
        }
    }
}

impl Error for ConfigLineError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ConfigLineError::InvalidInteger { source, .. } => Some(source),
            _ => None,
        }
    }
}
