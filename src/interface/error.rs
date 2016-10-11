use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NotConnected;

impl Display for NotConnected {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "NotConnected error: {}", self)
    }
}

impl Error for NotConnected {
    fn description(&self) -> &str {
        "Microbrute not connected."
    }
}
