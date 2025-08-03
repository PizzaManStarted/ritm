use std::fmt::Display;

use strum_macros::EnumIter;

use colored::Colorize;

use crate::modes::{choice_modes::ModeEvent, starting_modes::StartingMode};




#[derive(EnumIter)]
pub enum ModifyTuringMode {
    PrintSummary,
    AddState,
    AddTransitions,
    RemoveTransitions,
    RemoveState,
    SaveTM,
    FeedWord
}


impl Display for ModifyTuringMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            ModifyTuringMode::PrintSummary => "Print a summary of the TM",
            ModifyTuringMode::AddState => "Add a state",
            ModifyTuringMode::AddTransitions => "Add one or multiple transition",
            ModifyTuringMode::RemoveTransitions => "Remove one or multiple transition",
            ModifyTuringMode::RemoveState => "Remove a state",
            ModifyTuringMode::SaveTM => "Save this TM as a file",
            ModifyTuringMode::FeedWord => "Feed a word and start executing this TM",
        })
    }
}

impl ModeEvent for ModifyTuringMode {
    fn print_help(&self) {
        let tm_it_bold = "Turing Machine".italic().bold();
        print!("-> ");
        match self {
            ModifyTuringMode::PrintSummary => println!("Prints a detailed overview of the current {tm_it_bold}"),
            ModifyTuringMode::AddState => println!("Adds a {} to the current {tm_it_bold}", "state".purple()),
            ModifyTuringMode::AddTransitions => println!("Adds one or multiple {} to the current {tm_it_bold}", "transitions".purple()),
            ModifyTuringMode::RemoveTransitions => println!("Removes one or multiple {} from the current {tm_it_bold}", "transitions".purple()),
            ModifyTuringMode::RemoveState => println!("Removes a {} from the current {tm_it_bold}", "state".purple()),
            ModifyTuringMode::SaveTM => println!("Saves the current {tm_it_bold} as a file"),
            ModifyTuringMode::FeedWord => println!("Feeds a word to the {tm_it_bold} and starts executing it"),
        }
        println!("")
    }
    
    fn choose_option(&self) -> super::choice_modes::Modes {
        super::choice_modes::Modes::Start
    }
    
    
}

