use core::time;
use std::{fmt::Display};

use colored::{Color, ColoredString, Colorize};
use ritm_core::{turing_machine::{TuringExecutionSteps, TuringMachines}, turing_state::{TuringState, TuringStateType}};
use strum_macros::EnumIter;

use crate::{modes::choice_modes::{ModeEvent, Modes}, query_prim, query_string, query_usize, ripl_error::{print_error_help, RiplError}};



#[derive(EnumIter)]
pub enum ExecuteTuringMode {
    NextStep,
    SkipSteps,
    AutoPlay,
    Finish,
    Reset,
    FeedWord,
    SummaryGraph,
    SummaryExecution,
    Stop,
}


impl Display for ExecuteTuringMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            ExecuteTuringMode::NextStep => "Move to next step",
            ExecuteTuringMode::SkipSteps => "Skip multiple steps",
            ExecuteTuringMode::AutoPlay => "Execute at a given speed the TM",
            ExecuteTuringMode::Finish => "Finish the execution (can loop forever)",
            ExecuteTuringMode::Reset => "Reset the execution",
            ExecuteTuringMode::FeedWord => "Feed a new word and reset",
            ExecuteTuringMode::SummaryGraph => "Print a summary of the graph",
            ExecuteTuringMode::SummaryExecution => "Print a summary of the execution",
            ExecuteTuringMode::Stop => "Stop the execution",
        })
    }
}


impl ModeEvent for ExecuteTuringMode {
    fn print_help(&self) {
        todo!()
    }

    fn choose_option(&self, rl: &mut rustyline::Editor<(), rustyline::history::FileHistory>, storage: &mut crate::DataStorage) -> Modes {
        let mut tm = storage.iterator.as_mut().unwrap();
        let res = match self {
            ExecuteTuringMode::NextStep => {
                next_step(&mut tm);
                None
            },
            ExecuteTuringMode::SkipSteps => {
                // Get nb to skip
                let total = query_prim::<usize>(rl, String::from("Insert the number of steps to skip: "));
                if let Err(e) = total {
                    Some(e)
                }
                else {
                    let total = total.unwrap();
                    if total != 0 {
                        for _ in 0..total-1 {
                            if let None = tm.next() {
                                break;
                            }
                        }
                        next_step(&mut tm);
                    }
                    None
                }
            },
            ExecuteTuringMode::AutoPlay => {
                // ask for the speed
                let speed = query_prim::<f32>(rl, String::from("Insert the time in seconds to wait between steps (floats are accepted): "));
                if let Err(e) = speed {
                    Some(e)
                }
                else {
                    let sleep_time = time::Duration::from_secs_f32(speed.unwrap());
                    storage.is_running.store(true, std::sync::atomic::Ordering::SeqCst);

                    for step in &mut *tm {
                        // Allow the user to stop the execution if it is taking too long (or infinite)
                        if !storage.is_running.load(std::sync::atomic::Ordering::SeqCst) {
                            break;
                        }
                        print_step(&step);
                        std::thread::sleep(sleep_time);
                    }
                    None
                }
                
            },
            ExecuteTuringMode::Finish => {
                let mut last_step = None;
                storage.is_running.store(true, std::sync::atomic::Ordering::SeqCst);
                    
                for steps in &mut *tm {
                    // Allow the user to stop the execution if it is taking too long (or infinite)
                    if !storage.is_running.load(std::sync::atomic::Ordering::SeqCst) {
                        break;
                    }
                    last_step = Some(steps);
                }
                if let Some(step) = last_step {
                    print_step(&step);
                }
                else {
                    println!("Already finished");
                }
                None
            },
            ExecuteTuringMode::Stop => {
                storage.iterator = None;

                return Modes::Modify;
            },
            ExecuteTuringMode::Reset => {
                if let Err(e) = tm.reset() {
                    Some(RiplError::EncounteredTuringError { error: e })
                }
                else {
                    next_step(&mut tm);
                    None
                }
            },
            ExecuteTuringMode::FeedWord => {
                let word = query_string(rl, String::from("Give a new input to replace the current one with: "));
                if let Err(e) = word {
                    Some(e)
                }
                else {
                    if let Err(e) = tm.reset_word(&word.unwrap()) {
                        Some(RiplError::EncounteredTuringError { error: e })
                    }
                    else {
                        next_step(&mut tm);
                        None
                    }
                }
            },
            ExecuteTuringMode::SummaryGraph => {
                println!("{}", tm.get_turing_machine_graph_ref());
                None
            },
            ExecuteTuringMode::SummaryExecution => {
                None
            }
        };
        if let Some(e) = res {
            print_error_help(e);
        }
        Modes::Execute
    }
    
