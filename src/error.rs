use std::convert::From;
use std::io::Error as IoError;

#[derive(Debug)]
pub enum Error {
    NoBinary,
    Io(IoError),
}

impl From<IoError> for Error {
    fn from(e: IoError) -> Self {
        Error::Io(e)
    }
}
