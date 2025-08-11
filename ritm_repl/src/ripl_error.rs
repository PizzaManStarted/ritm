use std::fmt::{format, Display};

use colored::{ColoredString, Colorize};
use ritm_core::turing_errors::{TuringError, TuringParserError};

#[derive(Debug)]
pub enum RiplError {
    OutOfRangeIndexError{
        index: usize
    },
    UnknownCommandError{
        command: String,
    },
    CouldNotParseStringIntError{
        value: String
    },
    CouldNotParseStringError{
        value: String
    },
    ArgsNumberError {
        received : usize,
        expected : usize
    },
    EncounteredTuringError {
        error : TuringError
    },
    EncounteredParsingError {
        error : TuringParserError
    }
}


pub fn print_error_help(error: RiplError)
{
    println!("{}",
        match error {
            RiplError::OutOfRangeIndexError{index}=>format!("The given index \"{}\" is not a valid index",as_arg_error(index)),
            RiplError::UnknownCommandError{command}=>format!("The given command \"{}\" is not a known command",as_arg_error(command)),
            RiplError::CouldNotParseStringIntError{value}=>format!("Could not parse the value \"{}\" into a positive integer",as_arg_error(value)),
            RiplError::CouldNotParseStringError{value}=>format!("Could not parse the value \"{}\"", as_arg_error(value)),
            RiplError::ArgsNumberError{received,expected}=>format!("Read wrong number of arguments. Expected {} but read {} args",expected.to_string().blue(),as_arg_error(received)),
            RiplError::EncounteredTuringError { error } => format!("Ran into the following turing error : {:?}", error),
            RiplError::EncounteredParsingError { error } => format!("Ran into the following error during the parsing : {:?}", error)
                    }.red()
    )
}


fn as_arg_error<E>(val: E) -> ColoredString where E : Display
{
    val.to_string().green()
}