use std::io;

#[derive(Debug)]
pub enum FountainError {
    ParseError(String),
    IOError(io::Error),
}

impl From<io::Error> for FountainError {
    fn from(err: io::Error) -> FountainError {
        FountainError::IOError(err)
    }
}
