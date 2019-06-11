#[derive(Debug)]
pub enum Error {
    /// Error when parsing network address
    AddrError(std::net::AddrParseError),

    /// I/O error.
    IoError(std::io::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::AddrError(ref err) => write!(f, "Address error: {}", err),
            Error::IoError(ref err) => write!(f, "I/O error: {}", err),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Error::AddrError(ref err) => Some(err),
            Error::IoError(ref err) => Some(err),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IoError(error)
    }
}

impl From<std::net::AddrParseError> for Error {
    fn from(error: std::net::AddrParseError) -> Self {
        Error::AddrError(error)
    }
}
