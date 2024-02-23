use std::io;
use std::fmt;

pub enum Command {
    Dir,
    Help,
    Vol,
    Path(Option<String>, Option<String>),
    TaskList,
    Notepad,
    Echo(Option<String>, Option<String>, Option<String>, Option<String>),
    Color(Option<String>),
    Ping(Option<String>),
    Exit,
    Unknown,
}

#[derive(Debug)]
pub enum CommandError {
    IOError(io::Error),
    NotFound(String),
    InvalidArgument(String),
    MissingArguments(String),
    TooManyArguments(String),
    CommandFailed(String)
}
impl From<io::Error> for CommandError {
    fn from(err: io::Error) -> Self {
        CommandError::IOError(err)
    }
}
impl From<nix::errno::Errno> for CommandError {
    fn from(err: nix::errno::Errno) -> Self {
        CommandError::IOError(io::Error::new(io::ErrorKind::Other, err.to_string()))
    }
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandError::IOError(err) => write!(f, "I/O error {}", err),
            CommandError::NotFound(msg) => write!(f, "Command not found {}", msg),
            CommandError::InvalidArgument(msg) => write!(f, "Invalid argument(s) {}", msg),
            CommandError::MissingArguments(msg) => write!(f, "Missing argument(s) {}", msg),
            CommandError::TooManyArguments(msg) => write!(f, "Too many arguments {}", msg),
            CommandError::CommandFailed(msg) => write!(f, "Command failed {}", msg),
        }
    }
}

