use std::{f32::consts::E, fmt::Display, fs::File, io::Write, ops::DerefMut, path::{Path, PathBuf}};

use ritm_core::{turing_graph::TuringMachineGraph, turing_machine::TuringMachines, turing_parser::{self, parse_transition_string}, turing_state::TuringTransitionMultRibbons};
use rustyline::{history::FileHistory, Editor};
use strum_macros::EnumIter;

use colored::Colorize;

use crate::{modes::{choice_modes::{ModeEvent, Modes}, execute_mode, starting_modes::StartingMode}, query_prim, query_string, ripl_error::{print_error_help, RiplError}, DataStorage};




#[derive(EnumIter)]
pub enum ModifyTuringMode {
    PrintSummary,
    AddState,
    AddTransitions,
    RemoveTransitions,
    RemoveState,
    SaveTM,
    FeedWord,
    UnloadTM,
}


impl Display for ModifyTuringMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            ModifyTuringMode::PrintSummary => "Print a summary of the Turing Machine",
            ModifyTuringMode::AddState => "Add a state",
            ModifyTuringMode::AddTransitions => "Add one or multiple transition",
            ModifyTuringMode::RemoveTransitions => "Remove one or multiple transition",
            ModifyTuringMode::RemoveState => "Remove a state",
            ModifyTuringMode::SaveTM => "Save this TM as a file",
            ModifyTuringMode::FeedWord => "Feed a word and start executing this Turing Machine",
            ModifyTuringMode::UnloadTM => "Unload the current Turing Machine"
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
            ModifyTuringMode::UnloadTM => println!("Unloads the current Turing Machine and go back to the turing machine creation phase")
        }
        println!("")
    }
    
    fn choose_option(&self, rl: &mut Editor<(), FileHistory>, storage: &mut DataStorage) -> Modes {    
        let tm = storage.graph.as_mut().unwrap();
        match self {
            ModifyTuringMode::PrintSummary => {
                println!("{}", tm);
            },
            ModifyTuringMode::AddState => {
                let res = get_state_name(rl);
                if let Err(e) = res {
                    print_error_help(e);
                }
                else {
                    tm.add_state(&res.unwrap());
                }
            },
            ModifyTuringMode::AddTransitions => {
                if let Err(e) = add_transition(rl, tm) {
                    print_error_help(e);
                }
            },
            ModifyTuringMode::RemoveTransitions => todo!(),
            ModifyTuringMode::RemoveState => {
                let res = get_state_name(rl);
                if let Err(e) = res {
                    print_error_help(e);
                }
                else {
                    if let Err(e) = tm.remove_state_with_name(&res.unwrap()) {
                        print_error_help(RiplError::EncounteredTuringError { error: e });
                    }
                }
            },
            ModifyTuringMode::SaveTM => {
                if let Err(e) = save_tm(rl, &tm, &storage.curr_path) {
                    print_error_help(e);
                }
            },
            ModifyTuringMode::FeedWord => {
                let res = query_string(rl, format!("Enter the word to feed to this Turing machine: "));
                if let Err(e) = res  {
                    print_error_help(e);
                }
                else {
                    let res = TuringMachines::new(tm.clone(), res.unwrap(), ritm_core::turing_machine::Mode::SaveAll);
                    if let Err(e) = res {
                        print_error_help(RiplError::EncounteredTuringError { error: e });
                    }
                    else {
                        storage.iterator = Some(res.unwrap());
                        execute_mode::next_step(rl, &mut storage.iterator.as_mut().unwrap(),  storage.clear_after_step);
                        return Modes::Execute;
                    }
                }
            },
            ModifyTuringMode::UnloadTM => {
                storage.graph = None;
                return Modes::Start;
            },
        }
        Modes::Modify
    }
    
    fn get_help_color(str : colored::ColoredString) -> colored::ColoredString {
        str.purple()
    }
    
    
}


fn get_state_name(rl: &mut Editor<(), FileHistory>) -> Result<String, RiplError>
{
    let name = query_string(rl, format!("Enter the {} of the state: ", "name".blue()));

    name
}



fn add_transition(rl: &mut Editor<(), FileHistory>, turing_graph: &mut TuringMachineGraph) -> Result<(), RiplError>
{
    let transitions = query_transition(rl, format!("Enter one or multiple {} to add to the graph: ", "transitions".blue()));

    if let Err(e) = transitions {
        return Err(e);
    }

    let (q1, vec_tm, q2) = transitions.unwrap();

    for transition in vec_tm {
        if let Err(e) = turing_graph.append_rule_state_by_name(&q1, transition, &q2) {
            return Err(RiplError::EncounteredTuringError { error: e });
        }
    }

    Ok(())
}




pub fn query_transition(rl: &mut Editor<(), FileHistory>, query: String) -> Result<(String, Vec<TuringTransitionMultRibbons>, String), RiplError>
{
    println!("{}", query);
    loop {
        let readline = rl.readline("==> ");
        match readline {
            Ok(l) => {
                let l = l.trim().to_string();
                if l.is_empty() {
                    continue;
                }
                rl.add_history_entry(l.to_string()).unwrap();
                
                let res = parse_transition_string(l);
                if let Err(e) = res {
                    return Err(RiplError::EncounteredParsingError { error: e });
                }
                return Ok(res.unwrap());
            },
            Err(e) => return Err(RiplError::CouldNotParseStringError { value: e.to_string() }),
        }
    }
}


fn save_tm(rl: &mut Editor<(), FileHistory>, tm: &TuringMachineGraph, current_path: &Option<PathBuf>) -> Result<(), RiplError>
{
    let tm_string = turing_parser::graph_to_string(tm);
    if tm_string.is_empty() {
        println!("{}", "Nothing to save because this machine has no transitions !".red());
        return Ok(());
    }
    loop {
        println!("Enter the {} of the {} to create: ", "path".bold().blue(), "file".bold());
        let readline = match current_path {
            Some(p) => {
                rl.readline_with_initial("==> ", (format!("{}", p.as_path().join("turing_machine").to_str().unwrap()).as_str(), ".tm"))
            },
            None => {
                rl.readline("==> ")
            },
        };
        match readline {
            Ok(l) => {
                let l = l.trim().to_string();
                if l.is_empty() {
                    continue;
                }
                rl.add_history_entry(l.to_string()).unwrap();
                
                let path = Path::new(&l);
                // Check that no file with this name exists
                
                if path.exists() {
                    // If it does, ask user for confirmation before overwritting it
                    let choice = query_string(rl, format!("A file with this name already exists, rewrite it ? {}: ", "Y(es) or N(o)".italic().blue()));
                    match choice {
                        Ok(choice) => {
                            let choice = choice.to_lowercase();
                            if !choice.eq("y") && !choice.eq("yes") {
                                continue;
                            }
                        },
                        Err(e) => {
                            return Err(e);
                        },
                    }
                }

                let file = File::create(path);
                match file {
                    Ok(mut f) => {
                        if let Err(e) = f.write_all(tm_string.as_bytes()) {
                            return Err(RiplError::FileError { file_path: Some(e.to_string()) });
                        }
                    },
                    Err(e) => {
                        return Err(RiplError::FileError { file_path: Some(e.to_string()) });
                    },
                }

                println!("{}{}", "Saved the file at the location : ".green(), path.to_str().unwrap());
                return Ok(());
            },
            Err(e) => return Err(RiplError::CouldNotParseStringError { value: e.to_string() }),
        }
    }

}