use std::fmt::Display;

use crate::{modes::choice_modes::{ModeEvent, Modes}, ripl_error::{print_error_help, RiplError}};
use colored::Colorize;
use ritm_core::turing_graph::TuringMachineGraph;
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
    
    fn choose_option(&self, rl: &mut Editor<(), FileHistory>) -> Modes {
        let res = match self {
            StartingMode::CreateTM => {
                create_tm(rl)
            },
            StartingMode::LoadTM => {
                todo!()
            },
        };
        if let Err(e) = res {
            print_error_help(e);
            return Modes::Start;
        }
        Modes::Modify
    }

}



fn create_tm(rl: &mut Editor<(), FileHistory>) -> Result<TuringMachineGraph, RiplError>
{
    println!("Enter the numbers of {} of the Turing machine ({}) :", "writting ribbons".blue(), "k".blue().italic());
    loop {
        let readline = rl.readline("==> ");
        match readline {
            Ok(l) => {
                let l = l.trim().to_string();
                if l.is_empty() {
                    continue;
                }
                // Read requested nb
                let index_res = l.parse();
                if let Err(_) = &index_res {
                    return Err(RiplError::CouldNotParseStringIntError { value: l });
                }
            
                let tm = TuringMachineGraph::new(index_res.unwrap());
    
                if let Err(e) = tm {
                    return Err(RiplError::EncounteredTuringError { error: e });
                }
                
                return Ok(tm.unwrap());
            },
            Err(e) => return Err(RiplError::CouldNotParseStringError { value: e.to_string() }),
        }
    }
}