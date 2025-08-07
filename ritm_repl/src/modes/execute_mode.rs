use std::fmt::Display;

use colored::Colorize;
use strum_macros::EnumIter;

use crate::{modes::choice_modes::{ModeEvent, Modes}, query_string, query_usize, ripl_error::{print_error_help, RiplError}};



#[derive(EnumIter)]
pub enum ExecuteTuringMode {
    NextStep,
    SkipSteps,
    AutoPlay,
    Finish,
    Stop,
    Reset,
    FeedWord,
    SummaryGraph,
    SummaryExecution,
}


impl Display for ExecuteTuringMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            ExecuteTuringMode::NextStep => "Move to next step",
            ExecuteTuringMode::SkipSteps => "Skip multiple steps",
            ExecuteTuringMode::AutoPlay => "Execute at a given speed the TM",
            ExecuteTuringMode::Finish => "Finish the execution (can loop forever)",
            ExecuteTuringMode::Stop => "Stop the execution",
            ExecuteTuringMode::Reset => "Reset the execution",
            ExecuteTuringMode::FeedWord => "Feed a new word and reset",
            ExecuteTuringMode::SummaryGraph => "Print a summary of the graph",
            ExecuteTuringMode::SummaryExecution => "Print a summary of the execution",
        })
    }
}


impl ModeEvent for ExecuteTuringMode {
    fn print_help(&self) {
        todo!()
    }

    fn choose_option(&self, rl: &mut rustyline::Editor<(), rustyline::history::FileHistory>, storage: &mut crate::DataStorage) -> Modes {
        let mut tm = storage.iterator.as_mut().unwrap();
        match self {
            ExecuteTuringMode::NextStep => {
                println!("{}", tm.next().unwrap());
            },
            ExecuteTuringMode::SkipSteps => {
                // Get nb to skip
                let total = query_usize(rl, String::from("Insert the number of steps to skip: "));
                if let Err(e) = total {
                    print_error_help(e)
                }
                else {
                    let total = total.unwrap() - 1;
                    for step in &mut *tm {
                        if step.get_nb_iterations() >= total - 1 {
                            break;
                        }
                    }
                    println!("{}", tm.next().unwrap());
                }
            },
            ExecuteTuringMode::AutoPlay => todo!(),
            ExecuteTuringMode::Finish => todo!(),
            ExecuteTuringMode::Stop => {
                storage.iterator = None;

                return Modes::Modify;
            },
            ExecuteTuringMode::Reset => {
                if let Err(e) = tm.reset() {
                    print_error_help(RiplError::EncounteredTuringError { error: e });
                }
                // TODO : MOVE TO THE FIRST StEP !!
            },
            ExecuteTuringMode::FeedWord => {
                let word = query_string(rl, String::from("Give a new input to replace the current one with: "));
                if let Err(e) = word {
                    print_error_help(e);
                }
                else {
                    if let Err(e) = tm.reset_word(&word.unwrap()) {
                        print_error_help(RiplError::EncounteredTuringError { error: e });
                    }
                }
                // TODO : MOVE TO THE FIRST StEP !!
            },
            ExecuteTuringMode::SummaryGraph => {
                println!("{}", tm.get_turing_machine_graph_ref());
            },
            ExecuteTuringMode::SummaryExecution => {

            }
        }
        Modes::Execute
    }
    
    fn get_help_color(str : colored::ColoredString) -> colored::ColoredString {
        str.yellow()
    }
}
// printf '%s\n' "0" "1" "6" "tmp" | cargo run