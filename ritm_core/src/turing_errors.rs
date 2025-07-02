use std::fmt::Debug;

pub enum TuringError {
    /// Error thrown when an action not supported by the turing machines is performed (ex: creating a turing machine with 0 ribbon or trying to remove the initial state)
    IllegalActionError {
        cause : String
    },
    /// Error returned when a transition tried to move a pointer out of the ribbons
    OutofRangeRibbonError {
        accessed_index: usize,
        ribbon_size: usize,
    },
    /// Error returned when trying to access an out of range transition
    OutOfRangeTransitionError {
        accessed_index : usize,
        states_len : usize,
    },
    /// Error when a word given to a turing machine did not end on an accepting state
    WordNotAcceptedError,
    /// Error when a transition was given a wrong number of args
    ArgsSizeTransitionError,
    /// Error when trying to access a state using an index that goes outside of the bound 
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
            Self::IllegalActionError { 
                            cause 
                    } => f
                        .debug_struct("IllegalActionError")
                        .field("cause", cause)
                        .finish(),
            Self::OutofRangeRibbonError {
                        accessed_index: tried_to_access,
                        ribbon_size,
                    } => f
                        .debug_struct("OutofRangeRibbonError")
                        .field("tried_to_access", tried_to_access)
                        .field("ribbon_size", ribbon_size)
                        .finish(),
            Self::OutOfRangeTransitionError { accessed_index, states_len } => f
                        .debug_struct("OutOfRangeTransitionError")
                        .field("accessed_index", accessed_index)
                        .field("states_len", states_len)
                        .finish(),
            Self::WordNotAcceptedError => write!(f, "WordNotAcceptedError"),
            Self::ArgsSizeTransitionError => write!(f, "NotEnougthArgsError"),
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
