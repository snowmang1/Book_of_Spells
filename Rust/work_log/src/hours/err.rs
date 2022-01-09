
use std::fmt;

#[derive(Debug)]
pub enum Errors {
    ParseErrHour(String),
    ParseErrMin(String),
    ParseErrSec(String),
}

impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            Errors::ParseErrHour(s)    => write!(f, "ERROR PARSING STRING INTO I8, HOUR: {}"
                                                , s),
            Errors::ParseErrMin(s)     => write!(f, "ERROR PARSING STRING INTO I8, MIN: {}"
                                                , s),
            Errors::ParseErrSec(s)     => write!(f, "ERROR PARSING STRING INTO I8, SEC: {}"
                                                , s),
        }
    }
}
