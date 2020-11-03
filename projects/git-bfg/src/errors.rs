#[derive(Debug)]
pub enum CleanerError {
    IOError(std::io::Error),
    GitError(git2::Error),
    UnknownError,
}

pub type Result<T> = std::result::Result<T, CleanerError>;

impl From<()> for CleanerError {
    fn from(_: ()) -> Self {
        Self::UnknownError
    }
}

impl From<std::io::Error> for CleanerError {
    fn from(e: std::io::Error) -> Self {
        Self::IOError(e)
    }
}

impl From<git2::Error> for CleanerError {
    fn from(e: git2::Error) -> Self {
        Self::GitError(e)
    }
}
