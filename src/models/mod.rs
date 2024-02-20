use std::io;

pub enum Command {
    Dir(Option<String>, Option<String>, Option<String>, Option<String>),
    Help,
    Vol,
    Path,
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
    CommandFailed(String)
} impl From<io::Error> for CommandError {
    fn from(err: io::Error) -> Self {
        CommandError::IOError(err)
    }
}