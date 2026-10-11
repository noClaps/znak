use std::fmt::Display;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Frontmatter { line: String, cause: String },
    Content { line: String, cause: String },
    Other(String),
}

impl Error {
    pub fn frontmatter(line: &str, cause: &str) -> Self {
        Self::Frontmatter {
            line: line.to_string(),
            cause: cause.to_string(),
        }
    }
    pub fn content(line: &str, cause: &str) -> Self {
        Self::Content {
            line: line.to_string(),
            cause: cause.to_string(),
        }
    }
    pub fn other(cause: &str) -> Self {
        Self::Other(cause.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Frontmatter { line, cause } => write!(
                f,
                "\
Error encountered while parsing frontmatter
Line: {line}
Error: {cause}"
            ),
            Self::Content { line, cause } => write!(
                f,
                "\
Error encountered while parsing content
Line: {line}
Error: {cause}"
            ),
            Self::Other(cause) => write!(f, "Error: {cause}"),
        }
    }
}

impl std::error::Error for Error {}
