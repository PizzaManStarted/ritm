use std::fmt::Debug;

pub enum TuringError {
    OutOfBoundsPointerError,
    ReadOnlyRibbonError,
    OutofRangeRibbonError {
        tried_to_access: usize,
        ribbon_size: usize,
    },
    WordNotAcceptedError,
    NotEnougthArgsError,
}

impl Debug for TuringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OutOfBoundsPointerError => write!(f, "OutOfBoundsPointerError"),
            Self::ReadOnlyRibbonError => write!(f, "ReadOnlyRibbonError"),
            Self::OutofRangeRibbonError {
                tried_to_access,
                ribbon_size,
            } => f
                .debug_struct("OutofRangeRibbonError")
                .field("tried_to_access", tried_to_access)
                .field("ribbon_size", ribbon_size)
                .finish(),
            Self::WordNotAcceptedError => write!(f, "WordNotAcceptedError"),
            Self::NotEnougthArgsError => write!(f, "NotEnougthArgsError"),
        }
    }
}
