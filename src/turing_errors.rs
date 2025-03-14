use std::fmt::Debug;

pub enum TuringError {
    /// Error returned when a transition tried to move a pointer out of the ribbons
    OutofRangeRibbonError {
        accessed_index: usize,
        ribbon_size: usize,
    },
    /// Error when a word given to a turing machine did not end on an accepting state
    WordNotAcceptedError,
    /// Error when a transition was not given enougth args
    NotEnougthArgsTransitionError,
}

impl Debug for TuringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OutofRangeRibbonError {
                accessed_index: tried_to_access,
                ribbon_size,
            } => f
                .debug_struct("OutofRangeRibbonError")
                .field("tried_to_access", tried_to_access)
                .field("ribbon_size", ribbon_size)
                .finish(),
            Self::WordNotAcceptedError => write!(f, "WordNotAcceptedError"),
            Self::NotEnougthArgsTransitionError => write!(f, "NotEnougthArgsError"),
        }
    }
}