    fn get_help_color(str : colored::ColoredString) -> colored::ColoredString {
        str.yellow()
    }
}
// printf '%s\n' "0" "1" "6" "tmp" | cargo run


pub fn next_step(mut tm: &mut TuringMachines) -> bool
{
    match tm.next() {
        Some(step) => {
            print_step(&step);
            true
        },
        None => {
            println!("{}", "No more steps left".bold().cyan());
            false
        },
    }
}

fn print_step(st: &TuringExecutionSteps)
{
    // Print the iteration number : 

    
    // println!("{}", st);

    match st {
        TuringExecutionSteps::FirstIteration { init_state, init_read_ribbon, init_write_ribbons } => {

            println!("{} {}", "* Iteration: ".bold().magenta(), st.get_nb_iterations().to_string().bold());
            println!("{}", format_ribbons(st, Color::Magenta));
        },
        TuringExecutionSteps::TransitionTaken { previous_state, reached_state, state_pointer, transition_index_taken, transition_taken, read_ribbon, write_ribbons, iteration } => {        
            if let TuringStateType::Accepting = reached_state.state_type {
                println!("{} {}", "* Iteration: ".bold().green(), st.get_nb_iterations().to_string().bold());

                print!("{}", "* Transition taken: ".bold().green());
                println!("{} {} {} {} {}", color_state(previous_state), "{", transition_taken, "}", color_state(reached_state));

                println!("{}", format_ribbons(st, Color::Green));
            }
            else {
                println!("{} {}", "* Iteration: ".bold().blue(), st.get_nb_iterations().to_string().bold());
    
                print!("{}", "* Transition taken: ".bold().blue());
                println!("{} {} {} {} {}", color_state(previous_state), "{", transition_taken, "}", color_state(reached_state));
                println!("{}", format_ribbons(st, Color::Blue));                
            }

            
        },
        TuringExecutionSteps::Backtracked { previous_state, reached_state, state_pointer, read_ribbon, write_ribbons, iteration } => {
            println!("{} {}", "* Iteration: ".bold().yellow(), st.get_nb_iterations().to_string().bold());
            print!("{}", "\t-> Backtracked: ".bold().yellow());
            println!("From {} to {}", color_state(previous_state), color_state(reached_state));
            println!("{}", format_ribbons(st, Color::Yellow));
        },
    }
}

fn color_state(state: &TuringState) -> ColoredString
{
    format!("q_{}", state.name).color(match state.state_type {
        ritm_core::turing_state::TuringStateType::Accepting => {Color::Green},
        ritm_core::turing_state::TuringStateType::Rejecting => {Color::Red}
        _ => {Color::White}
    })
}

fn format_ribbons(st: &TuringExecutionSteps, color: Color) -> ColoredString
{
    let first = format!("{}\n{}\n", "* Reading ribbon: ".bold(), st.get_reading_ribbon().to_string().white());

    let mut second = format!("{}\n", "* Writing ribbons: ".bold());
    for rib in st.get_writting_ribbons() {
        second = format!("{}{}", second, format!("{}\n", rib).white());
    }
    format!("{}{}", first, second).color(color)
}