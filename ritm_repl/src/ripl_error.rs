use std::fmt::{format, Display};

use colored::{ColoredString, Colorize};

#[derive(Debug)]
pub enum RiplError {
    OutOfRangeIndexError{
        index: usize
    },
    UnknownCommandError{
        command: String,
    },
    CouldNotParseStringError{
        value: String
    },
    ArgsNumberError {
        received : usize,
        expected : usize
    }
}


pub fn print_error_help(error: RiplError)
{
    println!("{}",
        match error {
            RiplError::OutOfRangeIndexError{index}=>format!("The given index \"{}\" is not a valid index", as_arg_error(index)),
            RiplError::UnknownCommandError{command}=>format!("The given command \"{}\" is not a known command", as_arg_error(command)),
            RiplError::CouldNotParseStringError{value}=>format!("Could not parse the value \"{}\" into an integer", as_arg_error(value)),
            RiplError::ArgsNumberError { received, expected } => format!("Read wrong number of arguments. Expected {} but read {} args", expected.to_string().blue(), as_arg_error(received)),
                    }.red()
    )
}


fn as_arg_error<E>(val: E) -> ColoredString where E : Display
{
    val.to_string().green()
}