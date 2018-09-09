use std::io;
use std::result;

#[derive(Debug, Fail)]
pub enum Error {

    #[fail(display = "[filedb] DB File not exists: {}", path)]
    DBFile {
        path: String
    },

    #[fail(display = "[filedb] I/O Error: {:?}", error)]
    IOError {
        error: io::Error
    },
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::IOError { error }
    }
}

pub type Result<T> = result::Result<T, Error>;
