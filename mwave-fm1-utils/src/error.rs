use std::error::Error;
use std::fmt::Display;

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct FM1Error(pub(crate) String);

impl Display for FM1Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for FM1Error {}


pub(crate) fn convert_error(e: impl Error) -> FM1Error {
    FM1Error(e.to_string())
}
