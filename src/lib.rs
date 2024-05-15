pub mod position;
pub mod writer;

pub use crate::position::{GCodeOffset, GCodePosition};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GCodeError {
    /// Generic I/O error
    IOError,
    /// Value out of range
    OutOfRangeError,
}
impl From<std::io::Error> for GCodeError {
    fn from(value: std::io::Error) -> Self {
        match value.kind() {
            /* TODO */
            _ => GCodeError::IOError,
        }
    }
}
impl std::fmt::Display for GCodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Self::IOError => "IOError",
            Self::OutOfRangeError => "OutOfRangeError",
        };

        write!(f, "GCodeError::{}", name)
    }
}
impl std::error::Error for GCodeError {}
