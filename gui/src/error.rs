use std::{fmt::Display, format, writeln};

use crate::turing::TransitionId;

#[derive(Debug, PartialEq)]
pub enum RitmError {
    GuiError(GuiError),
    CoreError(String),
}

#[derive(Debug, PartialEq)]
pub enum GuiError {
    // General
    InvalidApplicationState,

    // Graph
    GraphError { error: String },
    NoStateSelected,
    NoTransitionSelected,
    StateAlreadyExist { state_id: usize, name: String },
    TransitionAlreadyExist { transition_id: TransitionId },
    InvalidTransition { reason: String },

    // Code
    CodeError { error: String },
    SyntaxError { error: String },

    // Turing
    NoStateEditing,
    NoTransitionEditing,

    // IO
    InvalidInput { input: String },
    FileError { error: String },

    // Control
    NoStep,
}

impl Display for RitmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error = match self {
            Self::GuiError(s) => format!("{s}"),
            Self::CoreError(s) => format!("CORE ERROR : {s}"),
        };
        writeln!(f, "{}", error)
    }
}

impl Display for GuiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error = match self {
            // General
            Self::InvalidApplicationState => "Invalid application state !".to_string(),

            // Graph
            Self::GraphError { error } => format!("Error in the graph : {}", error),
            Self::NoStateSelected => "No state selected".to_string(),
            Self::NoTransitionSelected => "No transition selected".to_string(),
            Self::StateAlreadyExist { state_id, name } => {
                format!("State {} with id {} already exist", name, state_id)
            }
            Self::TransitionAlreadyExist { transition_id } => format!(
                "Transition {} between state {} and {} already exist",
                transition_id.id, transition_id.source_id, transition_id.target_id
            ),
            Self::InvalidTransition { reason } => format!("Invalid transition : {}", reason),

            // Code
            Self::CodeError { error } => format!("Error in code : {}", error),
            Self::SyntaxError { error } => format!("Syntax error : {}", error),

            // Turing
            Self::NoTransitionEditing => "No transition are being edited".to_string(),
            Self::NoStateEditing => "No state are being edited".to_string(),

            // IO
            Self::FileError { error } => format!("Could not load file: {}", error),
            Self::InvalidInput { input } => format!("{} is an invalid input", input),

            // Control
            Self::NoStep => "There is no next step !".to_string(),
        };
        writeln!(f, "{}", error)
    }
}
