use std::{fmt::Display, fs::File};

use crate::{modes::choice_modes::{ModeEvent, Modes}, query_string, query_usize, ripl_error::{print_error_help, RiplError}, DataStorage};
use colored::Colorize;
use ritm_core::{turing_graph::TuringMachineGraph, turing_parser::parse_turing_graph_file_path};
use rustyline::{history::FileHistory, Editor};
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
            StartingMode::CreateTM => println!("Creates a new {tm_it} by specifying the {}", 
                                                "number of writting ribbons".purple()),
            StartingMode::LoadTM => println!("Loads a new {tm_it} by specifying a {} to it from",
                                                "file path".purple()),
        }
        println!("")
    }
    
    fn choose_option(&self, rl: &mut Editor<(), FileHistory>, storage: &mut DataStorage) -> Modes {
        let res = match self {
            StartingMode::CreateTM => {
                create_tm(rl)
            },
            StartingMode::LoadTM => {
                load_tm(rl)
            },
        };
        if let Err(e) = res {
            print_error_help(e);
            return Modes::Start;
        }
        // Store the created/loaded turing machine graph
        storage.graph = Some(res.unwrap());
        // Change the mode to allow modifying this tm graph
        Modes::Modify
    }
    
    fn get_help_color(str : colored::ColoredString) -> colored::ColoredString {
        str.blue()
    }

}



fn create_tm(rl: &mut Editor<(), FileHistory>) -> Result<TuringMachineGraph, RiplError>
{
    let res = query_usize(rl, format!("Enter the numbers of {} of the Turing machine ({}) :", "writting ribbons".blue(), "k".blue().italic()));

    if let Err(e) = res {
        return Err(e);
    }
    
    let tm = TuringMachineGraph::new(res.unwrap());
    if let Err(e) = tm {
        return Err(RiplError::EncounteredTuringError { error: e });
    }
    
    Ok(tm.unwrap())
}



fn load_tm(rl: &mut Editor<(), FileHistory>) -> Result<TuringMachineGraph, RiplError>
{
    let res = query_string(rl, format!("Enter the {} the Turing machine to read:", "path".blue()));

    if let Err(e) = res {
        return Err(e);
    }

    let tm = parse_turing_graph_file_path(res.unwrap());
    if let Err(e) = tm {
        return Err(RiplError::EncounteredTuringError { error: e });
    }
    
    Ok(tm.unwrap())

}