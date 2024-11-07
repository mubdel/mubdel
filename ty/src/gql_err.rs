use std::fmt::{Display, Formatter, Result};

pub enum GqlErrCategory {
    Unknown,
    BadUserInput,
    InternalServerError,
    Unauthorized,
}

impl Display for GqlErrCategory {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::Unknown => write!(f, "Unknown"),
            Self::BadUserInput => write!(f, "Bad User Input"),
            Self::InternalServerError => write!(f, "Internal Server Error"),
            Self::Unauthorized => write!(f, "Unauthorized"),
        }
    }
}
