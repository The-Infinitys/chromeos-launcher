use std::fmt;

use colored::Colorize;
/*
fn display_for(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // エラーの種類に基づいて詳細を整形
        writeln!(f)?;
        let kind_str = match &self.kind {
            ErrorKind::Io(err) => {
                format!("IO Error - {}", err)
            }
            ErrorKind::Parse(err) => {
                format!("Parse Error - {}", err)
            }
            ErrorKind::CommandNotFound(cmd) => {
                format!("Command \"{}\" not found", cmd)
            }
            ErrorKind::InvalidArguments(arg_info) => {
                format!("Invalid Arguments - {}", arg_info)
            }
            ErrorKind::Other(desc) => desc.to_string(),
        };
        if !kind_str.is_empty() {
            writeln!(f, "  {}: {}", "Kind".cyan().bold(), kind_str)?;
        }
        if let Some(msg) = &self.message {
            let formatted_str = {
                let lines: Vec<String> = msg
                    .split("\n")
                    .map(|line| format!("    {}", line.trim()))
                    .collect();
                lines.join("\n")
            };
            writeln!(f, "  {}:|\n{}", "Message".green().bold(), formatted_str)?;
        }
        Ok(())
    }
*/

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
