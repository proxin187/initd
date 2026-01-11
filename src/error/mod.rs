

#[derive(Debug)]
pub enum Error {
    NoServicePath,
    NotPid1,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Error::NoServicePath => f.write_str("couldnt find 'initd.services' in kernel command line parameters"),
            Error::NotPid1 => f.write_str("initd must be PID 1"),
        }
    }
}

impl std::error::Error for Error {}


