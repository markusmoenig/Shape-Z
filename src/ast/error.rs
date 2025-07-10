use std::{fmt, path::PathBuf};

/// Represents a parser error.
pub struct ParseError {
    pub message: String,
    pub line: usize,
    pub path: PathBuf,
}

impl ParseError {
    pub fn new<M>(message: M, line: usize, path: &PathBuf) -> Self
    where
        M: Into<String>,
    {
        Self {
            message: message.into(),
            line,
            path: path.clone(),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.line > 0 {
            if let Some(file) = self.path.to_str() {
                write!(f, "{} in {} at line {}.", self.message, file, self.line)
            } else {
                write!(f, "{} in <unknown file>.", self.message)
            }
        } else {
            if let Some(file) = self.path.to_str() {
                write!(f, "{}: \"{}\".", self.message, file)
            } else {
                write!(f, "{} in <unknown file>.", self.message)
            }
        }
    }
}

#[derive(Debug)]
pub struct RuntimeError {}
