use core::time;
use std::fmt::{format, Display};

use colored::{Color, ColoredString, Colorize};
use ritm_core::{turing_machine::{Mode, TuringExecutionSteps, TuringMachines}, turing_ribbon::{TuringReadRibbon, TuringWriteRibbon}, turing_state::{TuringState, TuringStateType}};
use strum_macros::EnumIter;

use crate::{modes::choice_modes::{ModeEvent, Modes}, query_prim, query_string, query_usize, ripl_error::{print_error_help, RiplError}};



#[derive(EnumIter)]
pub enum ExecuteTuringMode {
    NextStep,
    SkipSteps,
    AutoPlay,
    Finish,
    FakeGuessing,
    Reset,
    FeedWord,
    ToggleClearAfterStep,
    SetExecutionMode,
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
            ExecuteTuringMode::ToggleClearAfterStep => "Toggle on/off clearing after each step",
            ExecuteTuringMode::SetExecutionMode => "Sets the execution mode",
            ExecuteTuringMode::SummaryGraph => "Print a summary of the graph",
            ExecuteTuringMode::SummaryExecution => "Print a summary of the execution",
            ExecuteTuringMode::FakeGuessing => "Iterate over the correct path, if any (can loop forever)",
            ExecuteTuringMode::Stop => "Stop the execution",
        })
    }
}


impl ModeEvent for ExecuteTuringMode {
    fn print_help(&self) {
        print!("-> ");
        println!("{}", match self {
            ExecuteTuringMode::NextStep => format!("Advances the execution to the next iteration (if possible) and show the result."),
            ExecuteTuringMode::SkipSteps => format!("Skips a specified number of steps. If the execution finishes before the derised number of steps were passed, then it simply displays it and stops."),
            ExecuteTuringMode::AutoPlay => format!("{}\n{} {}", "Advances the execution by one step periodically until it reaches the end (if any exists).",
                                                    "But this can be interrupted at any time by using","CTRL+C".red().bold() ),
            ExecuteTuringMode::Finish => format!("{}\n{} {}","Tries to completly finish the execution and show the last step found." ,
                                                  "However, since it is possible for turing machines to loop forever, it's possible that this call never ends too. But it can be interrupted at any time by using", 
                                                  "CTRL+C".red().bold()),
            ExecuteTuringMode::Reset => format!("Resets the execution to the first iteration while keeping the first word."),
            ExecuteTuringMode::FeedWord => format!("Feeds a new word to the turing machine resets the execution to the first iteration."),
            ExecuteTuringMode::ToggleClearAfterStep => format!("Chooses wether the terminal should be cleared before showing an iteration. Clearing before an execution can lead to an easier way to follow an execution but it might also delete some important informations between steps."),
            ExecuteTuringMode::SetExecutionMode => format!("Sets the current execution mode of the Turing Machine. Different mode will result in different behaviors. It's recommended to use modes like StopAfter when it isn't clear if the machine can loop forever or not."),
            ExecuteTuringMode::FakeGuessing => format!("{}\n{} {}","Resets and executes completly the machine. And if a correct path is found, then the next iterations will only lead to the outcome where the word is accepted.",
                                                "Due to the nature of this call, if an infinite execution arises, stop the execution by pressing", "CTRL+C".red().bold()),
            ExecuteTuringMode::SummaryGraph => format!("Prints a detailed overview of the current Turing Machine"),
            ExecuteTuringMode::SummaryExecution => format!("Summarizes the current execution by showing some important informations, like the last iteration, the state of the memory."),
            ExecuteTuringMode::Stop => format!("Stops the execution of this machine and goes back to the graph modification mode"),
        }.green())
    }

