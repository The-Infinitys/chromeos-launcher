use std::fmt;

use colored::Colorize;

pub enum Error {
    Io(std::io::Error),
    Var(std::env::VarError),
}
impl Error {
    fn display_for(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Io(err) => write!(f, "  {}: {}", "IO error".cyan().bold(), err),
            Error::Var(err) => write!(
                f,
                "  {}: {}",
                "Environment variable error".green().bold(),
                err
            ),
        }
    }
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f,"{}:","Error".red().bold())?;
        self.display_for(f)
    }
}
impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	writeln!(f)?;
        self.display_for(f)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<std::env::VarError> for Error {
    fn from(err: std::env::VarError) -> Self {
        Error::Var(err)
    }
}

impl From<String> for Error {
    fn from(err: String) -> Self {
        Error::Io(std::io::Error::new(std::io::ErrorKind::InvalidInput, err))
    }
}
