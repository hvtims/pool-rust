use std::{error::Error, fmt};

#[derive(Debug)]
pub enum ParseErr {
    Empty,
    Malformed(Box<dyn Error>),
}

impl fmt::Display for ParseErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseErr::Empty => write!(f, "Todo file is empty"),
            ParseErr::Malformed(_) => write!(f, "Malformed todo file"),
        }
    }
}

impl Error for ParseErr {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ParseErr::Malformed(e) => Some(e.as_ref()),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct ReadErr(pub Box<dyn Error>);

impl fmt::Display for ReadErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to read todo file")
    }
}

impl Error for ReadErr {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.0.as_ref())
    }
}
