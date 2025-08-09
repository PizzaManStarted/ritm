use std::fmt::Debug;

#[derive(Debug)]
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
    /// Error when a transition cannot be added due to the number of ribbons it affects 
    IncompatibleTransitionError{
        /// Number of writting ribbons expected
        expected: usize,
        /// Numbers of writting ribbons got
        received: usize,
    },
    /// Error when trying to construct a transition with an incorrect number of arguments
    TransitionArgsError {
        reason: String
    },
    /// Error when trying to access a state using an index that goes outside of the bound 
    OutOfRangeStateError {
        accessed_index : usize,
        states_len : usize,
    },
    /// Error when trying to access a state using a string but the state does not exists
    UnknownStateError {
        state_name : String
    },
}

#[derive(Debug)]
pub enum TuringParserError {
    FileError {
        given_path: String,
        error_reason: String
    },
    /// Error when failing to parse a given string value
    ParsingError {
        line_col_pos: Option<(usize, usize)>,
        value: String,
        missing_value : Option<String>
    },
    
    /// Error when a [TuringError] was encountered **while** parsing a string value
    EncounteredTuringError {
        line_col_pos: Option<(usize, usize)>,
        turing_error: TuringError,
        value: String
    }
}