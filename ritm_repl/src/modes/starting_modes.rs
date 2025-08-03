use std::fmt::Display;

use crate::modes::choice_modes::{ModeEvent, Modes};
use colored::Colorize;
use strum_macros::EnumIter;


#[derive(EnumIter)]
pub enum StartingMode {
    CreateTM,
    LoadTM
}

impl Display for StartingMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            StartingMode::CreateTM => "Create new Turing Machine",
            StartingMode::LoadTM => "Load an existing Turing Machine",
        })
    }
}




impl ModeEvent for StartingMode {
    fn print_help(&self) {
        let tm_it = "Turing Machine".italic().bold();
        print!("-> ");
        match self {
            StartingMode::CreateTM => println!("Creates a new {tm_it} by specifying the {} and a {} (if any)", 
                                                "number of writting ribbons".purple(),
                                                "name".yellow() ),
            StartingMode::LoadTM => println!("Loads a new {tm_it} by specifying a {} to it from",
                                                "file path".purple()),
        }
        println!("")
    }
    
    fn choose_option(&self) -> Modes {
        Modes::Modify
    }

}