use std::fmt;

#[derive(Debug, Clone)]
pub struct NoSongError;

impl fmt::Display for NoSongError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "no song matching those lyrics")
    }
}