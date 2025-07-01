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
    /// Error when trying to access a state using an index that goes outide of the bound 
    OutOfRangeStateError {
        accessed_index : usize,
        states_len : usize,
    },
    /// Error when trying to access a state using a string but the state does not exists
    UnknownStateError {
        state_name : String
    }
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
            Self::OutOfRangeStateError { accessed_index, states_len } => f
                        .debug_struct("OutOfRangeStateError")
                        .field("accessed_index", accessed_index)
                        .field("states_len", states_len)
                        .finish(),
            Self::UnknownStateError { state_name } => f
                        .debug_struct("UnknownStateError")
                        .field("state_name", state_name)
                        .finish(),
        }
    }
}
