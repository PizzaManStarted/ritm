use std::fmt::{Debug, Display};

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

impl Display for TuringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            TuringError::IllegalActionError { cause } => {
                format!("The following action could cause an error or is simply not authorised : \n{}", cause)
            },
            TuringError::OutofRangeRibbonError { accessed_index, ribbon_size } => {
                format!("A transition caused the ribbon pointer to point ouside the bounds of the ribbon. Tried to access {} but the length of the ribbon was {}", accessed_index, ribbon_size)
            },
            TuringError::OutOfRangeTransitionError { accessed_index, states_len } => {
                format!("Tried to access a transition from the vector of transition in a state, but it was out of range. Tried to access the index {} but there are only {} transitions", accessed_index, states_len)
            },
            TuringError::IncompatibleTransitionError { expected, received } => {
                format!("Tried to append a transition affecting \"{received}\" ribbons while all others only affect \"{expected}\" ribbons")
            },
            TuringError::TransitionArgsError { reason } => {
                format!("There was a problem during the creation of a transition : \n{}", reason)
            },
            TuringError::OutOfRangeStateError { accessed_index, states_len } => {
                format!("Tried to access a state from the vector of states in the graph, but it was out of range. Tried to access the index {} but there are only {} states", accessed_index, states_len)
            },
            TuringError::UnknownStateError { state_name } => {
                format!("Tried to access a state called \"{}\", but it doesn't exists", state_name)
            },
        })
    }
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


impl Display for TuringParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", {
            match self {
                TuringParserError::FileError { given_path, error_reason } 
                    => format!("Ran into an error trying to open the file at \"{}\". The reason being : {}", given_path, error_reason),
                TuringParserError::ParsingError { line_col_pos, value, missing_value } 
                    => format!("Impossible to parse the given input.\n{}{}", get_arrow_under(value, line_col_pos), {
                        if let Some(token) = missing_value {
                            format!("\nThis token might be missing : \"{}\"", token.to_string())
                        }
                        else {
                            String::from("")
                        }
                    }),
                TuringParserError::EncounteredTuringError { line_col_pos, turing_error, value } => 
                    format!("Encountered an error at the following line: \n{}\nReason: {}", get_arrow_under(value, line_col_pos), turing_error),
            }
        })
    }
}

fn get_arrow_under(value: &String, line_col_pos : &Option<(usize, usize)>)  -> String
{
    if let Some((line, col)) = line_col_pos {
        let line_str = (line).to_string();
        format!("{line_str}: {value}\n{}{}^", String::from(" ").repeat(line_str.len() + 2), String::from("-").repeat(col-1))
    }
    else {
        value.to_string()
    }
}