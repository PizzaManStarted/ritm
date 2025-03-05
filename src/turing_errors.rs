use std::fmt::Debug;

pub enum TuringError
{
    OutOfBoundsPointerError,
    ReadOnlyRubonError,
    WordNotAcceptedError,
    NotEnougthArgsError,
}


impl Debug for TuringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OutOfBoundsPointerError => write!(f, "OutOfBoundsPointerError"),
            Self::ReadOnlyRubonError => write!(f, "ReadOnlyRubonError"),
            Self::WordNotAcceptedError => write!(f, "WordNotAcceptedError"),
            Self::NotEnougthArgsError => write!(f, "NotEnougthArgsError"),
        }
    }
}