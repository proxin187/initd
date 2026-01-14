

#[derive(Debug)]
pub enum Error {
    NoServicePath,
    InvalidServiceDirectory,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Error::NoServicePath => f.write_str("couldnt find 'initd.services' in kernel command line parameters"),
            Error::InvalidServiceDirectory => f.write_str("invalid 'initd.services' directory, must have subdirectories boot/, supervise/ and shutdown/"),
        }
    }
}

impl std::error::Error for Error {}