    fn choose_option(&self, rl: &mut rustyline::Editor<(), rustyline::history::FileHistory>, storage: &mut crate::DataStorage) -> Modes {
        let mut tm = storage.iterator.as_mut().unwrap();
        let res = match self {
            ExecuteTuringMode::NextStep => {
                next_step(rl, &mut tm, storage.clear_after_step);
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
                    let mut before_last_step = None;
                    let mut last_step = None;
                    if total != 0 {
                        for _ in 0..total-1 {
                            before_last_step = last_step;
                            last_step = tm.next();
                            if let None = last_step {
                                break;
                            }
                        }

                        if let None = last_step {
                            if let Some(ending_step) = before_last_step {
                                print_step(rl, &ending_step, storage.clear_after_step);
                            }
                        }
                        next_step(rl, &mut tm, storage.clear_after_step);
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
                    let speed = speed.unwrap();
                    if speed < 0. {
                        Some(RiplError::NegativeValueError { value: speed })
                    }
                    else {
                        let sleep_time = time::Duration::from_secs_f32(speed);
                        storage.is_running.store(true, std::sync::atomic::Ordering::SeqCst);
                        // print first step (to check if the exec is finished or not)
                        let first_step = tm.next();
                        if let Some(step) = first_step {
                            print_step(rl, &step, storage.clear_after_step);    
                        }
                        else {
                            println!("{}", "The execution is already finished".blue());
                        }
                        
                        for step in &mut *tm {
                            // Allow the user to stop the execution if it is taking too long (or infinite)
                            if !storage.is_running.load(std::sync::atomic::Ordering::SeqCst) {
                                break;
                            }
                            print_step(rl, &step, storage.clear_after_step);
                            std::thread::sleep(sleep_time);
                        }
                        None
                    }
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
                    print_step(rl, &step, storage.clear_after_step);
                }
                else {
                    println!("{}", "Already finished".blue());
                }
                None
            },
            ExecuteTuringMode::Stop => {
                storage.iterator = None;

                return Modes::Modify;
            },
            ExecuteTuringMode::Reset => {
                tm.reset();
                next_step(rl, &mut tm, storage.clear_after_step);
                None
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
                        next_step(rl,  &mut tm, storage.clear_after_step);
                        None
                    }
                }
            },
            ExecuteTuringMode::SummaryGraph => {
                println!("{}{}\n", "Mode of execution : ".blue(), tm.get_mode().to_string().yellow());
                println!("{}", tm.get_graph_ref().to_string().blue());
                None
            },
            ExecuteTuringMode::SummaryExecution => {
                summarise_execution(rl, tm, storage.clear_after_step);
                None
            },
            ExecuteTuringMode::ToggleClearAfterStep => {
                storage.clear_after_step = !storage.clear_after_step;
                println!("{} {}{}", "Will".blue(), match storage.clear_after_step {
                    false => "not ".blue().italic(),
                    true => "".into(),
                }, "clear the terminal after every steps".blue());
                None
            },
            ExecuteTuringMode::SetExecutionMode => {
                match query_mode(rl) {
                    Ok(mode) => {
                        tm.set_mode(&mode);
                        storage.exec_mode = mode;
                        None
                    },
                    Err(e) => Some(e),
                }
            },
            ExecuteTuringMode::FakeGuessing => {
                storage.is_running.store(true, std::sync::atomic::Ordering::SeqCst);
                let res = tm.get_path_to_accept(|| {
                    storage.is_running.load(std::sync::atomic::Ordering::SeqCst)
                });
                if let Some(vec) = res {
                    for p in vec {
                        print_step(rl, &p, false);
                    }
                }
                else {
                    println!("{}", format!("No path found to an accepting state").cyan());
                }
                None
            },
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


pub fn next_step(rl: &mut rustyline::Editor<(), rustyline::history::FileHistory>, mut tm: &mut TuringMachines, clear_after: bool) -> bool
{

    match tm.next() {
        Some(step) => {
            print_step(rl, &step, clear_after);
            true
        },
        None => {
            println!("{}", "No more steps left".bold().cyan());
            false
        },
    }
}

fn print_step(rl: &mut rustyline::Editor<(), rustyline::history::FileHistory>, st: &TuringExecutionSteps, clear_after: bool)
{
    if clear_after {
        rl.clear_screen().unwrap();
    }

    match st {
        TuringExecutionSteps::FirstIteration { init_state:_, init_read_ribbon:_, init_write_ribbons:_ } => {

            println!("{} {}", "* Iteration: ".bold().magenta(), st.get_nb_iterations().to_string().bold());
            println!("{}", format_ribbons(st.get_reading_ribbon(), st.get_writting_ribbons(), Color::Magenta));
        },
        TuringExecutionSteps::TransitionTaken { previous_state, reached_state, state_pointer:_, transition_index_taken:_, transition_taken, read_ribbon:_, write_ribbons:_, iteration:_ } => {        
            if let TuringStateType::Accepting = reached_state.state_type {
                println!("{} {}", "* Iteration: ".bold().green(), st.get_nb_iterations().to_string().bold());

                print!("{}", "* Transition taken: ".bold().green());
                println!("{} {} {} {} {}", color_state(previous_state), "{", transition_taken, "}", color_state(reached_state));

                println!("{}", format_ribbons(st.get_reading_ribbon(), st.get_writting_ribbons(), Color::Green));
            }
            else {
                println!("{} {}", "* Iteration: ".bold().blue(), st.get_nb_iterations().to_string().bold());
    
                print!("{}", "* Transition taken: ".bold().blue());
                println!("{} {} {} {} {}", color_state(previous_state), "{", transition_taken, "}", color_state(reached_state));
                println!("{}", format_ribbons(st.get_reading_ribbon(), st.get_writting_ribbons(), Color::Blue));                
            }

            
        },
        TuringExecutionSteps::Backtracked { previous_state, reached_state, state_pointer:_, read_ribbon:_, write_ribbons:_, iteration , backtracked_iteration} => {
            println!("{} {}", "* Iteration: ".bold().yellow(), st.get_nb_iterations().to_string().bold());
            print!("{}", "\t-> Backtracked: ".bold().yellow());
            println!("From iteration {} to {}. From state {} to {}", iteration.to_string().yellow(), backtracked_iteration.to_string().yellow(), color_state(previous_state), color_state(reached_state));
            println!("{}", format_ribbons(st.get_reading_ribbon(), st.get_writting_ribbons(), Color::Yellow));
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

fn format_ribbons(reading_ribbon: &TuringReadRibbon, writing_ribbons: &Vec<TuringWriteRibbon>, color: Color) -> ColoredString
{
    let first = format!("{}\n{}\n", "* Reading ribbon: ".bold(), reading_ribbon.to_string().white());

    let mut second = format!("{}\n", "* Writing ribbons: ".bold());
    for rib in writing_ribbons {
        second = format!("{}{}", second, format!("{}\n", rib).white());
    }
    format!("{}{}", first, second).color(color)
}



fn query_mode(rl: &mut rustyline::Editor<(), rustyline::history::FileHistory>) -> Result<Mode, RiplError>
{
    let save_all = "SaveAll";
    let stop_after = "StopAfter";
    let stop_first_reject = "StopFirstReject";

    let color_val = |val: &str| -> ColoredString { val.blue().bold() };

    loop {
        let ans = query_string(rl, format!("Choose a mode between {}, {} or {}", color_val(save_all), 
                                                                                                                color_val(stop_after), 
                                                                                                                color_val(stop_first_reject)));
        if let Err(e) = ans {
            return Err(e);
        }
        let ans = ans.unwrap().to_lowercase();
        // check that the string is valid
        if ans == save_all.to_lowercase() {
            return Ok(Mode::SaveAll);
        }
        else if ans == stop_after.to_lowercase() {
            let steps = query_prim::<usize>(rl, format!("Give the maximum number of {} :",  color_val("steps")));
            if let Err(e) = steps {
                return Err(e);
            }
            return Ok(Mode::StopAfter(steps.unwrap()));    
        }
        else if ans == stop_first_reject.to_lowercase() {
            return Ok(Mode::StopFirstReject);
        }
        println!("{}", "Unknown mode".red()) 
    }
}


fn summarise_execution(rl: &mut rustyline::Editor<(), rustyline::history::FileHistory>, tm: &TuringMachines, clear_after: bool)
{
    // Show the last iteration
    if let Some(it) = tm.get_last_step().as_ref() {
        println!("{}", "Last iteration :".italic());
        print_step(rl, it, clear_after);
    }


    println!("{}{}", "Is the execution over ? ".italic(), match tm.is_over() {
        true => format!("Yes"),
        false => format!("No"),
    }.cyan());


    println!("{}", format!("Memory stack (Size -> {}) :", tm.get_memory().len()).italic(), );

    if tm.get_memory().is_empty() {
        println!("{}", "\tEmpty".italic());
    }
    for saved_state in tm.get_memory() {
        println!("{}", "-=".repeat(14).bold().underline());
        println!("Saved at iteration : {}", saved_state.iteration);
        println!("Path left to explore : {}", saved_state.next_transitions.len().to_string().cyan());
        println!("At state : {}", tm.get_graph_ref().get_state(saved_state.saved_state_index).unwrap());
        println!("Saved Ribbons :\n{}", format_ribbons(&saved_state.saved_read_ribbon, &saved_state.saved_write_ribbons, Color::Cyan));
    }
}