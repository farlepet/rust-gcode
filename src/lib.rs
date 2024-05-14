pub mod writer;
pub mod position;

pub use crate::position::{GCodePosition, GCodeOffset};

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
            _ => GCodeError::IOError
        }
    }
}
